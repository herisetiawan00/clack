use crossterm::event::{self, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    cache::Cache, common::enums::request::Request, context::Context,
    entities::configuration::Configuration,
};

use super::Screen;

pub fn get() -> Screen<'static> {
    Screen {
        commands,
        keymaps,
        build,
    }
}

fn commands(config: &Configuration, command: &String, context: &mut Context) -> Option<Request> {
    let mut request: Option<Request> = None;
    match command.as_str() {
        "exit" => {
            context.route_pop();
        }
        "loading show" => {
            context.show_loading();
        }
        "loading hide" => {
            context.hide_loading();
        }
        "login" => {
            request = Some(Request::Authorization(String::from("login_success")));
        }
        "login_success" => {
            context.route_push(String::from("/home"));
        }
        _ => {}
    }

    request
}

fn keymaps(config: &Configuration, event: &event::Event, context: &mut Context) -> Option<String> {
    let mut command: Option<String> = None;

    if let event::Event::Key(KeyEvent { code, .. }) = event {
        match code {
            event::KeyCode::Char(c) => {
                context.command.push(c.clone());
            }
            event::KeyCode::Backspace => {
                context.command.pop();
            }
            event::KeyCode::Enter => {
                command = Some(context.command.clone());
                context.clear_command();
            }
            _ => {}
        }
    }

    command
}

fn build<'screen>(
    config: &Configuration,
    frame: &mut Frame,
    context: &Context,
    cache: &mut Cache<'screen>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(block, frame.area());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Percentage(40),
            Constraint::Min(0),
        ])
        .split(chunks[1]);

    let command = Paragraph::new(format!("> {}", context.command)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("command:")
            .title_alignment(Alignment::Center),
    );

    frame.render_widget(command, chunks[1]);
}
