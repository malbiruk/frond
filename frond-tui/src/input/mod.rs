use crate::actions::{ActionRegistry, NormalModeAction, UIAction};
use crate::app::Mode;
use crate::config::Config;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;
use uuid::Uuid;

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

    pub fn handle_input(
        &self,
        key_event: KeyEvent,
        current_mode: Mode,
        focused_message: Option<Uuid>,
    ) -> Option<UIAction> {
        let action_id = self.key_mapping.get_action_id(current_mode, key_event)?;
        let mut action = self.action_registry.get_action(&action_id)?.clone();

        action = self.resolve_context_dependent_action(action, focused_message);

        Some(action)
    }

    fn resolve_context_dependent_action(
        &self,
        action: UIAction,
        focused_message: Option<Uuid>,
    ) -> UIAction {
        match action {
            UIAction::NormalMode(NormalModeAction::EnterEditMode(id)) if id == Uuid::nil() => {
                UIAction::NormalMode(NormalModeAction::EnterEditMode(
                    focused_message.unwrap_or(Uuid::nil()),
                ))
            }
            UIAction::NormalMode(NormalModeAction::DeleteMessage(id)) if id == Uuid::nil() => {
                UIAction::NormalMode(NormalModeAction::DeleteMessage(
                    focused_message.unwrap_or(Uuid::nil()),
                ))
            }
            UIAction::NormalMode(NormalModeAction::ForkBranch(id)) if id == Uuid::nil() => {
                UIAction::NormalMode(NormalModeAction::ForkBranch(
                    focused_message.unwrap_or(Uuid::nil()),
                ))
            }
            UIAction::NormalMode(NormalModeAction::HideMessage(id)) if id == Uuid::nil() => {
                UIAction::NormalMode(NormalModeAction::HideMessage(
                    focused_message.unwrap_or(Uuid::nil()),
                ))
            }
            other => other,
        }
    }

    pub fn get_essential_actions(&self, mode: Mode) -> Vec<(&'static str, &UIAction)> {
        self.action_registry.get_essential_actions_for_mode(mode)
    }
}
