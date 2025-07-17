//! Tests for UIAction and ActionRegistry
//!
//! This module tests the action system including UIAction metadata and ActionRegistry functionality.

use frond::actions::{ActionRegistry, UIAction};
use frond::app::Mode;
use uuid::Uuid;

#[test]
fn action_scroll_up_has_correct_metadata() {
    let action = UIAction::ScrollUp;
    let info = action.info();

    assert_eq!(info.id, "scroll_up");
    assert_eq!(info.name, "Scroll Up");
    assert_eq!(info.description, "Scroll up one line");
    assert_eq!(info.available_in_modes, vec![Mode::Normal]);
    assert!(!info.requires_focus);
}

#[test]
fn action_scroll_down_has_correct_metadata() {
    let action = UIAction::ScrollDown;
    let info = action.info();

    assert_eq!(info.id, "scroll_down");
    assert_eq!(info.name, "Scroll Down");
    assert_eq!(info.description, "Scroll down one line");
    assert_eq!(info.available_in_modes, vec![Mode::Normal]);
    assert!(!info.requires_focus);
}

#[test]
fn action_enter_edit_mode_requires_focus() {
    let message_id = Uuid::new_v4();
    let action = UIAction::EnterEditMode(message_id);
    let info = action.info();

    assert_eq!(info.id, "edit_message");
    assert_eq!(info.name, "Edit Message");
    assert_eq!(info.description, "Edit the currently focused message");
    assert_eq!(info.available_in_modes, vec![Mode::Normal]);
    assert!(info.requires_focus);
}

#[test]
fn action_enter_append_mode_does_not_require_focus() {
    let action = UIAction::EnterAppendMode;
    let info = action.info();

    assert_eq!(info.id, "append_message");
    assert_eq!(info.name, "Append Message");
    assert_eq!(info.description, "Add a new message to the conversation");
    assert_eq!(info.available_in_modes, vec![Mode::Normal]);
    assert!(!info.requires_focus);
}

#[test]
fn action_delete_message_requires_focus() {
    let message_id = Uuid::new_v4();
    let action = UIAction::DeleteMessage(message_id);
    let info = action.info();

    assert_eq!(info.id, "delete_message");
    assert_eq!(info.name, "Delete Message");
    assert_eq!(info.description, "Delete the currently focused message");
    assert_eq!(info.available_in_modes, vec![Mode::Normal]);
    assert!(info.requires_focus);
}

#[test]
fn action_registry_creates_with_default_actions() {
    let registry = ActionRegistry::new();

    // Verify that default actions are registered
    assert!(registry.get_action("scroll_up").is_some());
    assert!(registry.get_action("scroll_down").is_some());
    assert!(registry.get_action("edit_message").is_some());
    assert!(registry.get_action("append_message").is_some());
    assert!(registry.get_action("delete_message").is_some());
    assert!(registry.get_action("show_help").is_some());
}

#[test]
fn action_registry_returns_none_for_unknown_action() {
    let registry = ActionRegistry::new();

    assert!(registry.get_action("unknown_action").is_none());
    assert!(registry.get_action("nonexistent").is_none());
    assert!(registry.get_action("").is_none());
}

#[test]
fn action_registry_scroll_up_has_correct_type() {
    let registry = ActionRegistry::new();

    let action = registry.get_action("scroll_up").unwrap();
    match action {
        UIAction::ScrollUp => {} // Expected
        _ => panic!("Expected ScrollUp action, got {:?}", action),
    }
}

#[test]
fn action_registry_filters_actions_for_normal_mode() {
    let registry = ActionRegistry::new();

    let normal_actions = registry.get_actions_for_mode(Mode::Normal);

    // Should contain navigation actions
    assert!(normal_actions.iter().any(|(id, _)| *id == "scroll_up"));
    assert!(normal_actions.iter().any(|(id, _)| *id == "scroll_down"));
    assert!(normal_actions.iter().any(|(id, _)| *id == "next_branch"));
    assert!(normal_actions.iter().any(|(id, _)| *id == "prev_branch"));

    // Should contain message actions
    assert!(normal_actions.iter().any(|(id, _)| *id == "edit_message"));
    assert!(normal_actions.iter().any(|(id, _)| *id == "append_message"));
    assert!(normal_actions.iter().any(|(id, _)| *id == "delete_message"));

    // Should contain utility actions
    assert!(normal_actions.iter().any(|(id, _)| *id == "show_help"));
}

#[test]
fn all_navigation_actions_available_in_normal_mode() {
    let navigation_actions = vec![
        UIAction::ScrollUp,
        UIAction::ScrollDown,
        UIAction::NextBranch,
        UIAction::PrevBranch,
        UIAction::NextTree,
        UIAction::PrevTree,
        UIAction::ScrollToMessage(Uuid::new_v4()),
        UIAction::FocusMessage(Uuid::new_v4()),
    ];

    for action in navigation_actions {
        let info = action.info();
        assert!(
            info.available_in_modes.contains(&Mode::Normal),
            "Navigation action {:?} should be available in Normal mode",
            action
        );
    }
}

#[test]
fn all_message_operations_require_focus() {
    let message_id = Uuid::new_v4();
    let focus_required_actions = vec![
        UIAction::EnterEditMode(message_id),
        UIAction::DeleteMessage(message_id),
        UIAction::ForkBranch(message_id),
        UIAction::HideMessage(message_id),
    ];

    for action in focus_required_actions {
        let info = action.info();
        assert!(
            info.requires_focus,
            "Message operation {:?} should require focus",
            action
        );
    }
}

#[test]
fn actions_are_cloneable() {
    let actions = vec![
        UIAction::ScrollUp,
        UIAction::ScrollDown,
        UIAction::EnterEditMode(Uuid::new_v4()),
        UIAction::EnterAppendMode,
        UIAction::DeleteMessage(Uuid::new_v4()),
        UIAction::NextBranch,
        UIAction::PrevBranch,
        UIAction::ShowHelp,
    ];

    for action in actions {
        let _cloned = action.clone();
        // If this compiles, the test passes
    }
}

#[test]
fn actions_are_debuggable() {
    let action = UIAction::EnterEditMode(Uuid::new_v4());
    let debug_str = format!("{:?}", action);
    assert!(debug_str.contains("EnterEditMode"));
}
