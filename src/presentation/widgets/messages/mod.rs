use std::cmp::{max, min};

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem},
};
use regex::Regex;

use crate::{
    cache::Cache,
    entities::configuration::Configuration,
    enums::{section::Section, widgets::Widgets},
    states::State,
    utils::string::split_with_space,
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

    let cloned_state = state.clone();

    let channel = state.channel.opened.clone();

    let title = channel.map_or(String::new(), |channel| channel.name.unwrap_or(channel.id));

    let mut list_item: Vec<ListItem> = Vec::new();

    for message in cloned_state.message.messages {
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

        let cache_id = format!("messages.{}.{}", title, message.ts);

        if let Some(widgets) = cache.widgets.get(&cache_id) {
            for widget in widgets {
                if let Widgets::Line(line) = widget {
                    let item = ListItem::new(line.clone()).style(style);
                    list_item.push(item);
                }
            }
        } else {
            let mut cache_data: Vec<Widgets> = Vec::new();

            let user_id = message
                .user
                .clone()
                .unwrap_or(message.bot_id.clone().unwrap_or_default());
            let user = cloned_state.global.get_user(user_id.clone());
            let text = message.text.clone();

            //let pattern = r"<@(\w+)>";
            //let re = Regex::new(pattern).unwrap();
            //
            //let result = re.replace_all(&text, |caps: &regex::Captures| {
            //    let user_id = &caps[1];
            //    cloned_state
            //        .global
            //        .get_user(user_id.to_string())
            //        .map_or(String::new(), |user| {
            //            format!("@{}", user.profile.display_name)
            //        })
            //});

            let user_name = user
                .clone()
                .map_or(user_id.clone(), |user| user.profile.display_name);
            let user_color = user.clone().map_or("ffffff".to_string(), |user| {
                user.color.unwrap_or("ffffff".to_string())
            });

            let r = u8::from_str_radix(&user_color[0..2], 16).unwrap();
            let g = u8::from_str_radix(&user_color[2..4], 16).unwrap();
            let b = u8::from_str_radix(&user_color[4..6], 16).unwrap();

            //let splited_message = split_with_space(
            //    text,
            //    //result.clone().to_string(),
            //    chunk.width as usize - 2,
            //    Some(user_name.len()),
            //);

            let splited_message = vec![text];

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

            for part in iterated_message {
                let line = Line::default().spans([Span::from(format!("{}", part))]);

                cache_data.push(Widgets::Line(line.clone()));

                let item = ListItem::new(line).style(style);

                list_item.push(item);
            }

            cache.widgets.insert(cache_id, cache_data);
        }
    }

    let height = min(chunk.height as i32 - 2, state.message.messages.len() as i32);
    let index = state.message.selected_index.unwrap_or(height as usize) as i32;
    let length = state.message.messages.len() as i32;
    let skip = max(length - max(length - index - height, 0) - height, 0) as usize;
    //let skip = max(
    //    list_item.len() as i32 - state.message.selected_index.unwrap_or(chunk.height.into()) as i32
    //        + 2,
    //    0,
    //) as usize;
    list_item = list_item
        .clone()
        .iter()
        .skip(skip)
        .map(|item| item.to_owned())
        .collect();

    match common::block::build(state.global.section == Section::Message, &state.global.mode) {
        Widgets::Block(block) => {
            result.push(WidgetData {
                rect: chunk,
                widget: Widgets::List(List::new(list_item).block(block.title(title))),
            });
        }
        _ => {}
    };

    result
}
