use crate::enums::{self, user_mode::UserMode};

pub fn initialize() -> super::State {
    super::State {
        authorization: None,
        global: super::GlobalState {
            members: Vec::new(),
            mode: UserMode::Normal,
        },
        navigator: super::NavigatorState {
            section: enums::section::Section::Channel,
            exit: false,
        },
        channel: super::ChannelState {
            channels: Vec::new(),
            direct_messages: Vec::new(),
            selected: None,
            selected_index: None,
            opened: None,
            search: String::new(),
        },
        message: super::MessageState {
            messages: Vec::new(),
            selected: None,
            selected_index: None,
        },
        input: super::InputState {
            value: String::new(),
            send: false,
        },
    }
}
