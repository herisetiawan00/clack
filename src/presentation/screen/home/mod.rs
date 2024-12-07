use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{
    cache::Cache,
    common::enums::request::Request,
    context::Context,
    entities::configuration::Configuration,
    presentation::widgets::{channels, status_line},
};

use super::Screen;

pub fn get() -> Screen<'static> {
    Screen {
        commands,
        keymaps,
        build,
    }
}

fn commands(_config: &Configuration, command: &String, context: &mut Context) -> Option<Request> {
    match command.as_str() {
        "back" => {
            context.route_pop();
        }
        _ => {}
    }

    None
}

fn keymaps(
    _config: &Configuration,
    event: &event::Event,
    _context: &mut Context,
) -> Option<String> {
    if let event::Event::Key(KeyEvent {
        modifiers, code, ..
    }) = event
    {
        match (modifiers, code) {
            (&KeyModifiers::SHIFT, KeyCode::Char('Q')) => {
                return Some(String::from("back"));
            }
            _ => {}
        }
    }

    None
}

fn build<'screen>(
    _config: &Configuration,
    frame: &mut Frame,
    _context: &Context,
    _cache: &mut Cache<'screen>,
) {
    frame.render_widget(Block::default().title("Home"), frame.area());
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    let status_rect = rects[1];

    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Max(40), Constraint::Min(1)])
        .split(rects[0]);

    let channel_rect = rects[0];
    let message_rect = rects[1];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(block.clone().title("Messages"), message_rect);

    channels::render(frame, channel_rect, &_config, &_context, _cache);
    status_line::render(frame, status_rect, &_config, &_context);
}
