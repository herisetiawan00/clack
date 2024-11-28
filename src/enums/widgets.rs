use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};

#[derive(Clone)]
pub enum Widgets<'render> {
    Block(Block<'render>),
    List(List<'render>),
    Line(Line<'render>),
    Paragraph(Paragraph<'render>),
}

impl Widgets<'_> {
    pub fn render(&self, frame: &mut Frame, rect: Rect) {
        match self {
            Widgets::Block(widget) => frame.render_widget(widget, rect),
            Widgets::List(widget) => frame.render_widget(widget, rect),
            Widgets::Line(widget) => frame.render_widget(widget, rect),
            Widgets::Paragraph(widget) => frame.render_widget(widget, rect),
        }
    }
}
