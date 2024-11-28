//use ratatui::layout::Rect;
//
//use crate::{
//    entities::configuration::Configuration,
//    enums::{section::Section, widgets::Widgets},
//    states::State,
//};
//
//use super::{common, section_data::SectionData};
//
//pub fn new() -> SectionData<'static> {
//    SectionData {
//        section: Section::Thread,
//        need_render,
//        render,
//    }
//}
//
//fn need_render(old_state: &State, state: &State) -> bool {
//    true
//}
//
//fn render(chunk: Rect, config: &Configuration, state: &State) -> Vec<(Rect, Widgets<'static>)> {
//    let mut result: Vec<(Rect, Widgets<'static>)> = Vec::new();
//
//    if let Widgets::Block(block) = common::block::build(
//        state.navigator.section == Section::Thread,
//        &state.global.mode,
//    ) {
//        result.push((
//            chunk,
//            Widgets::Block(block),
//            //Widgets::Paragraph(Paragraph::new(text).style(style).block(block)),
//        ))
//    }
//
//    result
//}
