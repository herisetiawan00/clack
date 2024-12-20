use crate::{
    entities::slack::{
        authorization::Authorization, conversations::Channel, messages::Message, users::Member,
    },
    enums::{section::Section, user_mode::UserMode},
};

#[derive(Clone, PartialEq)]
pub struct State {
    pub global: GlobalState,
    pub channel: ChannelState,
    pub message: MessageState,
    pub input: InputState,
    pub thread: ThreadState,
}

#[derive(Clone, PartialEq)]
pub struct GlobalState {
    pub members: Vec<Member>,
    pub section: Section,
    pub exit: bool,
}

#[derive(Clone, PartialEq)]
pub struct InputState {
    pub value: String,
    pub send: bool,
}

#[derive(Clone, PartialEq)]
pub struct ChannelState {
    pub channels: Vec<Channel>,
    pub direct_messages: Vec<Channel>,
    pub selected: Option<Channel>,
    pub selected_index: Option<usize>,
    pub opened: Option<Channel>,
    pub search: String,
}

#[derive(Clone, PartialEq)]
pub struct MessageState {
    pub messages: Vec<Message>,
    pub selected: Option<Message>,
    pub selected_index: Option<usize>,
    pub opened: Option<Message>,
}

#[derive(Clone, PartialEq)]
pub struct ThreadState {
    pub messages: Vec<Message>,
    pub selected: Option<Message>,
    pub selected_index: Option<usize>,
}

impl State {
    pub fn new() -> State {
        State {
            global: GlobalState::new(),
            channel: ChannelState::new(),
            message: MessageState::new(),
            input: InputState::new(),
            thread: ThreadState::new(),
        }
    }
}

impl GlobalState {
    pub fn new() -> GlobalState {
        GlobalState {
            members: Vec::new(),
            section: Section::Channel,
            exit: false,
        }
    }
    pub fn get_user(&self, id: String) -> Option<Member> {
        self.members
            .iter()
            .filter(|user| user.id == id)
            .next()
            .map_or(None, |user| Some(user.clone()))
    }
}

impl ChannelState {
    pub fn new() -> ChannelState {
        ChannelState {
            channels: Vec::new(),
            direct_messages: Vec::new(),
            selected: None,
            selected_index: None,
            opened: None,
            search: String::new(),
        }
    }
}

impl MessageState {
    pub fn new() -> MessageState {
        MessageState {
            messages: Vec::new(),
            selected: None,
            selected_index: None,
            opened: None,
        }
    }
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            value: String::new(),
            send: false,
        }
    }
}

impl ThreadState {
    pub fn new() -> ThreadState {
        ThreadState {
            messages: Vec::new(),
            selected: None,
            selected_index: None,
        }
    }
}
