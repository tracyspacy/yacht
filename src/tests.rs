#[cfg(test)]
mod tests {
    use crate::activities_manager::FrequencyType;
    use crate::app::{App, InputMode};

    #[test]
    fn test_default_app() {
        let app = App::default();
        // Verify default values
        assert_eq!(app.running, true);
        assert_eq!(app.input, String::new());
        assert_eq!(app.input_mode, InputMode::Inactive);
    }

    #[test]
    fn test_new_app() {
        // Test construction of a new App instance
        App::new();
    }

    #[test]
    fn test_enter_char_inserts_new_char_at_cursor_position() {
        let mut app = App::default();
        app.input = String::from("Hello");
        app.cursor_position = 3;

        app.enter_char('X');

        // Ensure that 'X' is inserted at cursor position
        assert_eq!(app.input, "HelXlo");
        // Ensure that cursor position is moved to the right
        assert_eq!(app.cursor_position, 4);
    }

    #[test]
    fn test_add_new_activity() {
        let mut app = App::default();

        let activity_name = String::from("THINK");
        let frequency = FrequencyType::AllWeek;

        app.input = activity_name.clone();
        app.input_mode = InputMode::ActiveName;
        app.add_new_activity_name();
        app.new_activity_frequency = frequency;

        app.add_new_activity();
        //dbg!(app.new_activity_frequency);
        //dbg!(app.new_activity_name);

        // Ensure that activity is added to all_activities
        assert!(app.all_activities.is_in_activities(activity_name));
        // Ensure that new_activity_name is cleared after adding activity
        assert_eq!(app.new_activity_name, "");
        // Ensure that new_activity_frequency is set correctly after adding activity
        assert_eq!(app.new_activity_frequency, FrequencyType::Invalid);
        // Ensure that logs indicate successful activity addition
        assert_eq!(app.logs, "Activity is added!");
        // Ensure that global status is refreshed after adding activity
        assert_eq!(app.day_status, false); // As it's not a perfect day yet
        assert_eq!(app.total_perfect_days, 0); // As it's not a perfect day yet
    }

    #[test]
    fn test_remove_activity() {
        let mut app = App::default();
        test_add_new_activity();
        let activity_name = String::from("Exercise");
        // Call remove_activity method
        app.remove_activity(0); // Assuming only one activity for simplicity

        // Ensure that activity is removed from all_activities
        assert!(!app.all_activities.is_in_activities(activity_name));
        // Ensure that logs indicate successful activity removal
        assert_eq!(app.logs, "Activity is removed!");
    }
}
