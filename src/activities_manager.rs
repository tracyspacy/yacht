use crate::init::ACTIVITIES_FILE;
use crate::time_utils;
use bincode;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, ErrorKind};

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
pub enum FrequencyType {
    AllWeek,
    WorkingDays,
    WeekEnds,
    Invalid,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ActivityDetails {
    start: i64,
    frequency: FrequencyType,
    completion_timestamps: HashSet<i64>,
}

impl ActivityDetails {
    pub fn new_activity(frequency: FrequencyType) -> ActivityDetails {
        //to test
        //let figures: HashSet<i64> = vec![1706117517 , 1705771917 ].into_iter().collect();

        ActivityDetails {
            start: time_utils::current_time_timestamp(),
            frequency,
            completion_timestamps: HashSet::new(),
        }
    }
}

#[derive(Debug)]
pub struct Day {
    activities: HashMap<String, bool>,
}

impl Day {
    pub fn today() -> Day {
        Self::get_day(0)
    }

    pub fn day_types_to_show(adjustment: i64) -> [FrequencyType; 2] {
        let weekday = time_utils::todays_weekday(adjustment).number_from_monday();
        match weekday {
            x if x >= 6 => [FrequencyType::AllWeek, FrequencyType::WeekEnds],
            _ => [FrequencyType::AllWeek, FrequencyType::WorkingDays],
        }
    }

    pub fn get_day(adjustment: i64) -> Day {
        let activities_data: AllActivities =
            AllActivities::load_from_file().expect("Failed to load activities");

        let mut today_activities: HashMap<String, bool> = HashMap::new();
        let td = Self::day_types_to_show(adjustment);

        activities_data
            .activities
            .iter()
            .filter(|(_, activity)| td.contains(&activity.frequency))
            .for_each(|(name, _)| {
                today_activities.insert(
                    name.clone(),
                    activities_data.is_activity_completed_on_day(String::from(name), adjustment),
                );
            });

        Day {
            activities: today_activities,
        }
    }

    pub fn today_activities_list(&self) -> (Vec<String>, Vec<String>) {
        let mut activities_status_list = Vec::new();
        self.activities.iter().for_each(|(key, value)| {
            let short_value: &str;
            match value {
                true => short_value = "[+]",
                false => short_value = "[-]",
            };
            activities_status_list.push((String::from(key), String::from(short_value)));
        });

        // Sort the vector based on the activities
        activities_status_list.sort_by(|a, b| a.0.cmp(&b.0));

        // Separate the sorted activities and their statuses into two lists
        let activities_list: Vec<String> = activities_status_list
            .iter()
            .map(|(activity, _)| activity.clone())
            .collect();
        let status_list: Vec<String> = activities_status_list
            .iter()
            .map(|(_, status)| status.clone())
            .collect();

        (activities_list, status_list)
    }

    pub fn is_perfect_day(&self) -> bool {
        match self.activities.is_empty() {
            true => false,
            false => self.activities.iter().all(|(_, &value)| value),
        }
    }

    /*
    maybe for later
    fn is_in_todays_activities(&self, name: String) -> bool {
        self.activities.contains_key(&name)
    }
    */
}

#[derive(Debug, Serialize, Deserialize)]

pub struct AllActivities {
    pub activities: HashMap<String, ActivityDetails>,
}

impl AllActivities {
    pub fn new() -> Self {
        AllActivities {
            activities: HashMap::new(),
        }
    }
    pub fn save_to_file(&self) -> io::Result<()> {
        let file = File::create(ACTIVITIES_FILE)?;
        bincode::serialize_into(file, self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn load_from_file() -> Result<Self, io::Error> {
        match File::open(ACTIVITIES_FILE) {
            Ok(file) => {
                bincode::deserialize_from(file).map_err(|e| io::Error::new(ErrorKind::Other, e))
            }
            Err(e) => Err(e),
        }
    }

    pub fn add_activity(
        &mut self,
        activity_name: String,
        activity: ActivityDetails,
    ) -> Result<(), &'static str> {
        match self.is_in_activities(activity_name.clone()) {
            false => match activity.frequency {
                FrequencyType::Invalid => Err("Wrong Frequency Type"),
                FrequencyType::AllWeek | FrequencyType::WorkingDays | FrequencyType::WeekEnds => {
                    self.activities
                        .insert(activity_name.to_uppercase(), activity);
                    self.save_to_file().map_err(|_| "Failed to save activities")
                }
            },
            true => Err("Activity Already Exists"),
        }
    }

    pub fn remove_activity(&mut self, activity_name: String) -> Result<(), &'static str> {
        self.activities
            .retain(|activity, _| activity != &activity_name);
        self.save_to_file().map_err(|_| "Failed to save activities")
    }

    pub fn set_activity_done(&mut self, activity_name: String) -> Result<(), &'static str> {
        match self.is_activity_completed_on_day(activity_name.clone(), 0) {
            false => {
                self.activities
                    .entry(activity_name)
                    .and_modify(|activity_details| {
                        activity_details
                            .completion_timestamps
                            .insert(time_utils::current_time_timestamp());
                    });
                self.save_to_file().map_err(|_| "Failed to save activities")
            }
            _ => Err("Already Done!"),
        }
    }

    pub fn is_in_activities(&self, name: String) -> bool {
        self.activities.contains_key(&name)
    }

    /*
    maybe for later
    fn purge_all(&mut self) {
        self.activities = HashMap::new();
        self.save_to_file().expect("Failed to save activities")
    }
    */

    fn is_activity_completed_on_day(&self, activity_name: String, adjustment: i64) -> bool {
        let activity = self.activities.get(&activity_name);
        match activity {
            Some(activity) => activity
                .completion_timestamps
                .iter()
                .any(|timestamp| time_utils::is_timestamp_on_day(*timestamp, adjustment)),
            None => {
                println!("Activity not found.");
                false
            }
        }
    }
}
