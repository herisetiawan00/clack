use crossterm::event::{KeyCode, KeyModifiers};

fn key_code_from_string(key: &str) -> KeyCode {
    match key {
        "down" => KeyCode::Down,
        "up" => KeyCode::Up,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "enter" => KeyCode::Enter,
        "backspace" => KeyCode::Backspace,
        "delete" => KeyCode::Delete,
        c if c.len() == 1 => match c.chars().next() {
            Some(char) => KeyCode::Char(char),
            None => panic!("key code not supported: {}", key),
        },
        _ => panic!("key code not supported: {}", key),
    }
}

fn key_modifier_from_string(key: &str) -> KeyModifiers {
    match key {
        "ctrl" => KeyModifiers::CONTROL,
        "alt" => KeyModifiers::ALT,
        "shift" => KeyModifiers::SHIFT,
        "super" => KeyModifiers::SUPER,
        _ => panic!("key modifiers not supported: {}", key),
    }
}

pub fn from_string(key: String) -> (KeyModifiers, KeyCode) {
    let splitted_key: Vec<&str> = key.split("+").collect();

    let mut key_modifiers = KeyModifiers::NONE;
    let mut key_code = KeyCode::Null;

    if splitted_key.len() == 1 {
        key_code = key_code_from_string(splitted_key[0]);
    } else if splitted_key.len() == 2 {
        key_modifiers = key_modifier_from_string(splitted_key[0]);
        key_code = key_code_from_string(splitted_key[1]);
    } else {
        panic!("key not supported: {}", key);
    }

    (key_modifiers, key_code)
}
