use ratatui::style::Color;

#[derive(Clone, PartialEq)]
pub enum UserMode {
    Normal,
    Interact,
    Search,
}

impl UserMode {
    pub fn to_string(&self) -> String {
        match self {
            UserMode::Normal => "NORMAL".to_string(),
            UserMode::Interact => "INTERACT".to_string(),
            UserMode::Search => "SEARCH".to_string(),
        }
    }
    pub fn to_color(&self) -> Color {
        match self {
            UserMode::Normal => Color::Green,
            UserMode::Interact => Color::Blue,
            UserMode::Search => Color::Yellow,
        }
    }
}
