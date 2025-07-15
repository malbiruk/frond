use crate::app::Mode;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Keybindings {
    pub mode_bindings: HashMap<Mode, Vec<KeyBinding>>,
}

#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub key: char,
    pub description: String,
    // TODO: Add action enum later
}

impl Default for Keybindings {
    fn default() -> Self {
        let mut mode_bindings = HashMap::new();

        mode_bindings.insert(
            Mode::Normal,
            vec![
                KeyBinding {
                    key: 'e',
                    description: "edit".to_string(),
                },
                KeyBinding {
                    key: 'd',
                    description: "delete".to_string(),
                },
                KeyBinding {
                    key: 'f',
                    description: "fork".to_string(),
                },
                KeyBinding {
                    key: 'h',
                    description: "hide".to_string(),
                },
                KeyBinding {
                    key: 'a',
                    description: "append".to_string(),
                },
                KeyBinding {
                    key: '?',
                    description: "help".to_string(),
                },
            ],
        );

        Self { mode_bindings }
    }
}
