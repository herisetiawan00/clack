use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};

use crate::enums::{user_mode::UserMode, widgets::Widgets};

pub fn build(highlight: bool, mode: &UserMode) -> Widgets<'static> {
    let style = if highlight {
        Style::default().fg(mode.to_color())
    } else {
        Style::default()
    };

    Widgets::Block(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(style),
    )
}
