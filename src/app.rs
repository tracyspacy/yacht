use crate::activities_manager::{ActivityDetails, AllActivities, Day, FrequencyType};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Inactive,
    ActiveName,
    ActiveFrequency,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub input: String,
    pub cursor_position: usize,
    pub input_mode: InputMode,
    pub selected: usize,
    pub todays_activities: Vec<String>,
    pub activities_status: Vec<String>,
    pub all_activities: AllActivities,
    pub new_activity_name: String,
    pub new_activity_frequency: FrequencyType,
    pub total_perfect_days: i64,
    pub logs: String,
    pub counter: i64,
    pub day_status: bool,
    pub activities_till_perfect_day: u8,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            input: String::new(),
            input_mode: InputMode::Inactive,
            cursor_position: 0,
            selected: 0,
            todays_activities: vec![String::from("...")],
            activities_status: vec![String::from("...")],
            all_activities: AllActivities::new(),
            new_activity_name: String::new(),
            new_activity_frequency: FrequencyType::Invalid,
            total_perfect_days: 0,
            logs: String::new(),
            counter: 0,
            day_status: false,
            activities_till_perfect_day: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut app = Self::default();
        app.all_activities =
            AllActivities::load_from_file().unwrap_or_else(|_| AllActivities::new());
        let today = Day::today();
        (app.todays_activities, app.activities_status) = today.today_activities_list();
        app.is_perfect_day_today();
        app.activities_till_perfect_day();
        app.total_perfect_days(-30);
        app
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        // Do something on tick if needed
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input
            .insert(self.cursor_position, new_char.to_ascii_uppercase());
        self.move_cursor_right();
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn refresh_global_status(&mut self) {
        self.is_perfect_day_today();
        self.activities_till_perfect_day();
        self.total_perfect_days(-30);
    }

    pub fn is_perfect_day_today(&mut self) {
        let today = Day::today();
        match today.is_perfect_day() {
            true => self.day_status = true,
            false => self.day_status = false,
        };
    }

    pub fn activities_till_perfect_day(&mut self) {
        match self.day_status {
            true => self.activities_till_perfect_day = 0,
            false => {
                self.activities_till_perfect_day = self
                    .activities_status
                    .iter()
                    .filter(|status| **status == String::from("[-]"))
                    .count() as u8
            }
        };
    }

    pub fn total_perfect_days(&mut self, period: i64) {
        self.total_perfect_days = (period..=0)
            .filter(|&i| Day::get_day(i).is_perfect_day())
            .count() as i64;
    }

    pub fn add_new_activity_name(&mut self) {
        self.new_activity_name = self.input.clone();
    }

    pub fn convert_activity_frequency(&mut self) {
        self.new_activity_frequency = match self.input.as_str() {
            "AW" => FrequencyType::AllWeek,
            "WD" => FrequencyType::WorkingDays,
            "WE" => FrequencyType::WeekEnds,
            _ => {
                self.logs = String::from("Invalid frequency. Please enter AW, WD, or WE.");
                FrequencyType::Invalid
            }
        };
    }

    pub fn add_new_activity(&mut self) {
        let activity_details = ActivityDetails::new_activity(self.new_activity_frequency.clone());
        match self
            .all_activities
            .add_activity(self.new_activity_name.clone(), activity_details)
        {
            Ok(_) => {
                self.clear_new_activity_data();
                self.refresh_global_status();
                self.logs = format!("Activity is added!");
            }
            Err(e) => {
                self.logs = format!("Error: {}", e);
            }
        }
    }

    fn clear_new_activity_data(&mut self) {
        self.new_activity_name.clear();
        self.new_activity_frequency = FrequencyType::Invalid;
        let today = Day::today();
        (self.todays_activities, self.activities_status) = today.today_activities_list();
    }

    pub fn set_activity_done(&mut self, activity_index: usize) {
        if let Some(activity) = self.todays_activities.get(activity_index) {
            match self
                .all_activities
                .set_activity_done(String::from(activity.clone()))
            {
                Ok(_) => {
                    self.logs = format!("Activity is set done!");
                    // Update today's activities list
                    let today = Day::today();
                    (self.todays_activities, self.activities_status) =
                        today.today_activities_list();
                    self.refresh_global_status();
                }
                Err(e) => {
                    self.logs = format!("Error: {}", e);
                }
            }
        }
    }

    pub fn remove_activity(&mut self, activity_index: usize) {
        if let Some(activity) = self.todays_activities.get(activity_index) {
            match self
                .all_activities
                .remove_activity(String::from(activity.clone()))
            {
                Ok(_) => {
                    self.logs = format!("Activity is removed!");
                    // Update today's activities list
                    let today = Day::today();
                    (self.todays_activities, self.activities_status) =
                        today.today_activities_list();
                    self.refresh_global_status();
                }

                Err(e) => {
                    self.logs = format!("Error: {}", e);
                }
            }
        }
    }
    /*
    maybe for later
    pub fn purge_all_activities(&mut self) {
        self.all_activities.purge_all();
        self.day = Day::today(); // Update day activities after purging all activities
    }
    */
}
