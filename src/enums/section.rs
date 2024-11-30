#[derive(PartialEq, Clone)]
pub enum Section {
    Channel,
    Message,
    Input,
    StatusLine,
    Thread,
    Command,
    Unknown,
}

impl Section {
    pub fn to_string(&self) -> String {
        match self {
            Section::Channel => "channel".to_string(),
            Section::Message => "message".to_string(),
            Section::Input => "input".to_string(),
            Section::StatusLine => "status_line".to_string(),
            _ => String::new(),
        }
    }

    pub fn from_str(value: &str) -> Section {
        match value {
            "channel" => Section::Channel,
            "message" => Section::Message,
            "input" => Section::Input,
            "status_line" => Section::StatusLine,
            _ => Section::Unknown,
        }
    }
}
