use crate::{
    cache::Cache,
    entities::configuration::Configuration,
    enums::{section::Section, widgets::Widgets},
    states::State,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use std::collections::HashMap;

use super::section_data::{SectionData, WidgetData};

pub fn new() -> SectionData<'static> {
    SectionData {
        section: Section::StatusLine,
        need_render,
        render,
    }
}

fn need_render(old_state: &State, state: &State) -> bool {
    old_state.global.mode != state.global.mode || old_state.channel.opened != state.channel.opened
}

fn render(
    chunk: Rect,
    config: &Configuration,
    state: &State,
    cache: &mut Cache<'static>,
) -> Vec<WidgetData<'static>> {
    let mut result: Vec<WidgetData> = Vec::new();

    let authorization = state.authorization.clone().unwrap();
    let global = state.global.clone();

    let user = global
        .get_user(authorization.authed_user.id.clone())
        .unwrap();

    let channel_name = state
        .channel
        .selected
        .clone()
        .map_or(String::new(), |channel| channel.name.unwrap_or(channel.id));

    let mode = global.mode.clone();
    let mode_name = mode.to_string();

    let mut placeholders: HashMap<&str, &str> = HashMap::new();
    placeholders.insert("user", user.profile.display_name.as_str());
    placeholders.insert("team", authorization.team.name.as_str());
    placeholders.insert("mode", mode_name.as_str());
    placeholders.insert("channel", channel_name.as_str());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunk);

    let mut left_text = config.status_line.left.template.clone();
    let mut right_text = config.status_line.right.template.clone();

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
            Style::default().fg(Color::Black).bg(mode.to_color()),
        ),
        Span::styled(
            config.status_line.left.separator.clone(),
            Style::default().fg(mode.to_color()).bg(Color::DarkGray),
        ),
        Span::styled(
            secondary_left,
            Style::default().fg(Color::White).bg(Color::DarkGray),
        ),
        Span::styled(
            config.status_line.left.separator.clone(),
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
            config.status_line.right.separator.clone(),
            Style::default().fg(Color::DarkGray).bg(Color::default()),
        ),
        Span::styled(
            secondary_right,
            Style::default().fg(Color::White).bg(Color::DarkGray),
        ),
        Span::styled(
            config.status_line.right.separator.clone(),
            Style::default().fg(mode.to_color()).bg(Color::DarkGray),
        ),
        Span::styled(
            primary_right,
            Style::default().fg(Color::Black).bg(mode.to_color()),
        ),
    ];

    result.push(WidgetData {
        rect: chunks[0],
        widget: Widgets::Line(Line::from(left_spans).alignment(Alignment::Left)),
    });

    result.push(WidgetData {
        rect: chunks[1],
        widget: Widgets::Line(Line::from(right_spans).alignment(Alignment::Right)),
    });

    result
}
