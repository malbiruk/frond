//! Simple tests for the refactored action system
//!
//! This module tests the basic functionality of the new schema-based action system.

use frond::actions::{
    ActionRegistry, ActionSchema, CommonActionSchema, EditModeActionSchema, NormalModeActionSchema,
};
use frond::app::Mode;
use strum::IntoEnumIterator;

#[test]
fn action_registry_creates_successfully() {
    let registry = ActionRegistry::new();

    // Should be able to get some schemas
    assert!(registry.get_schema("scroll_up").is_some());
    assert!(registry.get_schema("quit").is_some());
    assert!(registry.get_schema("exit_mode").is_some());
}

#[test]
fn schemas_have_correct_metadata() {
    let scroll_up = NormalModeActionSchema::ScrollUp;
    assert_eq!(scroll_up.id(), "scroll_up");
    assert_eq!(scroll_up.name(), "up");
    assert_eq!(scroll_up.description(), "scroll up one line");
    assert!(!scroll_up.requires_focus());

    let edit_message = NormalModeActionSchema::EnterEditMode;
    assert_eq!(edit_message.id(), "edit_message");
    assert_eq!(edit_message.name(), "edit");
    assert!(edit_message.requires_focus());

    let quit = CommonActionSchema::Quit;
    assert_eq!(quit.id(), "quit");
    assert_eq!(quit.name(), "quit");
    assert!(!quit.requires_focus());
}

#[test]
fn all_schemas_are_registered() {
    let registry = ActionRegistry::new();

    // Check that all common action schemas are registered
    for schema in CommonActionSchema::iter() {
        let action_schema = ActionSchema::Common(schema);
        assert!(
            registry.get_schema(action_schema.id()).is_some(),
            "Schema {} not registered",
            action_schema.id()
        );
    }

    // Check that all normal mode schemas are registered
    for schema in NormalModeActionSchema::iter() {
        let action_schema = ActionSchema::Normal(schema);
        assert!(
            registry.get_schema(action_schema.id()).is_some(),
            "Schema {} not registered",
            action_schema.id()
        );
    }

    // Check that all edit mode schemas are registered
    for schema in EditModeActionSchema::iter() {
        let action_schema = ActionSchema::Edit(schema);
        assert!(
            registry.get_schema(action_schema.id()).is_some(),
            "Schema {} not registered",
            action_schema.id()
        );
    }
}

#[test]
fn schemas_are_available_in_correct_modes() {
    let registry = ActionRegistry::new();

    let normal_schemas = registry.get_schemas_for_mode(Mode::Normal);
    let normal_ids: Vec<&str> = normal_schemas.iter().map(|(id, _)| *id).collect();

    // Should include common and normal mode actions
    assert!(normal_ids.contains(&"quit"));
    assert!(normal_ids.contains(&"scroll_up"));
    assert!(normal_ids.contains(&"edit_message"));

    // Should not include edit mode specific actions
    assert!(!normal_ids.contains(&"exit_mode"));
}

#[test]
fn essential_schemas_are_provided() {
    let registry = ActionRegistry::new();

    let normal_essentials = registry.get_essential_schemas_for_mode(Mode::Normal);
    let essential_ids: Vec<&str> = normal_essentials.iter().map(|(id, _)| *id).collect();

    // Should include key actions for normal mode
    assert!(essential_ids.contains(&"append_message"));
    assert!(essential_ids.contains(&"edit_message"));
    assert!(essential_ids.contains(&"delete_message"));
}
