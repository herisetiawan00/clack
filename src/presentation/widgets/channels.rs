use std::cmp::{max, min};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

use crate::{
    cache::Cache,
    common::constants::icon,
    context::Context,
    entities::{configuration::Configuration, slack::conversations::Channel},
    enums::{user_mode::UserMode, widgets::Widgets},
};

//pub fn keymaps() {
//
//}

pub fn render(
    frame: &mut Frame,
    rect: Rect,
    _config: &Configuration,
    _context: &Context,
    _cache: &mut Cache,
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
