use std::cmp::{max, min};

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
};

use crate::{
    cache::Cache,
    entities::configuration::Configuration,
    enums::{section::Section, widgets::Widgets},
    states::State,
    utils::string::split_text_with_custom_first,
};

use super::{
    common,
    section_data::{SectionData, WidgetData},
};

pub fn new() -> SectionData<'static> {
    SectionData {
        section: Section::Input,
        need_render,
        render,
    }
}

fn need_render(old_state: &State, state: &State) -> bool {
    old_state.input.value != state.input.value
        || old_state.channel.opened != state.channel.opened
        || old_state.global.section != state.global.section
        || old_state.global.mode != state.global.mode
}

fn render(
    chunk: Rect,
    config: &Configuration,
    state: &State,
    cache: &mut Cache<'static>,
) -> Vec<WidgetData<'static>> {
    let mut result: Vec<WidgetData> = Vec::new();

    let text = if state.input.value.is_empty() {
        let opened = state.channel.opened.clone();
        match opened {
            Some(channel) => {
                let prefix = if channel.is_im {
                    "\u{eabc}"
                } else if channel.is_mpim.unwrap_or(false) {
                    "\u{f0c0}"
                } else if channel.is_private.unwrap_or(false) {
                    "\u{e672}"
                } else {
                    "\u{f4df}"
                };

                format!("Message {} {}", prefix, channel.name.unwrap_or(channel.id))
            }
            None => String::from("Select channel and start mesaging :D"),
        }
    } else {
        state.input.value.clone()
    };

    let style = if state.input.value.is_empty() {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default()
    };

    let length = chunk.width as usize - 2;

    let splited_text: Vec<String> = text.split("\n").map(|text| text.to_string()).collect();

    let mut wrapped_text: Vec<String> = Vec::new();

    splited_text.clone().into_iter().for_each(|text| {
        wrapped_text.extend(split_text_with_custom_first(&text, length, length));
    });

    let mut list_item: Vec<ListItem> = Vec::new();

    for text in wrapped_text {
        let line = Line::default().spans([Span::from(text)]);
        let item = ListItem::new(line).style(style);
        list_item.push(item);
    }

    let skip = max(list_item.len() as i32 - (chunk.height as i32 - 2), 0) as usize;

    list_item = list_item
        .clone()
        .iter()
        .skip(skip)
        .map(|item| item.to_owned())
        .collect();

    if let Widgets::Block(block) =
        common::block::build(state.global.section == Section::Input, &state.global.mode)
    {
        result.push(WidgetData {
            rect: chunk,
            widget: Widgets::List(List::new(list_item.clone()).style(style).block(block)),
        });
    }

    result
}
