use crate::app::Mode;
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
        (
            "edit",
            &[
                // Mode operations
                ("exit_mode", "esc"),
                ("submit_message", "ctrl-enter"),
                // Character operations
                ("delete_char", "backspace"),
                ("delete_next_char", "delete"),
                ("insert_newline", "enter"),
                // Line operations
                ("delete_line_by_end", "alt-k"),
                ("delete_line_by_head", "alt-u"),
                // Word operations
                ("delete_word", "ctrl-w"),
                ("delete_next_word", "alt-d"),
                // Undo/Redo
                ("undo", "ctrl-z"),
                ("redo", "ctrl-Z"),
                // Clipboard operations
                ("copy", "ctrl-c"),
                ("cut", "ctrl-x"),
                ("paste", "ctrl-v"),
                // Selection operations
                ("select_all", "ctrl-a"),
                // Selection movement - character level
                ("select_forward", "shift-right"),
                ("select_back", "shift-left"),
                ("select_up", "shift-up"),
                ("select_down", "shift-down"),
                // Selection movement - word level
                ("select_word_forward", "ctrl-shift-right"),
                ("select_word_back", "ctrl-shift-left"),
                // Selection movement - line boundaries
                ("select_to_end", "shift-end"),
                ("select_to_head", "shift-home"),
                // Cursor movement - character level
                ("move_cursor_forward", "right"),
                ("move_cursor_back", "left"),
                ("move_cursor_up", "up"),
                ("move_cursor_down", "down"),
                // Cursor movement - word level
                ("move_cursor_word_forward", "ctrl-right"),
                ("move_cursor_word_end", "ctrl-e"),
                ("move_cursor_word_back", "ctrl-left"),
                // Cursor movement - paragraph level
                ("move_cursor_paragraph_forward", "ctrl-down"),
                ("move_cursor_paragraph_back", "ctrl-up"),
                // Cursor movement - line boundaries
                ("move_cursor_end", "end"),
                ("move_cursor_head", "home"),
                // Cursor movement - document boundaries
                ("move_cursor_top", "ctrl-home"),
                ("move_cursor_bottom", "ctrl-end"),
                // Cursor movement - viewport
                ("move_cursor_in_viewport", "f2"),
                // Help
                ("show_help", "f1"),
            ],
        ),
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
        // Function keys
        "f1" => KeyCode::F(1),
        "f2" => KeyCode::F(2),
        "f3" => KeyCode::F(3),
        "f4" => KeyCode::F(4),
        "f5" => KeyCode::F(5),
        "f6" => KeyCode::F(6),
        "f7" => KeyCode::F(7),
        "f8" => KeyCode::F(8),
        "f9" => KeyCode::F(9),
        "f10" => KeyCode::F(10),
        "f11" => KeyCode::F(11),
        "f12" => KeyCode::F(12),
        "?" => KeyCode::Char('?'),
        c if c.len() == 1 => KeyCode::Char(c.chars().next().unwrap()),
        _ => return None,
    };

    Some(KeyChord::new(key, modifiers))
}

pub fn parse_mode(s: &str) -> Result<Mode, String> {
    match s {
        "normal" => Ok(Mode::Normal),
        "edit" => Ok(Mode::Edit),
        _ => Err(format!("Unknown mode: {}", s)),
    }
}
