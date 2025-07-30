use crate::app::Mode;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub mod common;
pub mod edit_mode;
pub mod macros;
pub mod normal_mode;

pub use common::{CommonAction, CommonActionSchema};
pub use edit_mode::{EditModeAction, EditModeActionSchema};
pub use normal_mode::{NormalModeAction, NormalModeActionSchema};

// Schema enums - parameter-less for registry
#[derive(Debug, Clone)]
pub enum ActionSchema {
    Common(CommonActionSchema),
    Normal(NormalModeActionSchema),
    Edit(EditModeActionSchema),
}

impl ActionSchema {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Common(action) => action.id(),
            Self::Normal(action) => action.id(),
            Self::Edit(action) => action.id(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Common(action) => action.name(),
            Self::Normal(action) => action.name(),
            Self::Edit(action) => action.name(),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Common(action) => action.description(),
            Self::Normal(action) => action.description(),
            Self::Edit(action) => action.description(),
        }
    }

    pub fn requires_focus(&self) -> bool {
        match self {
            Self::Common(action) => action.requires_focus(),
            Self::Normal(action) => action.requires_focus(),
            Self::Edit(action) => action.requires_focus(),
        }
    }
}

// Action instances - with real parameters for dispatch
#[derive(Debug, Clone)]
pub enum UIAction {
    Common(CommonAction),
    NormalMode(NormalModeAction),
    EditMode(EditModeAction),
}

pub struct ActionRegistry {
    schemas: HashMap<&'static str, ActionSchema>,
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionRegistry {
    pub fn new() -> Self {
        let mut schemas = HashMap::new();

        // Register all common action schemas
        for schema in CommonActionSchema::iter() {
            let action_schema = ActionSchema::Common(schema);
            schemas.insert(action_schema.id(), action_schema);
        }

        // Register all normal mode action schemas
        for schema in NormalModeActionSchema::iter() {
            let action_schema = ActionSchema::Normal(schema);
            schemas.insert(action_schema.id(), action_schema);
        }

        // Register all edit mode action schemas
        for schema in EditModeActionSchema::iter() {
            let action_schema = ActionSchema::Edit(schema);
            schemas.insert(action_schema.id(), action_schema);
        }

        Self { schemas }
    }

    pub fn get_schema(&self, id: &str) -> Option<&ActionSchema> {
        self.schemas.get(id)
    }

    pub fn get_schemas_for_mode(&self, mode: Mode) -> Vec<(&'static str, &ActionSchema)> {
        self.schemas
            .iter()
            .filter(|(_, schema)| schema_available_in_mode(schema, mode))
            .map(|(id, schema)| (*id, schema))
            .collect()
    }

    pub fn get_essential_schemas_for_mode(&self, mode: Mode) -> Vec<(&'static str, &ActionSchema)> {
        let essential_order = match mode {
            Mode::Normal => vec![
                "append_message",
                "edit_message",
                "hide_message",
                "fork_branch",
                "delete_message",
                "show_help",
            ],
            Mode::Edit => vec!["show_help", "exit_mode"],
        };

        essential_order
            .into_iter()
            .filter_map(|action_id| {
                let schema = self.schemas.get(action_id)?;
                if schema_available_in_mode(schema, mode) {
                    Some((action_id, schema))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn schema_available_in_mode(schema: &ActionSchema, mode: Mode) -> bool {
    match (schema, mode) {
        // Common actions available in all modes
        (ActionSchema::Common(_), _) => true,
        // Normal mode actions only in normal mode
        (ActionSchema::Normal(_), Mode::Normal) => true,
        // Edit mode actions only in edit mode
        (ActionSchema::Edit(_), Mode::Edit) => true,
        _ => false,
    }
}

pub trait ActionDispatcher {
    fn dispatch(&mut self, action: UIAction);
}
