use crate::app::Mode;
use crate::input::KeyChord;
use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Keybindings {
    // Action ID -> KeyChord mapping per mode
    pub mode_bindings: HashMap<Mode, HashMap<String, KeyChord>>,
}

impl Default for Keybindings {
    fn default() -> Self {
        let mut mode_bindings = HashMap::new();

        let mut normal_bindings = HashMap::new();
        normal_bindings.insert("edit_message".to_string(), parse_key_chord("e").unwrap());
        normal_bindings.insert("delete_message".to_string(), parse_key_chord("d").unwrap());
        normal_bindings.insert("fork_branch".to_string(), parse_key_chord("f").unwrap());
        normal_bindings.insert("hide_message".to_string(), parse_key_chord("h").unwrap());
        normal_bindings.insert("append_message".to_string(), parse_key_chord("a").unwrap());
        normal_bindings.insert("show_help".to_string(), parse_key_chord("?").unwrap());
        normal_bindings.insert("scroll_up".to_string(), parse_key_chord("up").unwrap());
        normal_bindings.insert("scroll_down".to_string(), parse_key_chord("down").unwrap());
        normal_bindings.insert(
            "scroll_half_page_up".to_string(),
            parse_key_chord("ctrl-u").unwrap(),
        );
        normal_bindings.insert(
            "scroll_half_page_down".to_string(),
            parse_key_chord("ctrl-d").unwrap(),
        );
        normal_bindings.insert(
            "scroll_page_up".to_string(),
            parse_key_chord("pgup").unwrap(),
        );
        normal_bindings.insert(
            "scroll_page_down".to_string(),
            parse_key_chord("pgdn").unwrap(),
        );
        normal_bindings.insert(
            "scroll_to_top".to_string(),
            parse_key_chord("home").unwrap(),
        );
        normal_bindings.insert(
            "scroll_to_bottom".to_string(),
            parse_key_chord("end").unwrap(),
        );
        normal_bindings.insert(
            "scroll_to_previous_message".to_string(),
            parse_key_chord("shift-up").unwrap(),
        );
        normal_bindings.insert(
            "scroll_to_next_message".to_string(),
            parse_key_chord("shift-down").unwrap(),
        );
        normal_bindings.insert("prev_branch".to_string(), parse_key_chord("left").unwrap());
        normal_bindings.insert("next_branch".to_string(), parse_key_chord("right").unwrap());
        normal_bindings.insert(
            "prev_tree".to_string(),
            parse_key_chord("shift-left").unwrap(),
        );
        normal_bindings.insert(
            "next_tree".to_string(),
            parse_key_chord("shift-right").unwrap(),
        );
        normal_bindings.insert(
            "command_palette".to_string(),
            parse_key_chord("ctrl-p").unwrap(),
        );
        normal_bindings.insert("quit".to_string(), parse_key_chord("q").unwrap());

        mode_bindings.insert(Mode::Normal, normal_bindings);

        let mut edit_bindings = HashMap::new();
        edit_bindings.insert("exit_mode".to_string(), parse_key_chord("esc").unwrap());

        mode_bindings.insert(
            Mode::Edit(crate::app::EditMode::Append),
            edit_bindings.clone(),
        );
        mode_bindings.insert(
            Mode::Edit(crate::app::EditMode::EditInPlace {
                message_id: uuid::Uuid::nil(),
                has_messages_below: false,
            }),
            edit_bindings,
        );
        Self { mode_bindings }
    }
}

impl Keybindings {
    pub fn get_key_for_action(&self, mode: Mode, action_id: &str) -> Option<&KeyChord> {
        self.mode_bindings.get(&mode)?.get(action_id)
    }

    pub fn set_key_for_action(&mut self, mode: Mode, action_id: String, key: KeyChord) {
        self.mode_bindings
            .entry(mode)
            .or_default()
            .insert(action_id, key);
    }

    pub fn get_action_for_key(&self, mode: Mode, key: &KeyChord) -> Option<String> {
        self.mode_bindings
            .get(&mode)?
            .iter()
            .find(|(_, k)| *k == key)
            .map(|(action_id, _)| action_id.clone())
    }

    pub fn from_string_map(
        raw: HashMap<String, HashMap<String, String>>,
    ) -> Result<Keybindings, String> {
        let mut mode_bindings = HashMap::new();
        for (mode_str, actions) in raw {
            let mode = parse_mode(&mode_str)?; // You'll need a parser for Mode
            let mut action_map = HashMap::new();
            for (action_id, key_str) in actions {
                let key_chord = parse_key_chord(&key_str)
                    .ok_or_else(|| format!("Invalid key chord: {}", key_str))?;
                action_map.insert(action_id, key_chord);
            }
            mode_bindings.insert(mode, action_map);
        }
        Ok(Keybindings { mode_bindings })
    }
}

pub fn parse_key_chord(s: &str) -> Option<KeyChord> {
    let mut modifiers = KeyModifiers::NONE;
    let mut parts = s.split('-').collect::<Vec<_>>();

    // Check for modifiers at the start
    while let Some(&part) = parts.first() {
        match part {
            "ctrl" => {
                modifiers |= KeyModifiers::CONTROL;
                parts.remove(0);
            }
            "shift" => {
                modifiers |= KeyModifiers::SHIFT;
                parts.remove(0);
            }
            "alt" => {
                modifiers |= KeyModifiers::ALT;
                parts.remove(0);
            }
            _ => break,
        }
    }

    // The remaining part is the key
    let key_str = parts.join("-"); // In case key itself contains '-'

    let key = match key_str.as_str() {
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "esc" => KeyCode::Esc,
        "enter" => KeyCode::Enter,
        "tab" => KeyCode::Tab,
        "backspace" => KeyCode::Backspace,
        "delete" => KeyCode::Delete,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        "pgup" => KeyCode::PageUp,
        "pgdn" => KeyCode::PageDown,
        "?" => KeyCode::Char('?'),
        c if c.len() == 1 => KeyCode::Char(c.chars().next().unwrap()),
        _ => return None,
    };

    Some(KeyChord::new(key, modifiers))
}

pub fn parse_mode(s: &str) -> Result<Mode, String> {
    match s {
        "normal" => Ok(Mode::Normal),
        "edit" | "edit-append" => Ok(Mode::Edit(crate::app::EditMode::Append)),
        "edit-inplace" => Ok(Mode::Edit(crate::app::EditMode::EditInPlace {
            message_id: uuid::Uuid::nil(),
            has_messages_below: false,
        })),
        _ => Err(format!("Unknown mode: {}", s)),
    }
}
