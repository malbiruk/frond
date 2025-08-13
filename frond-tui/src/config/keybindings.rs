use crate::app::Mode;
use crate::input::KeyChord;
use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DefaultKeybinding {
    Single(&'static str),
    Multiple(&'static [&'static str]),
}

impl DefaultKeybinding {
    pub fn to_vec(&self) -> Vec<String> {
        match self {
            DefaultKeybinding::Single(s) => vec![s.to_string()],
            DefaultKeybinding::Multiple(arr) => arr.iter().map(|s| s.to_string()).collect(),
        }
    }
}

pub fn default_keybindings_config() -> HashMap<String, HashMap<String, DefaultKeybinding>> {
    use DefaultKeybinding::*;

    [
        (
            "normal",
            &[
                ("edit_message", Multiple(&["e", "i"])),
                ("delete_message", Multiple(&["d", "del"])),
                ("fork_branch", Single("f")),
                ("hide_message", Single("x")),
                ("append_message", Single("a")),
                ("show_help", Single("?")),
                ("scroll_up", Multiple(&["up", "k"])),
                ("scroll_down", Multiple(&["down", "j"])),
                ("scroll_half_page_up", Single("ctrl-u")),
                ("scroll_half_page_down", Single("ctrl-d")),
                ("scroll_page_up", Multiple(&["pgup", "ctrl-b"])),
                ("scroll_page_down", Multiple(&["pgdn", "ctrl-f"])),
                ("scroll_to_top", Multiple(&["home", "alt-up"])),
                ("scroll_to_bottom", Multiple(&["end", "alt-down"])),
                ("scroll_to_previous_message", Single("shift-up")),
                ("scroll_to_next_message", Single("shift-down")),
                ("prev_branch", Multiple(&["left", "h"])),
                ("next_branch", Multiple(&["right", "l"])),
                ("prev_tree", Multiple(&["shift-left", "H"])),
                ("next_tree", Multiple(&["shift-right", "L"])),
                ("command_palette", Single("ctrl-p")),
                ("normal_undo", Single("ctrl-z")),
                ("normal_redo", Single("ctrl-y")),
                ("quit", Multiple(&["q", "ctrl-c"])),
            ][..],
        ),
        (
            "edit",
            &[
                // Mode operations
                ("exit_edit_mode", Single("esc")),
                ("submit_message", Single("alt-enter")),
                // Character operations
                ("delete_char", Single("backspace")),
                ("delete_next_char", Single("del")),
                ("insert_newline", Single("enter")),
                // Line operations
                ("delete_line_by_end", Single("alt-k")),
                ("delete_line_by_head", Single("alt-u")),
                ("delete_line", Single("ctrl-k")),
                // Word operations
                ("delete_word", Single("alt-backspace")),
                ("delete_next_word", Single("alt-del")),
                // Undo/Redo
                ("undo", Single("ctrl-z")),
                ("redo", Single("ctrl-y")),
                // Clipboard operations
                ("copy", Single("ctrl-c")),
                ("cut", Single("ctrl-x")),
                ("paste", Single("ctrl-v")),
                // Selection operations
                ("select_all", Single("ctrl-a")),
                // Selection movement - character level
                ("select_forward", Single("shift-right")),
                ("select_back", Single("shift-left")),
                ("select_up", Single("shift-up")),
                ("select_down", Single("shift-down")),
                // Selection movement - word level
                ("select_word_forward", Single("ctrl-shift-right")),
                ("select_word_back", Single("ctrl-shift-left")),
                // Selection movement - line boundaries
                ("select_to_end", Multiple(&["shift-end", "shift-alt-right"])),
                (
                    "select_to_head",
                    Multiple(&["shift-home", "shift-alt-left"]),
                ),
                ("select_line", Single("ctrl-l")),
                (
                    "select_to_top",
                    Multiple(&["ctrl-shift-home", "shift-alt-up"]),
                ),
                (
                    "select_to_bottom",
                    Multiple(&["ctrl-shift-end", "shift-alt-down"]),
                ),
                // Cursor movement - character level
                ("move_cursor_forward", Single("right")),
                ("move_cursor_back", Single("left")),
                ("move_cursor_up", Single("up")),
                ("move_cursor_down", Single("down")),
                // Cursor movement - word level
                ("move_cursor_word_forward", Single("ctrl-right")),
                ("move_cursor_word_end", Single("ctrl-e")),
                ("move_cursor_word_back", Single("ctrl-left")),
                // Cursor movement - paragraph level
                ("move_cursor_paragraph_forward", Single("ctrl-down")),
                ("move_cursor_paragraph_back", Single("ctrl-up")),
                // Cursor movement - line boundaries
                ("move_cursor_end", Multiple(&["end", "alt-right"])),
                ("move_cursor_head", Multiple(&["home", "alt-left"])),
                // Cursor movement - document boundaries
                ("move_cursor_top", Multiple(&["ctrl-home", "alt-up"])),
                ("move_cursor_bottom", Multiple(&["ctrl-end", "alt-down"])),
                // Help
                ("show_help", Single("f1")),
            ],
        ),
    ]
    .into_iter()
    .map(|(mode, actions)| {
        (
            mode.to_string(),
            actions
                .iter()
                .map(|(action, binding)| (action.to_string(), binding.clone()))
                .collect(),
        )
    })
    .collect()
}

#[derive(Debug, Clone)]
pub struct Keybindings {
    // Action ID -> Vec<KeyChord> mapping per mode
    pub mode_bindings: HashMap<Mode, HashMap<String, Vec<KeyChord>>>,
}

impl Default for Keybindings {
    fn default() -> Self {
        Keybindings::from_config(default_keybindings_config())
            .expect("Default keybindings config is valid")
    }
}

impl Keybindings {
    pub fn get_keys_for_action(&self, mode: Mode, action_id: &str) -> Option<&Vec<KeyChord>> {
        self.mode_bindings.get(&mode)?.get(action_id)
    }

    pub fn add_key_for_action(&mut self, mode: Mode, action_id: String, key: KeyChord) {
        self.mode_bindings
            .entry(mode)
            .or_default()
            .entry(action_id)
            .or_default()
            .push(key);
    }

    pub fn set_keys_for_action(&mut self, mode: Mode, action_id: String, keys: Vec<KeyChord>) {
        self.mode_bindings
            .entry(mode)
            .or_default()
            .insert(action_id, keys);
    }

    pub fn get_action_for_key(&self, mode: Mode, key: &KeyChord) -> Option<String> {
        self.mode_bindings
            .get(&mode)?
            .iter()
            .find(|(_, keys)| keys.contains(key))
            .map(|(action_id, _)| action_id.clone())
    }

    pub fn from_config(
        config: HashMap<String, HashMap<String, DefaultKeybinding>>,
    ) -> Result<Keybindings, String> {
        validate_config(&config)?;
        let mode_bindings = build_mode_bindings(config)?;
        Ok(Keybindings { mode_bindings })
    }
}

fn validate_config(
    config: &HashMap<String, HashMap<String, DefaultKeybinding>>,
) -> Result<(), String> {
    for (mode_str, actions) in config {
        parse_mode(mode_str)?;
        for (action_id, binding) in actions {
            for key_str in binding.to_vec() {
                parse_key_chord(&key_str).ok_or_else(|| {
                    format!("Invalid key chord: {} for action: {}", key_str, action_id)
                })?;
            }
        }
    }
    Ok(())
}

fn build_mode_bindings(
    config: HashMap<String, HashMap<String, DefaultKeybinding>>,
) -> Result<HashMap<Mode, HashMap<String, Vec<KeyChord>>>, String> {
    let defaults = default_keybindings_config();
    let mut mode_bindings = HashMap::new();

    for (mode_str, default_actions) in &defaults {
        let mode = parse_mode(mode_str)?;
        let mut action_map = HashMap::new();

        let config_actions = config.get(mode_str);

        for (action_id, default_binding) in default_actions {
            let binding = config_actions
                .and_then(|actions| actions.get(action_id))
                .unwrap_or(default_binding);

            let key_chords: Result<Vec<KeyChord>, String> = binding
                .to_vec()
                .into_iter()
                .map(|key_str| {
                    parse_key_chord(&key_str)
                        .ok_or_else(|| format!("Invalid key chord: {}", key_str))
                })
                .collect();

            action_map.insert(action_id.clone(), key_chords?);
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
        "del" => KeyCode::Delete,
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
        c if c.len() == 1 => {
            let ch = c.chars().next().unwrap();
            KeyCode::Char(ch)
        }
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
