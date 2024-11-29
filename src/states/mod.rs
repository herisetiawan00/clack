use crate::{
    entities::slack::{
        authorization::Authorization, conversations::Channel, messages::Message, users::Member,
    },
    enums::{section::Section, user_mode::UserMode},
};

#[derive(Clone)]
pub struct State {
    pub authorization: Option<Authorization>,
    pub global: GlobalState,
    pub channel: ChannelState,
    pub message: MessageState,
    pub input: InputState,
}

#[derive(Clone)]
pub struct GlobalState {
    pub members: Vec<Member>,
    pub mode: UserMode,
    pub section: Section,
    pub exit: bool,
}

#[derive(Clone)]
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
}

impl State {
    pub fn new() -> State {
        State {
            authorization: None,
            global: GlobalState {
                members: Vec::new(),
                mode: UserMode::Normal,
                section: Section::Channel,
                exit: false,
            },
            channel: ChannelState {
                channels: Vec::new(),
                direct_messages: Vec::new(),
                selected: None,
                selected_index: None,
                opened: None,
                search: String::new(),
            },
            message: MessageState {
                messages: Vec::new(),
                selected: None,
                selected_index: None,
            },
            input: InputState {
                value: String::new(),
                send: false,
            },
        }
    }
}

impl GlobalState {
    pub fn get_user(&self, id: String) -> Option<Member> {
        self.members
            .iter()
            .filter(|user| user.id == id)
            .next()
            .map_or(None, |user| Some(user.clone()))
    }
}
