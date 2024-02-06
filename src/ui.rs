use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListState, Padding, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, InputMode};
/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resoursces:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let p = Paragraph::new(format!(
        "COMMANDS:\n\
        Press `q` to stop running the program.\n\
        Press `n` to add a new activity. `ESC` to quit new activity input mode.\n\
        Select ACTIVITY + `d` to set an activity as DONE or `r` to REMOVE an activity \n\
            ",
    ))
    .block(
        Block::default()
            .title("Y.A.c.H.T.")
            .title_style(Style::default().add_modifier(Modifier::BOLD))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::White).bg(Color::Black))
    .alignment(Alignment::Left);

    let activities_list = List::new(app.todays_activities.clone())
        .block(
            Block::default()
                .title("TODAY'S ACTIVITIES")
                .title_style(Style::default().add_modifier(Modifier::BOLD))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Black).bg(Color::White))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(false);

    let activities_status = List::new(app.activities_status.clone())
        .block(
            Block::default()
                .title_alignment(Alignment::Center)
                .borders(Borders::NONE)
                .padding(Padding::vertical(1)),
        )
        .style(Style::default().fg(Color::Black).bg(Color::White));

    let today_status_text = match app.day_status {
        true => String::from("A PERFECT DAY! (づ ◕‿◕ )づ"),
        false => String::from("JUST A DAY └(・。・)┘"),
    };

    let status_title = "STATUS".to_string() + " >>" + &app.status_day.to_uppercase() + "<< ";

    let global_status = Paragraph::new(format!(
        "Today is {} \n\
        \n\
        {} activities remained till perfect day \n\
        \n\
        {} perfect days during last 30 days \n\
        ",
        today_status_text, app.activities_till_perfect_day, app.total_perfect_days,
    ))
    .block(
        Block::default()
            .title(status_title)
            .title_style(Style::default().add_modifier(Modifier::BOLD))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Black).bg(Color::White))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true });

    let input_text: String;
    match app.input_mode {
        InputMode::ActiveName => input_text = format!("activity name: {}", app.input.as_str()),
        InputMode::ActiveFrequency => {
            input_text = format!(
                "AW for all week, WD for working days and WE for weekends: {}",
                app.input.as_str()
            )
        }
        InputMode::Inactive => input_text = format!(""),
    };

    let input = Paragraph::new(input_text)
        .style(match app.input_mode {
            InputMode::Inactive => Style::default().fg(Color::Black).bg(Color::White),
            _ => Style::default().fg(Color::White).bg(Color::Black),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("NEW ACTIVITY INPUT FIELD")
                .title_style(Style::default().add_modifier(Modifier::BOLD)),
        );

    let logs = Paragraph::new(app.logs.clone())
        .block(
            Block::default()
                .title("RECENT LOG")
                .title_style(Style::default().add_modifier(Modifier::BOLD))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Black).bg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    let size = frame.size();
    let split = size.height / 3;

    let top_half = Rect {
        x: size.x,
        y: size.y,
        width: size.width,
        height: split,
    };
    let list_field = Rect {
        x: size.x,
        y: size.y + split,
        width: size.width / 2,
        height: split,
    };

    let status_part = Rect {
        x: size.x + (list_field.width as f32 * 0.8) as u16,
        y: size.y + split,
        width: 5,
        height: split,
    };

    let global_status_field = Rect {
        x: size.x + list_field.width,
        y: size.y + split,
        width: size.width / 2,
        height: split,
    };

    let input_field = Rect {
        x: size.x,
        y: size.y + 2 * split,
        width: size.width,
        height: split / 2,
    };

    let log_field = Rect {
        x: size.x,
        y: input_field.y + split / 2,
        width: size.width,
        height: split / 2,
    };

    //frame.set_cursor(input_field.x +app.cursor_position as u16 + 21, input_field.y+1);

    let mut list_state: ListState = ListState::default();
    list_state.select(Some(app.selected));

    //static
    frame.render_widget(p, top_half);
    //list
    frame.render_stateful_widget(activities_list, list_field, &mut list_state);
    frame.render_widget(activities_status, status_part);
    //input
    frame.render_widget(input, input_field);
    //logs
    frame.render_widget(logs, log_field);
    //dynamic, but based on other actions
    frame.render_widget(global_status, global_status_field);
}
