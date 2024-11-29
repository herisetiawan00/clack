use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
};

use crate::{
    cache::Cache,
    entities::configuration::Configuration,
    enums::{section::Section, widgets::Widgets},
    states::State,
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

    if let Widgets::Block(block) =
        common::block::build(state.global.section == Section::Input, &state.global.mode)
    {
        result.push(WidgetData {
            rect: chunk,
            widget: Widgets::Paragraph(Paragraph::new(text).style(style).block(block)),
        });
    }

    result
}
