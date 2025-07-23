use crate::app::{EditMode, Mode};
use crate::input::KeyChord;
use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use std::collections::HashMap;

pub fn default_keybindings_string_map() -> HashMap<String, HashMap<String, String>> {
    [
        (
            "normal",
            &[
                ("edit_message", "e"),
                ("delete_message", "d"),
                ("fork_branch", "f"),
                ("hide_message", "h"),
                ("append_message", "a"),
                ("show_help", "?"),
                ("scroll_up", "up"),
                ("scroll_down", "down"),
                ("scroll_half_page_up", "ctrl-u"),
                ("scroll_half_page_down", "ctrl-d"),
                ("scroll_page_up", "pgup"),
                ("scroll_page_down", "pgdn"),
                ("scroll_to_top", "home"),
                ("scroll_to_bottom", "end"),
                ("scroll_to_previous_message", "shift-up"),
                ("scroll_to_next_message", "shift-down"),
                ("prev_branch", "left"),
                ("next_branch", "right"),
                ("prev_tree", "shift-left"),
                ("next_tree", "shift-right"),
                ("command_palette", "ctrl-p"),
                ("quit", "q"),
            ][..],
        ),
        ("edit-append", &[("exit_mode", "esc")]),
        ("edit-inplace", &[("exit_mode", "esc")]),
    ]
    .into_iter()
    .map(|(mode, actions)| {
        (
            mode.to_string(),
            actions
                .iter()
                .map(|(action, key)| (action.to_string(), key.to_string()))
                .collect(),
        )
    })
    .collect()
}

#[derive(Debug, Clone)]
pub struct Keybindings {
    // Action ID -> KeyChord mapping per mode
    pub mode_bindings: HashMap<Mode, HashMap<String, KeyChord>>,
}

impl Default for Keybindings {
    fn default() -> Self {
        Keybindings::from_string_map(default_keybindings_string_map())
            .expect("Default keybindings string map is valid")
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
        validate_raw_config(&raw)?;
        let mode_bindings = build_mode_bindings(raw)?;
        Ok(Keybindings { mode_bindings })
    }
}

fn validate_raw_config(raw: &HashMap<String, HashMap<String, String>>) -> Result<(), String> {
    for (mode_str, raw_actions) in raw {
        parse_mode(mode_str)?;
        for (action_id, key_str) in raw_actions {
            parse_key_chord(key_str).ok_or_else(|| {
                format!("Invalid key chord: {} for action: {}", key_str, action_id)
            })?;
        }
    }
    Ok(())
}

fn build_mode_bindings(
    raw: HashMap<String, HashMap<String, String>>,
) -> Result<HashMap<Mode, HashMap<String, KeyChord>>, String> {
    let defaults = default_keybindings_string_map();
    let mut mode_bindings = HashMap::new();
    let empty_map = HashMap::new();

    for (mode_str, default_actions) in &defaults {
        let mode = parse_mode(mode_str)?;
        let mut action_map = HashMap::new();

        let raw_actions = raw.get(mode_str).unwrap_or(&empty_map);

        for (action_id, default_key) in default_actions {
            let key_str = raw_actions.get(action_id).unwrap_or(default_key);
            let key_chord = parse_key_chord(key_str)
                .ok_or_else(|| format!("Invalid key chord: {}", key_str))?;
            action_map.insert(action_id.clone(), key_chord);
        }

        mode_bindings.insert(mode, action_map);
    }
    Ok(mode_bindings)
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
        "edit" | "edit-append" => Ok(Mode::Edit(EditMode::Append)),
        "edit-inplace" => Ok(Mode::Edit(EditMode::EditInPlace {
            message_id: uuid::Uuid::nil(),
            has_messages_below: false,
        })),
        _ => Err(format!("Unknown mode: {}", s)),
    }
}
