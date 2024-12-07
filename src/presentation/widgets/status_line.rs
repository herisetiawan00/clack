use std::collections::HashMap;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    Frame,
};

use crate::{context::Context, entities::configuration::Configuration};

pub fn render(frame: &mut Frame, rect: Rect, _config: &Configuration, _context: &Context) {
    let authorization = _context.auth.clone().unwrap();
    let global = _context.state.global.clone();
    let user = global
        .get_user(authorization.authed_user.id)
        .expect("User is logged in");
    let channel_name = _context
        .state
        .channel
        .selected
        .clone()
        .map_or(String::new(), |channel| channel.name.unwrap_or(channel.id));
    let mode = _context.mode.clone();
    let mode_name = mode.to_string();

    let mut placeholders: HashMap<&str, &str> = HashMap::new();
    placeholders.insert("user", user.profile.display_name.as_str());
    placeholders.insert("team", authorization.team.name.as_str());
    placeholders.insert("mode", mode_name.as_str());
    placeholders.insert("channel", channel_name.as_str());

    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rect);
    let left_rect = rects[0];
    let right_rect = rects[1];

    let mut left_text = _config.status_line.left.template.clone();
    let mut right_text = _config.status_line.right.template.clone();

    for (key, value) in placeholders {
        left_text = left_text.replace(&format!("%{}%", key), value);
        right_text = right_text.replace(&format!("%{}%", key), value);
    }

    let splited_left = left_text
        .split("<>")
        .map(|text| text.to_owned())
        .collect::<Vec<String>>();
    let splited_right = right_text
        .split("<>")
        .map(|text| text.to_owned())
        .collect::<Vec<String>>();

    let primary_left = splited_left.get(0).unwrap_or(&String::new()).clone();
    let secondary_left = splited_left.get(1).unwrap_or(&String::new()).clone();
    let teriary_left = splited_left.get(2).unwrap_or(&String::new()).clone();

    let left_spans: Vec<Span> = vec![
        Span::styled(
            primary_left,
            Style::default()
                .fg(Color::Black)
                .bg(_context.mode.to_color()),
        ),
        Span::styled(
            _config.status_line.left.separator.clone(),
            Style::default()
                .fg(_context.mode.to_color())
                .bg(Color::DarkGray),
        ),
        Span::styled(
            secondary_left,
            Style::default().fg(Color::White).bg(Color::DarkGray),
        ),
        Span::styled(
            _config.status_line.left.separator.clone(),
            Style::default().fg(Color::DarkGray).bg(Color::default()),
        ),
        Span::styled(
            teriary_left,
            Style::default().fg(Color::White).bg(Color::default()),
        ),
    ];

    let primary_right = splited_right.get(0).unwrap_or(&String::new()).clone();
    let secondary_right = splited_right.get(1).unwrap_or(&String::new()).clone();
    let teriary_right = splited_right.get(2).unwrap_or(&String::new()).clone();

    let right_spans: Vec<Span> = vec![
        Span::styled(
            teriary_right,
            Style::default().fg(Color::White).bg(Color::default()),
        ),
        Span::styled(
            _config.status_line.right.separator.clone(),
            Style::default().fg(Color::DarkGray).bg(Color::default()),
        ),
        Span::styled(
            secondary_right,
            Style::default().fg(Color::White).bg(Color::DarkGray),
        ),
        Span::styled(
            _config.status_line.right.separator.clone(),
            Style::default()
                .fg(_context.mode.to_color())
                .bg(Color::DarkGray),
        ),
        Span::styled(
            primary_right,
            Style::default()
                .fg(Color::Black)
                .bg(_context.mode.to_color()),
        ),
    ];

    let left_line = Line::from(left_spans).alignment(Alignment::Left);
    let right_line = Line::from(right_spans).alignment(Alignment::Right);

    frame.render_widget(left_line, left_rect);
    frame.render_widget(right_line, right_rect);
}
