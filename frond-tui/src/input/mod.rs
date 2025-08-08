use crate::actions::CommonActionSchema;
use crate::actions::{ActionRegistry, ActionSchema, CommonAction, UIAction};
use crate::app::Mode;
use crate::config::Config;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct KeyChord {
    pub key: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyChord {
    pub fn new(key: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { key, modifiers }
    }

    pub fn char(c: char) -> Self {
        Self::new(KeyCode::Char(c), KeyModifiers::NONE)
    }

    pub fn ctrl(c: char) -> Self {
        Self::new(KeyCode::Char(c), KeyModifiers::CONTROL)
    }

    pub fn shift(c: char) -> Self {
        Self::new(KeyCode::Char(c), KeyModifiers::SHIFT)
    }

    pub fn alt(c: char) -> Self {
        Self::new(KeyCode::Char(c), KeyModifiers::ALT)
    }
}

#[derive(Debug, Clone)]
pub struct KeyMapping {
    // Key -> Action ID mapping per mode
    pub mode_mappings: HashMap<Mode, HashMap<KeyChord, String>>,
}

impl KeyMapping {
    pub fn from_config(config: &Config) -> Self {
        let mut mode_mappings = HashMap::new();

        // Build reverse mapping: KeyChord -> Action ID
        for (mode, action_bindings) in &config.keybindings.mode_bindings {
            let mut key_to_action = HashMap::new();
            for (action_id, key_chord) in action_bindings {
                key_to_action.insert(key_chord.clone(), action_id.clone());
            }
            mode_mappings.insert(*mode, key_to_action);
        }

        Self { mode_mappings }
    }

    pub fn get_action_id(&self, mode: Mode, key_event: KeyEvent) -> Option<String> {
        let chord = KeyChord::new(key_event.code, key_event.modifiers);
        self.mode_mappings.get(&mode)?.get(&chord).cloned()
    }
}

pub struct InputHandler {
    key_mapping: KeyMapping,
    action_registry: ActionRegistry,
}

impl InputHandler {
    pub fn new(config: &Config) -> Self {
        Self {
            key_mapping: KeyMapping::from_config(config),
            action_registry: ActionRegistry::new(),
        }
    }

    pub fn handle_input(&self, key_event: KeyEvent, current_mode: Mode) -> Option<UIAction> {
        if let Some(action_id) = self.key_mapping.get_action_id(current_mode, key_event) {
            if let Some(schema) = self.action_registry.get_schema(&action_id) {
                return self.schema_to_action(schema, &current_mode);
            }
        }

        None
    }
    
    /// Check if a key event should be handled as raw input in the current mode
    pub fn should_handle_as_raw_input(&self, key_event: KeyEvent, current_mode: Mode) -> bool {
        // In edit mode, unmapped keys are raw text input
        current_mode == Mode::Edit && 
            self.key_mapping.get_action_id(current_mode, key_event).is_none()
    }

    fn schema_to_action(&self, schema: &ActionSchema, current_mode: &Mode) -> Option<UIAction> {
        match schema {
            ActionSchema::Common(common_schema) => {
                let action = match common_schema {
                    CommonActionSchema::Quit => CommonAction::Quit,
                    CommonActionSchema::ShowHelp => CommonAction::ShowHelp(*current_mode),
                };
                Some(UIAction::Common(action))
            }
            ActionSchema::Normal(normal_schema) => {
                let action = normal_schema.to_action();
                Some(UIAction::NormalMode(action))
            }
            ActionSchema::Edit(edit_schema) => {
                let action = edit_schema.to_action();
                Some(UIAction::EditMode(action))
            }
        }
    }

    pub fn get_essential_schemas(&self, mode: Mode) -> Vec<(&'static str, &ActionSchema)> {
        self.action_registry.get_essential_schemas_for_mode(mode)
    }
}
