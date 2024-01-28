use crate::activities_manager::{ActivityDetails, AllActivities, FrequencyType};

pub const ACTIVITIES_FILE: &str = "activities.bin";

pub fn initialize_activities() {
    // Check if the activities file exists
    if !std::path::Path::new(ACTIVITIES_FILE).exists() {
        // If the file doesn't exist, initialize and save the activities
        initialize_and_save_activities();
    }

    fn initialize_and_save_activities() {
        // Create initial set of activities
        let mut initial_activities = AllActivities::new();

        match initial_activities.add_activity(
            String::from("Go for a 2 km run"),
            ActivityDetails::new_activity(FrequencyType::AllWeek),
        ) {
            Ok(_) => println!("Done"),
            Err(e) => println!("{:?}", e),
        };

        match initial_activities.add_activity(
            String::from("Study Marxism"),
            ActivityDetails::new_activity(FrequencyType::AllWeek),
        ) {
            Ok(_) => println!("Done"),
            Err(e) => println!("{:?}", e),
        };

        match initial_activities.add_activity(
            String::from("Complete 1 task on Exercism"),
            ActivityDetails::new_activity(FrequencyType::AllWeek),
        ) {
            Ok(_) => println!("Done"),
            Err(e) => println!("{:?}", e),
        };

        match initial_activities.add_activity(
            String::from("Play Tennis"),
            ActivityDetails::new_activity(FrequencyType::AllWeek),
        ) {
            Ok(_) => println!("Done"),
            Err(e) => println!("{:?}", e),
        };

        match initial_activities.add_activity(
            String::from("Practice a foreign language"),
            ActivityDetails::new_activity(FrequencyType::AllWeek),
        ) {
            Ok(_) => println!("Done"),
            Err(e) => println!("{:?}", e),
        };

        initial_activities
            .save_to_file()
            .expect("Failed to save initial activities");
    }
}
