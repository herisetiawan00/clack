use std::cmp::max;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{
    entities::{configuration::Configuration, slack::conversations::Channel},
    enums::{section::Section, user_mode::UserMode},
    states::{ChannelState, MessageState, State, ThreadState},
    utils,
};

pub fn generic(event: Event, config: &Configuration, state: &mut State) {
    let up = utils::keycode::from_string(config.keymaps.focus.up.clone());
    let down = utils::keycode::from_string(config.keymaps.focus.down.clone());
    let left = utils::keycode::from_string(config.keymaps.focus.left.clone());
    let right = utils::keycode::from_string(config.keymaps.focus.right.clone());
    let exit = utils::keycode::from_string(config.keymaps.exit.clone());

    match state.global.mode {
        UserMode::Normal => match event {
            Event::Key(KeyEvent {
                modifiers, code, ..
            }) => match (modifiers, code) {
                key if key == up => match state.global.section {
                    Section::Input => {
                        state.global.section = if state.message.opened.is_some() {
                            Section::Thread
                        } else {
                            Section::Message
                        }
                    }
                    _ => {}
                },
                key if key == down => match state.global.section {
                    Section::Message | Section::Thread => state.global.section = Section::Input,
                    _ => {}
                },
                key if key == left => match state.global.section {
                    Section::Message | Section::Input => state.global.section = Section::Channel,
                    Section::Thread => state.global.section = Section::Message,
                    _ => {}
                },
                key if key == right => match state.global.section {
                    Section::Channel => state.global.section = Section::Message,
                    Section::Message => {
                        if state.message.opened.is_some() {
                            state.global.section = Section::Thread
                        }
                    }
                    _ => {}
                },
                key if key == exit => {
                    state.global.exit = true;
                }
                _ => {}
            },

            _ => {}
        },
        _ => {}
    }
}

pub fn channels(event: Event, config: &Configuration, state: &mut State) {
    if state.global.section != Section::Channel {
        return;
    }

    let up = utils::keycode::from_string(config.keymaps.up.clone());
    let down = utils::keycode::from_string(config.keymaps.down.clone());
    let right = utils::keycode::from_string(config.keymaps.right.clone());
    let open = utils::keycode::from_string(config.keymaps.open.clone());
    let search = utils::keycode::from_string(config.keymaps.search.clone());

    let filtered_channels: Vec<&Channel> = state
        .channel
        .channels
        .iter()
        .filter(|channel| {
            channel
                .name
                .clone()
                .unwrap_or(channel.id.clone())
                .contains(&state.channel.search)
        })
        .collect();
    let last_index = max(filtered_channels.len() as i32 - 1, 0) as usize;

    if let Some(channel) = state.channel.selected.clone() {
        if !filtered_channels.contains(&&channel) {
            state.channel.selected = None;
            state.channel.selected_index = None;
        }
    }

    match state.global.mode {
        UserMode::Normal => match event {
            Event::Key(KeyEvent {
                modifiers, code, ..
            }) => match (modifiers, code) {
                key if key == up => {
                    let next_index = match state.channel.selected.clone() {
                        Some(selected) => {
                            let index = filtered_channels
                                .iter()
                                .position(|channel| channel == &&selected);

                            match index {
                                Some(index) => {
                                    if index == 0 {
                                        last_index
                                    } else {
                                        index - 1
                                    }
                                }
                                None => last_index,
                            }
                        }
                        None => last_index,
                    };

                    state.channel.selected = Some(filtered_channels[next_index].clone());
                    state.channel.selected_index = Some(next_index);
                }
                key if key == down => {
                    let next_index = match state.channel.selected.clone() {
                        Some(selected) => {
                            let index = filtered_channels
                                .iter()
                                .position(|channel| channel.to_owned() == &selected);

                            match index {
                                Some(index) => {
                                    if index == last_index {
                                        0
                                    } else {
                                        index + 1
                                    }
                                }
                                None => 0,
                            }
                        }
                        None => 0,
                    };

                    state.channel.selected = Some(filtered_channels[next_index].clone());
                    state.channel.selected_index = Some(next_index);
                }
                key if key == open || key == right => {
                    if state.channel.opened != state.channel.selected {
                        state.channel.opened = state.channel.selected.clone();
                        state.message = MessageState::new();
                        state.thread = ThreadState::new();
                    }
                    state.global.section = Section::Message;
                }
                key if key == search => {
                    state.global.mode = UserMode::Search;
                }
                (KeyModifiers::NONE, KeyCode::Esc) => {
                    state.channel.search = String::new();
                }
                _ => {}
            },

            _ => {}
        },
        UserMode::Search => match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char(code) => {
                    state.channel.search.push(code);
                }
                KeyCode::Delete | KeyCode::Backspace => {
                    state.channel.search.pop();
                }
                KeyCode::Enter => {
                    state.global.mode = UserMode::Normal;
                }
                KeyCode::Esc => {
                    state.global.mode = UserMode::Normal;
                    state.channel.search = String::new();
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }
}

pub fn input(event: Event, config: &Configuration, state: &mut State) {
    if state.global.section != Section::Input {
        return;
    }

    let interact = utils::keycode::from_string(config.keymaps.interact.clone());
    let send = utils::keycode::from_string(config.keymaps.send.clone());

    match state.global.mode {
        UserMode::Normal => match event {
            Event::Key(KeyEvent {
                modifiers, code, ..
            }) => match (modifiers, code) {
                key if key == interact => {
                    state.global.mode = UserMode::Interact;
                }
                (KeyModifiers::NONE, KeyCode::Enter) => {
                    state.input.send = true;
                }
                _ => {}
            },
            _ => {}
        },
        UserMode::Interact => match event {
            Event::Key(KeyEvent {
                modifiers, code, ..
            }) => match (modifiers, code) {
                key if key == send => {
                    state.input.send = true;
                }
                (KeyModifiers::NONE, KeyCode::Char(code)) => {
                    state.input.value.push(code);
                }
                (KeyModifiers::SHIFT, KeyCode::Char(code)) => {
                    state
                        .input
                        .value
                        .push_str(code.to_uppercase().to_string().as_str());
                }
                (KeyModifiers::NONE, code)
                    if code == KeyCode::Delete || code == KeyCode::Backspace =>
                {
                    state.input.value.pop();
                }
                (KeyModifiers::NONE, KeyCode::Enter) => {
                    state.input.value.push_str("\n");
                }
                (KeyModifiers::NONE, KeyCode::Esc) => {
                    state.global.mode = UserMode::Normal;
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    };
}

pub fn messages(event: Event, config: &Configuration, state: &mut State) {
    if state.global.section != Section::Message {
        return;
    }

    let up = utils::keycode::from_string(config.keymaps.up.clone());
    let down = utils::keycode::from_string(config.keymaps.down.clone());
    let right = utils::keycode::from_string(config.keymaps.right.clone());

    let last_index = max(state.message.messages.len() as i32 - 1, 0) as usize;

    match state.global.mode {
        UserMode::Normal => match event {
            Event::Key(KeyEvent {
                modifiers, code, ..
            }) => match (modifiers, code) {
                key if key == up => {
                    if state.message.messages.is_empty() {
                        return;
                    }
                    let next_index = match state.message.selected.clone() {
                        Some(selected) => {
                            let index = state
                                .message
                                .messages
                                .iter()
                                .position(|message| message == &selected);

                            match index {
                                Some(index) => {
                                    if index > 0 {
                                        index - 1
                                    } else {
                                        index
                                    }
                                }
                                None => last_index,
                            }
                        }
                        None => last_index,
                    };

                    state.message.selected = Some(state.message.messages[next_index].clone());
                    state.message.selected_index = Some(next_index);
                }
                key if key == down => {
                    if state.message.messages.is_empty() {
                        return;
                    }
                    let next_index = match state.message.selected.clone() {
                        Some(selected) => {
                            let index = state
                                .message
                                .messages
                                .iter()
                                .position(|message| message == &selected);

                            match index {
                                Some(index) => {
                                    if index < last_index {
                                        index + 1
                                    } else {
                                        index
                                    }
                                }
                                None => last_index,
                            }
                        }
                        None => last_index,
                    };

                    state.message.selected = Some(state.message.messages[next_index].clone());
                    state.message.selected_index = Some(next_index);
                }
                key if key == right => {
                    if state.message.selected.is_some() {
                        state.message.opened = state.message.selected.clone();
                        state.global.section = Section::Thread;
                    }
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }
}

pub fn thread(event: Event, config: &Configuration, state: &mut State) {
    if state.global.section != Section::Thread {
        return;
    }

    let up = utils::keycode::from_string(config.keymaps.up.clone());
    let down = utils::keycode::from_string(config.keymaps.down.clone());

    let last_index = max(state.thread.messages.len() as i32 - 1, 0) as usize;

    match state.global.mode {
        UserMode::Normal => match event {
            Event::Key(KeyEvent {
                modifiers, code, ..
            }) => match (modifiers, code) {
                key if key == up => {
                    let next_index = match state.thread.selected.clone() {
                        Some(selected) => {
                            let index = state
                                .thread
                                .messages
                                .iter()
                                .position(|message| message == &selected);

                            match index {
                                Some(index) => {
                                    if index > 0 {
                                        index - 1
                                    } else {
                                        index
                                    }
                                }
                                None => last_index,
                            }
                        }
                        None => last_index,
                    };

                    state.thread.selected = Some(state.thread.messages[next_index].clone());
                    state.thread.selected_index = Some(next_index);
                }
                key if key == down => {
                    if state.thread.messages.is_empty() {
                        return;
                    }
                    let next_index = match state.thread.selected.clone() {
                        Some(selected) => {
                            let index = state
                                .thread
                                .messages
                                .iter()
                                .position(|message| message == &selected);

                            match index {
                                Some(index) => {
                                    if index < last_index {
                                        index + 1
                                    } else {
                                        index
                                    }
                                }
                                None => 0,
                            }
                        }
                        None => 0,
                    };

                    state.thread.selected = Some(state.thread.messages[next_index].clone());
                    state.thread.selected_index = Some(next_index);
                }
                (KeyModifiers::NONE, KeyCode::Esc) => {
                    state.message.opened = None;
                    state.global.section = Section::Message;
                    state.thread = ThreadState::new();
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }
}
