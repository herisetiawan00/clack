pub mod home;
pub mod landing;

use crossterm::event;
use ratatui::Frame;

use crate::{
    cache::Cache, common::enums::request::Request, context::Context,
    entities::configuration::Configuration,
};

#[derive(Clone)]
pub struct Screen<'screen> {
    pub commands:
        fn(config: &Configuration, command: &String, context: &mut Context) -> Option<Request>,
    pub keymaps:
        fn(config: &Configuration, event: &event::Event, context: &mut Context) -> Option<String>,
    pub build: fn(
        config: &Configuration,
        frame: &mut Frame,
        context: &Context,
        cache: &mut Cache<'screen>,
    ),
}
