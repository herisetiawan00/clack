use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem},
};

use crate::{
    cache::Cache,
    entities::configuration::Configuration,
    enums::{section::Section, widgets::Widgets},
    states::State,
    utils::string::{date_format, split_text_with_custom_first},
};

use super::{
    common,
    section_data::{SectionData, WidgetData},
};

pub fn new() -> SectionData<'static> {
    SectionData {
        section: Section::Thread,
        need_render,
        render,
    }
}

fn need_render(old_state: &State, state: &State) -> bool {
    true
}

fn render(
    chunk: Rect,
    config: &Configuration,
    state: &State,
    cache: &mut Cache<'static>,
) -> Vec<WidgetData<'static>> {
    let mut result: Vec<WidgetData<'static>> = Vec::new();
    let opened = state.message.opened.clone().unwrap();
    let message_ts = opened.ts;

    let mut list_item: Vec<ListItem> = Vec::new();

    for message in state.thread.messages.clone() {
        let style = if state
            .thread
            .selected
            .clone()
            .map_or(false, |selected| selected == message.clone())
        {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };

        let cache_id = format!("messages.{}.{}", message_ts.clone(), message.ts.clone());

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
            let user = state.global.get_user(user_id.clone());
            let text = message.text.clone().unwrap_or_default();

            let user_name = user
                .clone()
                .map_or(user_id.clone(), |user| user.profile.display_name);
            let user_color = user.clone().map_or("ffffff".to_string(), |user| {
                user.color.unwrap_or("ffffff".to_string())
            });

            let time = date_format(message.ts.clone(), "%A, %B %d at %H:%M");

            let r = u8::from_str_radix(&user_color[0..2], 16).unwrap();
            let g = u8::from_str_radix(&user_color[2..4], 16).unwrap();
            let b = u8::from_str_radix(&user_color[4..6], 16).unwrap();

            let name_line = Line::default().spans([
                Span::styled(
                    format!("{} ", user_name),
                    Style::default()
                        .fg(Color::Rgb(r, g, b))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    time.unwrap_or_default(),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            cache_data.push(Widgets::Line(name_line.clone()));
            list_item.push(ListItem::new(name_line).style(style));

            let splited_message = split_text_with_custom_first(
                &text.as_str(),
                chunk.width as usize - 2,
                chunk.width as usize - 2,
            );

            for part in splited_message {
                let line = Line::default().spans([Span::from(part.clone())]);
                cache_data.push(Widgets::Line(line.clone()));
                list_item.push(ListItem::new(line).style(style));
            }

            let mut message_footer = Vec::new();

            if let Some(reactions) = message.reactions {
                for reaction in reactions {
                    let text = format!("[:{}: {}]", reaction.name, reaction.count);
                    message_footer.push(text);
                }
            }

            if !message_footer.is_empty() {
                let joined_footer = message_footer.join(" ");
                let length = chunk.width as usize - 2;

                let splited_footer = split_text_with_custom_first(&joined_footer, length, length);

                let style = Style::default().fg(Color::Gray);

                for part in splited_footer {
                    let line = Line::default()
                        .spans([Span::from(part.clone())])
                        .style(style);
                    cache_data.push(Widgets::Line(line.clone()));
                    list_item.push(ListItem::new(line).style(style));
                }
            }

            if message.ts == message_ts {
                let text = format!("{} replies", opened.reply_count.unwrap_or_default());
                let dash = "-".repeat(chunk.width as usize - 3 - text.len());

                let style = Style::default().fg(Color::DarkGray);

                let line =
                    Line::default().spans([Span::styled(format!("{} {}", text, dash), style)]);
                cache_data.push(Widgets::Line(line.clone()));
                list_item.push(ListItem::new(line));
            }

            cache.widgets.insert(cache_id, cache_data);
        }
    }

    if let Widgets::Block(block) =
        common::block::build(state.global.section == Section::Thread, &state.global.mode)
    {
        result.push(WidgetData {
            rect: chunk,
            widget: Widgets::List(List::new(list_item).block(block.title("Thread"))),
        })
    }

    result
}
