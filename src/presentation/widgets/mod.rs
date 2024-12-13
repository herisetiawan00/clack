use crossterm::event;
use ratatui::{layout::Rect, Frame};

use crate::{
    cache::Cache, common::enums::request::Request, context::Context,
    entities::configuration::Configuration,
};

pub mod channels;
pub mod loading;
pub mod messages;
pub mod status_line;

#[derive(Clone)]
pub struct Widget<'widget> {
    pub commands:
        fn(config: &Configuration, command: &String, context: &mut Context) -> Option<Request>,
    pub keymaps:
        fn(config: &Configuration, event: &event::Event, context: &mut Context) -> Option<String>,
    pub build: fn(
        config: &Configuration,
        frame: &mut Frame,
        context: &Context,
        cache: &mut Cache<'widget>,
        area: Rect,
    ),
}
