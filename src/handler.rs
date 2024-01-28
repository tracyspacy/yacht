use crate::app::{App, AppResult, InputMode};
use crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `q`
        KeyCode::Char('q') | KeyCode::Char('Q') if app.input_mode == InputMode::Inactive => {
            app.quit();
        }

        KeyCode::Esc if app.input_mode != InputMode::Inactive => {
            app.input_mode = InputMode::Inactive;
            app.input.clear();
            app.cursor_position = 0;
        }

        KeyCode::Char('d') | KeyCode::Char('D') if app.input_mode == InputMode::Inactive => {
            app.set_activity_done(app.selected);
        }
        KeyCode::Char('r') | KeyCode::Char('R') if app.input_mode == InputMode::Inactive => {
            app.remove_activity(app.selected);
        }

        KeyCode::Up if app.todays_activities.is_empty() == false => {
            if app.selected > 0 {
                app.selected -= 1;
            }
        }
        KeyCode::Down if app.todays_activities.is_empty() == false => {
            if app.selected < app.todays_activities.len() - 1 {
                app.selected += 1;
            }
        }

        KeyCode::Char('n') | KeyCode::Char('N') if app.input_mode == InputMode::Inactive => {
            app.input_mode = InputMode::ActiveName;
        }

        KeyCode::Char(c) if app.input_mode == InputMode::ActiveName => {
            app.enter_char(c);
        }

        KeyCode::Char(c) if app.input_mode == InputMode::ActiveFrequency => match c {
            'a' | 'w' | 'd' | 'e' => app.enter_char(c),
            _ => {}
        },

        KeyCode::Backspace if app.input_mode != InputMode::Inactive => {
            app.delete_char();
        }

        KeyCode::Enter => match app.input_mode {
            InputMode::ActiveName => {
                app.add_new_activity_name();
                app.input_mode = InputMode::ActiveFrequency;
                app.input.clear();
                app.cursor_position = 0
            }
            InputMode::ActiveFrequency => {
                app.convert_activity_frequency();
                app.add_new_activity();
                app.input.clear();
                app.cursor_position = 0;
                app.input_mode = InputMode::Inactive;
            }
            _ => {}
        },

        _ => {}
    }
    Ok(())
}
