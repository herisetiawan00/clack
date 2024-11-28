use std::collections::HashMap;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::{
    cache::Cache,
    entities::configuration::Configuration,
    enums::{section::Section, widgets::Widgets},
    states::State,
};

use super::section_data::{SectionData, WidgetData};

pub fn new() -> SectionData<'static> {
    SectionData {
        section: Section::StatusLine,
        need_render,
        render,
    }
}

fn build_line(
    mut primary: String,
    mut secondary: String,
    placeholders: &HashMap<&str, &str>,
    alignment: Alignment,
    config: &Configuration,
    state: &State,
) -> Line<'static> {
    for (key, value) in placeholders {
        let key_pattern = format!("%{}%", key);
        primary = primary.clone().replace(&key_pattern, value);
        secondary = secondary.clone().replace(&key_pattern, value);
    }

    let color = state.global.mode.to_color();

    let mut spans = vec![Span::styled(
        format!(
            "{}{}{}",
            primary,
            config.appearance.separator.clone(),
            secondary
        ),
        Style::default()
            .fg(Color::Black)
            .bg(color)
            .add_modifier(Modifier::BOLD),
    )];

    if alignment == Alignment::Left {
        spans.push(Span::styled(
            config.appearance.right_separator.clone(),
            Style::default().fg(color),
        ));
    } else if alignment == Alignment::Right {
        spans.insert(
            0,
            Span::styled(
                config.appearance.left_separator.clone(),
                Style::default().fg(color),
            ),
        )
    }

    Line::default().spans(spans).alignment(alignment)
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

    let left_line = build_line(
        config.status_line.left.primary.clone(),
        config.status_line.left.secondary.clone(),
        &placeholders,
        Alignment::Left,
        config,
        state,
    );

    let right_line = build_line(
        config.status_line.right.primary.clone(),
        config.status_line.right.secondary.clone(),
        &placeholders,
        Alignment::Right,
        config,
        state,
    );

    result.push(WidgetData {
        rect: chunks[0],
        widget: Widgets::Line(left_line),
    });

    result.push(WidgetData {
        rect: chunks[1],
        widget: Widgets::Line(right_line),
    });

    result
}
