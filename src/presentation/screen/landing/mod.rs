use crossterm::event::{self, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::context::Context;

use super::Screen;

pub fn get() -> Screen {
    Screen {
        commands,
        keymaps,
        build,
    }
}

fn commands(command: &String, context: &mut Context) {
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
        _ => {}
    }
}

fn keymaps(event: &event::Event, context: &mut Context) -> Option<String> {
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

fn build(frame: &mut Frame, context: &Context) {
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
