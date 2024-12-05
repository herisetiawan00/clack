pub mod landing;

use crossterm::event;
use ratatui::Frame;

use crate::context::Context;

#[derive(Clone)]
pub struct Screen {
    pub commands: fn(command: &String, context: &mut Context),
    pub keymaps: fn(event: &event::Event, context: &mut Context) -> Option<String>,
    pub build: fn(frame: &mut Frame, context: &Context),
}
