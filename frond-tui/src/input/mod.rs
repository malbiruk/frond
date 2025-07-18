use crate::actions::{
    ActionRegistry, ActionSchema, CommonAction, CursorDirection, EditModeAction, NormalModeAction,
    UIAction,
};
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
        // First try to get action from keybinding
        if let Some(action_id) = self.key_mapping.get_action_id(current_mode, key_event) {
            if let Some(schema) = self.action_registry.get_schema(&action_id) {
                return self.schema_to_action(schema, focused_message, key_event);
            }
        }

        // If no keybinding found, handle special cases for edit mode
        if let Mode::Edit(_) = current_mode {
            return self.handle_edit_mode_direct_input(key_event);
        }

        None
    }

    fn schema_to_action(
        &self,
        schema: &ActionSchema,
        focused_message: Option<Uuid>,
        key_event: KeyEvent,
    ) -> Option<UIAction> {
        match schema {
            ActionSchema::Common(common_schema) => {
                use crate::actions::CommonActionSchema;
                let action = match common_schema {
                    CommonActionSchema::Quit => CommonAction::Quit,
                    CommonActionSchema::ClearError => CommonAction::ClearError,
                    CommonActionSchema::ShowHelp => CommonAction::ShowHelp(Mode::Normal), // Default mode
                    CommonActionSchema::ShowError => return None, // Can't create without error message
                    CommonActionSchema::UpdateConfig => return None, // Can't create without config
                };
                Some(UIAction::Common(action))
            }
            ActionSchema::Normal(normal_schema) => {
                use crate::actions::NormalModeActionSchema;
                let action = match normal_schema {
                    NormalModeActionSchema::ScrollUp => NormalModeAction::ScrollUp,
                    NormalModeActionSchema::ScrollDown => NormalModeAction::ScrollDown,
                    NormalModeActionSchema::EnterAppendMode => NormalModeAction::EnterAppendMode,
                    NormalModeActionSchema::EnterCommandPalette => {
                        NormalModeAction::EnterCommandPalette
                    }
                    NormalModeActionSchema::NextBranch => NormalModeAction::NextBranch,
                    NormalModeActionSchema::PrevBranch => NormalModeAction::PrevBranch,
                    NormalModeActionSchema::NextTree => NormalModeAction::NextTree,
                    NormalModeActionSchema::PrevTree => NormalModeAction::PrevTree,

                    // Actions that require focused message
                    NormalModeActionSchema::EnterEditMode => {
                        let message_id = focused_message?;
                        NormalModeAction::EnterEditMode(message_id)
                    }
                    NormalModeActionSchema::DeleteMessage => {
                        let message_id = focused_message?;
                        NormalModeAction::DeleteMessage(message_id)
                    }
                    NormalModeActionSchema::ForkBranch => {
                        let message_id = focused_message?;
                        NormalModeAction::ForkBranch(message_id)
                    }
                    NormalModeActionSchema::HideMessage => {
                        let message_id = focused_message?;
                        NormalModeAction::HideMessage(message_id)
                    }
                    NormalModeActionSchema::ShowMessage => {
                        let message_id = focused_message?;
                        NormalModeAction::ShowMessage(message_id)
                    }
                };
                Some(UIAction::NormalMode(action))
            }
            ActionSchema::Edit(edit_schema) => {
                use crate::actions::EditModeActionSchema;
                let action = match edit_schema {
                    EditModeActionSchema::ExitCurrentMode => EditModeAction::ExitCurrentMode,
                    EditModeActionSchema::DeleteChar => EditModeAction::DeleteChar,
                    EditModeActionSchema::MoveCursorLeft => {
                        EditModeAction::MoveCursor(CursorDirection::Left)
                    }
                    EditModeActionSchema::MoveCursorRight => {
                        EditModeAction::MoveCursor(CursorDirection::Right)
                    }
                    EditModeActionSchema::MoveCursorUp => {
                        EditModeAction::MoveCursor(CursorDirection::Up)
                    }
                    EditModeActionSchema::MoveCursorDown => {
                        EditModeAction::MoveCursor(CursorDirection::Down)
                    }
                    EditModeActionSchema::SelectAll => EditModeAction::SelectAll,
                    EditModeActionSchema::Cut => EditModeAction::Cut,
                    EditModeActionSchema::Copy => EditModeAction::Copy,

                    // Actions that need parameters from key event
                    EditModeActionSchema::InsertChar => {
                        if let KeyCode::Char(c) = key_event.code {
                            EditModeAction::InsertChar(c)
                        } else {
                            return None;
                        }
                    }
                    EditModeActionSchema::Paste => return None, // Needs clipboard content
                };
                Some(UIAction::EditMode(action))
            }
        }
    }

    fn handle_edit_mode_direct_input(&self, key_event: KeyEvent) -> Option<UIAction> {
        // Handle direct character input in edit mode (not bound to actions)
        match key_event.code {
            KeyCode::Char(c) if key_event.modifiers.is_empty() => {
                Some(UIAction::EditMode(EditModeAction::InsertChar(c)))
            }
            _ => None,
        }
    }

    pub fn get_essential_schemas(&self, mode: Mode) -> Vec<(&'static str, &ActionSchema)> {
        self.action_registry.get_essential_schemas_for_mode(mode)
    }
}
