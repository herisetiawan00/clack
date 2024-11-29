use std::{
    cmp::{max, min},
    collections::HashMap,
};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
};

use crate::{
    cache::Cache,
    entities::{configuration::Configuration, slack::conversations::Channel},
    enums::{section::Section, user_mode::UserMode, widgets::Widgets},
    states::State,
};

use super::{
    common,
    section_data::{SectionData, WidgetData},
};

pub fn new() -> SectionData<'static> {
    SectionData {
        section: Section::Channel,
        need_render,
        render,
    }
}

fn need_render(old_state: &State, state: &State) -> bool {
    old_state.channel != state.channel
        || old_state.global.mode != state.global.mode
        || old_state.global.section != state.global.section
}

fn render(
    chunk: Rect,
    config: &Configuration,
    state: &State,
    cache: &mut Cache<'static>,
) -> Vec<WidgetData<'static>> {
    let mut result: Vec<WidgetData> = Vec::new();

    let channel_state = state.channel.clone();
    let channels = channel_state.channels.clone();

    let show_search =
        state.global.section == Section::Channel && state.global.mode == UserMode::Search;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(if show_search {
            vec![Constraint::Percentage(100), Constraint::Length(3)]
        } else {
            vec![Constraint::Percentage(100)]
        })
        .split(chunk);

    let height = min(chunks[0].height as i32 - 2, channels.len() as i32);
    let index = channel_state.selected_index.unwrap_or(0) as i32;

    let visible_channel: Vec<&Channel> = channels
        .iter()
        .filter(|channel| {
            channel
                .name
                .clone()
                .unwrap_or(channel.id.clone())
                .contains(&state.channel.search)
                && (channel.is_member.unwrap_or(true) || channel.is_user_deleted.unwrap_or(false))
        })
        .skip(max(index - height + 1, 0) as usize)
        .take(height as usize)
        .collect();

    let mut list_item: Vec<ListItem> = Vec::new();

    for channel in visible_channel {
        let style = match channel_state.selected.clone() {
            Some(selected) => {
                if channel == &selected {
                    Style::default().bg(Color::Cyan).fg(Color::Black)
                } else {
                    Style::default()
                }
            }
            None => Style::default(),
        };

        let cache_id = format!("channel.{}", channel.id.clone());

        let item = if let Some(Widgets::Line(widget)) = cache.widget.get(&cache_id) {
            widget.to_owned()
        } else {
            let mut line = Line::default();

            let prefix = if channel.is_im {
                "\u{eabc}"
            } else if channel.is_mpim.unwrap_or(false) {
                "\u{f0c0}"
            } else if channel.is_private.unwrap_or(false) {
                "\u{e672}"
            } else {
                "\u{f4df}"
            };

            let title = if channel.is_im {
                state
                    .global
                    .get_user(channel.user.clone().unwrap().clone())
                    .map_or(channel.user.clone().unwrap(), |user| {
                        user.profile.display_name
                    })
            } else {
                channel.name.clone().unwrap_or(channel.id.clone())
            };

            line.push_span(Span::from(format!(" {} {}", prefix, title)));

            cache.widget.insert(cache_id, Widgets::Line(line.clone()));

            line
        };

        list_item.push(ListItem::new(item).style(style));
    }

    match common::block::build(state.global.section == Section::Channel, &state.global.mode) {
        Widgets::Block(block) => {
            let widget = List::new(list_item).block(block.clone());
            result.push(WidgetData {
                rect: chunks[0],
                widget: Widgets::List(widget),
            });

            if show_search {
                result.push(WidgetData {
                    rect: chunks[1],
                    widget: Widgets::Paragraph(
                        Paragraph::new(format!("/{}", channel_state.search)).block(block),
                    ),
                });
            }
        }
        _ => {}
    }

    result
}
