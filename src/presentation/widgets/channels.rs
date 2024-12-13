use std::cmp::{max, min};

use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};
use regex::Regex;

use crate::{
    cache::Cache,
    common::{constants::icon, enums::request::Request},
    context::Context,
    entities::{configuration::Configuration, slack::conversations::Channel},
    enums::{user_mode::UserMode, widgets::Widgets},
    utils,
};

use super::Widget;

pub fn get<'widget>() -> Widget<'widget> {
    Widget {
        commands,
        keymaps,
        build,
    }
}

fn commands(_config: &Configuration, command: &String, context: &mut Context) -> Option<Request> {
    let move_up_re = Regex::new(r"^move up (\d+)$").unwrap();
    let move_down_re = Regex::new(r"^move down (\d+)$").unwrap();

    let filtered_channels: Vec<&Channel> = context
        .state
        .channel
        .channels
        .iter()
        .filter(|channel| {
            channel
                .name
                .clone()
                .unwrap_or(channel.id.clone())
                .contains(&context.state.channel.search)
        })
        .collect();
    let last_index = max(filtered_channels.len() as i32 - 1, 0) as usize;

    if let Some(channel) = context.state.channel.selected.clone() {
        if !filtered_channels.contains(&&channel) {
            context.state.channel.selected = None;
            context.state.channel.selected_index = None;
        }
    }

    match command.as_str() {
        command if command.starts_with("move up ") => {
            if let Some(captures) = move_up_re.captures(command) {
                let len: usize = captures
                    .get(1)
                    .map_or(0, |len| len.as_str().parse().unwrap());

                let next_index = match context.state.channel.selected.clone() {
                    Some(selected) => {
                        let index = filtered_channels
                            .iter()
                            .position(|channel| channel == &&selected);

                        match index {
                            Some(index) => {
                                if index == 0 {
                                    last_index
                                } else {
                                    max(0, index as i32 - len as i32) as usize
                                }
                            }
                            None => last_index,
                        }
                    }
                    None => last_index,
                };

                context.state.channel.selected = Some(filtered_channels[next_index].clone());
                context.state.channel.selected_index = Some(next_index);
            }
        }
        command if command.starts_with("move down ") => {
            if let Some(captures) = move_down_re.captures(command) {
                let len: usize = captures
                    .get(1)
                    .map_or(0, |len| len.as_str().parse().unwrap());

                let next_index = match context.state.channel.selected.clone() {
                    Some(selected) => {
                        let index = filtered_channels
                            .iter()
                            .position(|channel| channel == &&selected);

                        match index {
                            Some(index) => {
                                if index == last_index {
                                    0
                                } else {
                                    min(last_index, index + len)
                                }
                            }
                            None => 0,
                        }
                    }
                    None => 0,
                };

                context.state.channel.selected = Some(filtered_channels[next_index].clone());
                context.state.channel.selected_index = Some(next_index);
            }
        }
        command if command.starts_with("open ") => {
            let channel_id = command.replace("open ", "");

            let channel = context
                .state
                .channel
                .channels
                .iter()
                .find(|channel| channel.id == channel_id);

            if let Some(channel) = channel {
                context.state.channel.opened = Some(channel.clone());
                return Some(Request::GetConversationHistory(channel_id));
            }
        }
        _ => {}
    }
    None
}

pub fn keymaps(
    _config: &Configuration,
    event: &event::Event,
    _context: &mut Context,
) -> Option<String> {
    let up = utils::keycode::from_string(_config.keymaps.up.clone());
    let down = utils::keycode::from_string(_config.keymaps.down.clone());
    let open = utils::keycode::from_string(_config.keymaps.open.clone());

    if let event::Event::Key(KeyEvent {
        modifiers, code, ..
    }) = event.clone()
    {
        match (modifiers, code) {
            (KeyModifiers::SHIFT, KeyCode::Char('Q')) => {
                return Some(String::from("back"));
            }
            key if key == up => {
                return Some(String::from("move up 1"));
            }
            key if key == down => {
                return Some(String::from("move down 1"));
            }
            key if key == open => {
                if let Some(channel) = &_context.state.channel.selected {
                    return Some(format!("open {}", channel.id));
                }
            }
            _ => {}
        }
    }

    None
}

pub fn build(
    _config: &Configuration,
    frame: &mut Frame,
    _context: &Context,
    _cache: &mut Cache,
    rect: Rect,
) {
    let channel_state = _context.state.channel.clone();
    let channels = channel_state.channels;

    let is_focus = _context.is_focus(&String::from("channels"));

    let show_search = is_focus && _context.mode == UserMode::Search;

    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints(if show_search {
            vec![Constraint::Min(3), Constraint::Length(3)]
        } else {
            vec![Constraint::Min(3)]
        })
        .split(rect);

    let channels_rect = rects[0];

    let height = min(channels_rect.height as i32 - 2, channels.len() as i32);
    let index = channel_state.selected_index.unwrap_or_default() as i32;

    let visible_channels: Vec<Channel> = channels
        .iter()
        .skip(max(index - height + 1, 0) as usize)
        .take(height as usize)
        .map(|channel| channel.clone())
        .collect();

    let mut list_item: Vec<ListItem> = Vec::new();

    for channel in visible_channels {
        let mut style = Style::default();

        if channel_state
            .selected
            .clone()
            .map_or(false, |selected| channel == selected)
        {
            style = style.bg(Color::Cyan).fg(Color::Black);
        }

        let cache_id = format!("channels.{}", channel.id.clone());

        let item = if let Some(Widgets::Line(widget)) = _cache.widget.get(&cache_id) {
            widget.clone()
        } else {
            let mut line = Line::default();

            let prefix = if channel.is_im {
                icon::USER
            } else if channel.is_mpim.unwrap_or(false) {
                icon::GROUP
            } else if channel.is_private.unwrap_or(false) {
                icon::LOCK
            } else {
                icon::HASHTAG
            };

            let title = if channel.is_im {
                _context
                    .state
                    .global
                    .get_user(channel.user.clone().unwrap())
                    .map_or(channel.user.clone().unwrap(), |user| {
                        user.profile.display_name
                    })
            } else {
                channel.name.clone().unwrap_or(channel.id.clone())
            };

            line.push_span(Span::from(format!(" {} {}", prefix, title)));

            _cache.widget.insert(cache_id, Widgets::Line(line.clone()));

            line
        };

        list_item.push(ListItem::new(item).style(style));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(format!("Channels {}", list_item.clone().len(),));

    frame.render_widget(List::new(list_item).block(block), rect);
}
