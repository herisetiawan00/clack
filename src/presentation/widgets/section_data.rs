use std::collections::HashMap;

use ratatui::layout::Rect;

use crate::{
    cache::Cache,
    entities::configuration::Configuration,
    enums::{section::Section, widgets::Widgets},
    states::State,
};

pub struct SectionData<'data> {
    pub section: Section,
    pub render: fn(Rect, &Configuration, &State, &mut Cache<'data>) -> Vec<WidgetData<'data>>,
    pub need_render: fn(&State, &State) -> bool,
}

#[derive(Clone)]
pub struct WidgetData<'data> {
    pub rect: Rect,
    pub widget: Widgets<'data>,
}
