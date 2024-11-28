use std::collections::HashMap;

use ratatui::text::Line;

use crate::enums::widgets::Widgets;

#[derive(Clone)]
pub struct Cache<'cache> {
    pub widget: HashMap<String, Widgets<'cache>>,
    pub widgets: HashMap<String, Vec<Widgets<'cache>>>,
}

impl Cache<'_> {
    pub fn new() -> Cache<'static> {
        Cache {
            widget: HashMap::new(),
            widgets: HashMap::new(),
        }
    }
}
