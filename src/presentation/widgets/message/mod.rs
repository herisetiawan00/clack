use std::{
    cmp::{max, min},
    collections::HashMap,
};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Padding},
};
use regex::Regex;

use crate::{
    cache::Cache,
    entities::{configuration::Configuration, slack::messages::Message},
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

    let empty_line = Line::default();

    let mut message_index: HashMap<usize, usize> = HashMap::new();
    let mut index = 0;

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
            let full_dash_length = chunks[1].width as usize - date_length - 1;
            let dash_length = full_dash_length / 2;
            let right_length = if full_dash_length & 1 == 1 {
                dash_length + 1
            } else {
                dash_length
            };
            let right_dash = "-".repeat(right_length + (chunks[0].width as usize - 1));
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
            let text = message.text.clone().unwrap_or_default();

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

                cache_data_time.push(Widgets::Line(empty_line.clone()));
                list_time.push(ListItem::new(empty_line.clone()).style(style))
            }

            let mut message_footer = Vec::new();

            if let Some(count) = message.reply_count {
                if count > 0 {
                    let text = format!("[+{} replies]", count);
                    message_footer.push(text);
                }
            }

            if let Some(reactions) = message.reactions {
                for reaction in reactions {
                    let text = format!("[:{}: {}]", reaction.name, reaction.count);
                    message_footer.push(text);
                }
            }

            if !message_footer.is_empty() {
                let joined_footer = message_footer.join(" ");
                let length = chunks[1].width as usize - 1;

                let splited_footer = split_text_with_custom_first(&joined_footer, length, length);

                let style = Style::default().fg(Color::Gray);

                for part in splited_footer {
                    let line = Line::default()
                        .spans([Span::from(part.clone())])
                        .style(style);
                    cache_data.push(Widgets::Line(line.clone()));
                    let item = ListItem::new(line).style(style);
                    list_item.push(item);

                    cache_data_time.push(Widgets::Line(empty_line.clone()));
                    list_time.push(ListItem::new(empty_line.clone()).style(style))
                }
            }

            cache.widgets.insert(cache_id, cache_data);
            cache.widgets.insert(cache_id_time, cache_data_time);
        }

        message_index.insert(index, list_item.len());
        index += 1;
    }

    let height = min(chunks[1].height as i32 - 2, list_item.len() as i32);

    let index = if message_index.is_empty() {
        0
    } else {
        let selected = state.message.selected_index.clone();
        if let Some(index) = selected {
            message_index.get(&index).unwrap().clone()
        } else {
            message_index
                .iter()
                .max_by_key(|&(_, v)| v)
                .unwrap()
                .1
                .clone()
        }
    };
    let length = list_item.len() as i32;
    let skip = max(0, (length - height) - (length - index as i32)) as usize;
    list_item = list_item
        .clone()
        .iter()
        .skip(skip)
        .take(height as usize)
        .map(|item| item.to_owned())
        .collect();

    list_time = list_time
        .clone()
        .iter()
        .skip(skip)
        .take(height as usize)
        .map(|item| item.to_owned())
        .collect();

    match common::block::build(state.global.section == Section::Message, &state.global.mode) {
        Widgets::Block(block) => {
            result.push(WidgetData {
                rect: chunk,
                widget: Widgets::Block(block.title(format!("{} - {}", length, index))),
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
