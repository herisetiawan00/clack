use std::cmp::max;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{
    entities::{configuration::Configuration, slack::conversations::Channel},
    enums::{section::Section, user_mode::UserMode},
    states::{ChannelState, NavigatorState, State},
    utils,
};

pub fn generic(event: Event, config: &Configuration, mut state: State) -> State {
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
                key if key == up => match state.navigator.section {
                    Section::Input => state.navigator.section = Section::Message,
                    _ => {}
                },
                key if key == down => match state.navigator.section {
                    Section::Message => state.navigator.section = Section::Input,
                    _ => {}
                },
                key if key == left => match state.navigator.section {
                    Section::Message | Section::Input => state.navigator.section = Section::Channel,
                    _ => {}
                },
                key if key == right => match state.navigator.section {
                    Section::Channel => state.navigator.section = Section::Message,
                    _ => {}
                },
                key if key == exit => {
                    state.navigator.exit = true;
                }
                _ => {}
            },

            _ => {}
        },
        _ => {}
    }

    state
}

pub fn channels(event: Event, config: &Configuration, mut state: State) -> State {
    let mut cloned_state = state.clone();

    if state.navigator.section != Section::Channel {
        return state;
    }

    let up = utils::keycode::from_string(config.keymaps.up.clone());
    let down = utils::keycode::from_string(config.keymaps.down.clone());
    let right = utils::keycode::from_string(config.keymaps.right.clone());
    let open = utils::keycode::from_string(config.keymaps.open.clone());
    let search = utils::keycode::from_string(config.keymaps.search.clone());

    let filtered_channels: Vec<&Channel> = cloned_state
        .channel
        .channels
        .iter()
        .filter(|channel| {
            channel
                .name
                .clone()
                .unwrap_or(channel.id.clone())
                .contains(&cloned_state.channel.search)
        })
        .collect();
    let last_index = max(filtered_channels.len() as i32 - 1, 0) as usize;

    if let Some(channel) = cloned_state.channel.selected.clone() {
        if !filtered_channels.contains(&&channel) {
            cloned_state.channel.selected = None;
            cloned_state.channel.selected_index = None;
        }
    }

    match cloned_state.global.mode {
        UserMode::Normal => match event {
            Event::Key(KeyEvent {
                modifiers, code, ..
            }) => match (modifiers, code) {
                key if key == up => {
                    let next_index = match cloned_state.channel.selected {
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

                    cloned_state.channel.selected = Some(filtered_channels[next_index].clone());
                    cloned_state.channel.selected_index = Some(next_index);
                }
                key if key == down => {
                    let next_index = match cloned_state.channel.selected {
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

                    cloned_state.channel.selected = Some(filtered_channels[next_index].clone());
                    cloned_state.channel.selected_index = Some(next_index);
                }
                key if key == open || key == right => {
                    cloned_state.channel.opened = cloned_state.channel.selected.clone();
                    cloned_state.navigator.section = Section::Message;
                }
                key if key == search => {
                    cloned_state.global.mode = UserMode::Search;
                }
                (KeyModifiers::NONE, KeyCode::Esc) => {
                    cloned_state.channel.search = String::new();
                }
                _ => {}
            },

            _ => {}
        },
        UserMode::Search => match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char(code) => {
                    cloned_state.channel.search.push(code);
                }
                KeyCode::Delete | KeyCode::Backspace => {
                    cloned_state.channel.search.pop();
                }
                KeyCode::Enter => {
                    cloned_state.global.mode = UserMode::Normal;
                }
                KeyCode::Esc => {
                    cloned_state.global.mode = UserMode::Normal;
                    cloned_state.channel.search = String::new();
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }

    state = cloned_state;

    state
}

pub fn input(event: Event, config: &Configuration, mut state: State) -> State {
    let mut cloned_state = state.clone();

    if state.navigator.section != Section::Input {
        return state;
    }

    let interact = utils::keycode::from_string(config.keymaps.interact.clone());
    let send = utils::keycode::from_string(config.keymaps.send.clone());

    match cloned_state.global.mode {
        UserMode::Normal => match event {
            Event::Key(KeyEvent {
                modifiers, code, ..
            }) => match (modifiers, code) {
                key if key == interact => {
                    cloned_state.global.mode = UserMode::Interact;
                }
                (KeyModifiers::NONE, KeyCode::Enter) => {
                    cloned_state.input.send = true;
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
                    cloned_state.input.send = true;
                }
                (KeyModifiers::NONE, KeyCode::Char(code)) => {
                    cloned_state.input.value.push(code);
                }
                (KeyModifiers::SHIFT, KeyCode::Char(code)) => {
                    cloned_state
                        .input
                        .value
                        .push_str(code.to_uppercase().to_string().as_str());
                }
                (KeyModifiers::NONE, code)
                    if code == KeyCode::Delete || code == KeyCode::Backspace =>
                {
                    cloned_state.input.value.pop();
                }
                (KeyModifiers::NONE, KeyCode::Enter) => {
                    cloned_state.input.value.push_str("\n");
                }
                (KeyModifiers::NONE, KeyCode::Esc) => {
                    cloned_state.global.mode = UserMode::Normal;
                    //cloned_state.input.value = String::new();
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    };

    state = cloned_state;

    state
}

pub fn messages(event: Event, config: &Configuration, mut state: State) -> State {
    let mut cloned_state = state.clone();

    if state.navigator.section != Section::Message {
        return state;
    }

    let up = utils::keycode::from_string(config.keymaps.up.clone());
    let down = utils::keycode::from_string(config.keymaps.down.clone());

    let last_index = max(cloned_state.message.messages.len() as i32 - 1, 0) as usize;

    match cloned_state.global.mode {
        UserMode::Normal => match event {
            Event::Key(KeyEvent {
                modifiers, code, ..
            }) => match (modifiers, code) {
                key if key == up => {
                    let next_index = match cloned_state.message.selected {
                        Some(selected) => {
                            let index = cloned_state
                                .message
                                .messages
                                .iter()
                                .position(|message| message == &selected);

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

                    cloned_state.message.selected =
                        Some(cloned_state.message.messages[next_index].clone());
                    cloned_state.message.selected_index = Some(next_index);
                }
                key if key == down => {
                    let next_index = match cloned_state.message.selected {
                        Some(selected) => {
                            let index = cloned_state
                                .message
                                .messages
                                .iter()
                                .position(|message| message == &selected);

                            match index {
                                Some(index) => {
                                    if index == last_index {
                                        0
                                    } else {
                                        index + 1
                                    }
                                }
                                None => last_index,
                            }
                        }
                        None => last_index,
                    };

                    cloned_state.message.selected =
                        Some(cloned_state.message.messages[next_index].clone());
                    cloned_state.message.selected_index = Some(next_index);
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }

    state = cloned_state;

    state
}
