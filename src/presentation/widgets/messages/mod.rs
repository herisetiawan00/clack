use std::cmp::{max, min};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Padding},
};
use regex::Regex;

use crate::{
    cache::Cache,
    entities::configuration::Configuration,
    enums::{section::Section, widgets::Widgets},
    states::State,
    utils::string::{date_format, split_text_with_custom_first, split_with_space},
};

use super::{
    common,
    section_data::{SectionData, WidgetData},
};

pub fn new() -> SectionData<'static> {
    SectionData {
        section: Section::Message,
        need_render,
        render,
    }
}

fn need_render(old_state: &State, state: &State) -> bool {
    old_state.channel.opened != state.channel.opened
        || old_state.global.mode != state.global.mode
        || old_state.global.section != state.global.section
        || old_state.message != state.message
}

fn render(
    chunk: Rect,
    config: &Configuration,
    state: &State,
    cache: &mut Cache<'static>,
) -> Vec<WidgetData<'static>> {
    let mut result: Vec<WidgetData<'static>> = Vec::new();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(7), Constraint::Min(20)])
        .split(chunk);

    let channel = state.channel.opened.clone();

    let title = channel.map_or(String::new(), |channel| channel.name.unwrap_or(channel.id));

    let mut list_time: Vec<ListItem> = Vec::new();
    let mut list_item: Vec<ListItem> = Vec::new();

    let mut prev_time = String::new();
    let mut prev_date = String::new();

    for message in state.message.messages.clone() {
        let style = if state
            .message
            .selected
            .clone()
            .map_or(false, |selected| selected == message.clone())
        {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };

        let date = date_format(message.ts.clone(), "%A, %B %d").unwrap_or(String::new());

        if prev_date != date {
            prev_date = date.clone();

            let style = Style::default().fg(Color::DarkGray);

            let date_length = date.len() + 2;
            let dash_length = (chunks[1].width as usize - date_length - 2) / 2;
            let right_dash = "-".repeat(dash_length + chunks[0].width as usize + 1);
            let left_dash = "-".repeat(dash_length - (chunks[0].width as usize - 1));

            let time_line = Line::default().spans([Span::styled(
                "-".repeat(chunks[0].width as usize - 1),
                style,
            )]);
            list_time.push(ListItem::new(time_line));

            let date_line = Line::default().spans([Span::styled(
                format!("{} {} {}", left_dash, date, right_dash),
                style,
            )]);
            list_item.push(ListItem::new(date_line));
        }

        let cache_id = format!("messages.{}.{}", title, message.ts);
        let cache_id_time = format!("{}.time", cache_id);

        if let Some(widgets) = cache.widgets.get(&cache_id) {
            if let Some(times) = cache.widgets.get(&cache_id_time) {
                for time in times {
                    if let Widgets::Line(line) = time {
                        let item = ListItem::new(line.clone()).style(style);
                        list_time.push(item)
                    }
                }
            }
            for widget in widgets {
                if let Widgets::Line(line) = widget {
                    let item = ListItem::new(line.clone()).style(style);
                    list_item.push(item);
                }
            }
        } else {
            let mut cache_data: Vec<Widgets> = Vec::new();
            let mut cache_data_time: Vec<Widgets> = Vec::new();

            let user_id = message
                .user
                .clone()
                .unwrap_or(message.bot_id.clone().unwrap_or_default());
            let user = state.global.get_user(user_id.clone());
            let text = message.text.clone();

            let pattern = r"<@(\w+)>";
            let re = Regex::new(pattern).unwrap();

            let result = re.replace_all(&text, |caps: &regex::Captures| {
                let user_id = &caps[1];
                state
                    .global
                    .get_user(user_id.to_string())
                    .map_or(String::new(), |user| {
                        format!("@{}", user.profile.display_name)
                    })
            });

            let user_name = user
                .clone()
                .map_or(user_id.clone(), |user| user.profile.display_name);
            let user_color = user.clone().map_or("ffffff".to_string(), |user| {
                user.color.unwrap_or("ffffff".to_string())
            });

            let r = u8::from_str_radix(&user_color[0..2], 16).unwrap();
            let g = u8::from_str_radix(&user_color[2..4], 16).unwrap();
            let b = u8::from_str_radix(&user_color[4..6], 16).unwrap();

            let splited_message = split_text_with_custom_first(
                &result,
                chunks[1].width as usize - user_name.len() - 2,
                chunks[1].width as usize - 1,
            );

            let mut iterated_message = splited_message.iter();

            let first_text = iterated_message.next();

            let first_line = Line::default().spans([
                Span::styled(
                    user_name.clone(),
                    Style::default()
                        .fg(Color::Rgb(r, g, b))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" {}", first_text.unwrap_or(&String::new())),
                    Style::default(),
                ),
            ]);

            cache_data.push(Widgets::Line(first_line.clone()));
            let first_item = ListItem::new(first_line).style(style);
            list_item.push(first_item);

            let time = date_format(message.ts, "%H:%M").map_or(String::new(), |time| {
                if prev_time == time {
                    String::new()
                } else {
                    prev_time = time.clone();
                    time
                }
            });

            let time_line =
                Line::default().spans([Span::styled(time, Style::default().fg(Color::DarkGray))]);
            cache_data_time.push(Widgets::Line(time_line.clone()));
            list_time.push(ListItem::new(time_line).style(style));

            for part in iterated_message {
                let line = Line::default().spans([Span::from(part.clone())]);
                cache_data.push(Widgets::Line(line.clone()));
                let item = ListItem::new(line).style(style);
                list_item.push(item);

                let time_line = Line::default();
                cache_data_time.push(Widgets::Line(time_line.clone()));
                list_time.push(ListItem::new(time_line).style(style))
            }

            //if message.

            cache.widgets.insert(cache_id, cache_data);
            cache.widgets.insert(cache_id_time, cache_data_time);
        }
    }

    let height = min(chunks[1].height as i32 - 2, list_item.len() as i32);
    let index = state.message.selected_index.unwrap_or(height as usize) as i32;
    let length = list_item.len() as i32;
    let skip = max(length - max(length - index - height, 0) - height, 0) as usize;
    list_item = list_item
        .clone()
        .iter()
        .skip(skip)
        .map(|item| item.to_owned())
        .collect();

    list_time = list_time
        .clone()
        .iter()
        .skip(skip)
        .map(|item| item.to_owned())
        .collect();

    match common::block::build(state.global.section == Section::Message, &state.global.mode) {
        Widgets::Block(block) => {
            result.push(WidgetData {
                rect: chunk,
                widget: Widgets::Block(block),
            });
        }
        _ => {}
    };

    let block = Block::default();

    result.push(WidgetData {
        rect: chunks[0],
        widget: Widgets::List(List::new(list_time).block(block.clone().padding(Padding {
            left: 1,
            top: 1,
            right: 0,
            bottom: 1,
        }))),
    });
    result.push(WidgetData {
        rect: chunks[1],
        widget: Widgets::List(List::new(list_item).block(block.padding(Padding {
            left: 0,
            top: 1,
            right: 0,
            bottom: 1,
        }))),
    });

    result
}
