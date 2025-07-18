//! Tests for action system functionality
//!
//! These tests verify that the action system correctly provides discoverable actions
//! and that actions can be resolved from user input.

use frond::actions::{ActionRegistry, UIAction};
use frond::app::{EditMode, Mode};
use frond::config::Config;
use frond::input::InputHandler;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use uuid::Uuid;

// === Core Functionality Tests ===

#[test]
fn action_registry_provides_actions_for_normal_mode() {
    let registry = ActionRegistry::new();
    let actions = registry.get_schemas_for_mode(Mode::Normal);

    // Should provide basic editing actions
    let action_ids: Vec<&str> = actions.iter().map(|(id, _)| *id).collect();
    assert!(action_ids.contains(&"edit_message"));
    assert!(action_ids.contains(&"delete_message"));
    assert!(action_ids.contains(&"append_message"));
    assert!(action_ids.contains(&"quit"));
}

#[test]
fn action_registry_provides_actions_for_edit_mode() {
    let registry = ActionRegistry::new();
    let actions = registry.get_schemas_for_mode(Mode::Edit(EditMode::Append));

    let action_ids: Vec<&str> = actions.iter().map(|(id, _)| *id).collect();
    assert!(action_ids.contains(&"exit_mode"));
    assert!(action_ids.contains(&"quit"));
}

#[test]
fn action_registry_provides_essential_actions() {
    let registry = ActionRegistry::new();

    let normal_essentials = registry.get_essential_schemas_for_mode(Mode::Normal);
    assert!(!normal_essentials.is_empty());

    let edit_essentials = registry.get_essential_schemas_for_mode(Mode::Edit(EditMode::Append));
    assert!(!edit_essentials.is_empty());
}

#[test]
fn input_handler_produces_quit_action() {
    let config = Config::default();
    let handler = InputHandler::new(&config);

    let quit_key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
    let action = handler.handle_input(quit_key, Mode::Normal, None);

    // Should produce some form of quit action
    assert!(action.is_some());
    match action.unwrap() {
        UIAction::Common(_) => {} // Expected quit to be common action
        _ => panic!("Quit should be a common action"),
    }
}

#[test]
fn input_handler_produces_edit_action_with_focus() {
    let config = Config::default();
    let handler = InputHandler::new(&config);
    let message_id = Uuid::new_v4();

    let edit_key = KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE);
    let action = handler.handle_input(edit_key, Mode::Normal, Some(message_id));

    assert!(action.is_some());
    // Should produce an edit action that knows about the message
    match action.unwrap() {
        UIAction::NormalMode(_) => {} // Expected to be normal mode action
        _ => panic!("Edit should be a normal mode action"),
    }
}

#[test]
fn input_handler_handles_edit_action_without_focus() {
    let config = Config::default();
    let handler = InputHandler::new(&config);

    let edit_key = KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE);
    let action = handler.handle_input(edit_key, Mode::Normal, None);

    // Edit action requires focus, so should return None without focus
    assert!(
        action.is_none(),
        "Edit action should not work without focus"
    );
}

#[test]
fn input_handler_produces_exit_action_in_edit_mode() {
    let config = Config::default();
    let handler = InputHandler::new(&config);

    let escape_key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    let action = handler.handle_input(escape_key, Mode::Edit(EditMode::Append), None);

    assert!(action.is_some());
    match action.unwrap() {
        UIAction::EditMode(_) => {} // Expected to be edit mode action
        _ => panic!("Exit should be an edit mode action"),
    }
}

#[test]
fn input_handler_handles_unmapped_keys_gracefully() {
    let config = Config::default();
    let handler = InputHandler::new(&config);

    let unmapped_key = KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE);
    let action = handler.handle_input(unmapped_key, Mode::Normal, None);

    // Should handle gracefully (return None)
    assert!(action.is_none());
}

#[test]
fn input_handler_supports_navigation_keys() {
    let config = Config::default();
    let handler = InputHandler::new(&config);

    let navigation_keys = vec![KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right];

    for key in navigation_keys {
        let event = KeyEvent::new(key, KeyModifiers::NONE);
        let action = handler.handle_input(event, Mode::Normal, None);

        // Should produce some navigation action
        assert!(
            action.is_some(),
            "Navigation key {:?} should produce an action",
            key
        );
    }
}

// === Behavioral Tests ===

#[test]
fn action_registry_consistent_between_calls() {
    let registry1 = ActionRegistry::new();
    let registry2 = ActionRegistry::new();

    let actions1 = registry1.get_schemas_for_mode(Mode::Normal);
    let actions2 = registry2.get_schemas_for_mode(Mode::Normal);

    // Should be deterministic
    assert_eq!(actions1.len(), actions2.len());
}

#[test]
fn action_registry_mode_isolation() {
    let registry = ActionRegistry::new();

    let normal_actions = registry.get_schemas_for_mode(Mode::Normal);
    let edit_actions = registry.get_schemas_for_mode(Mode::Edit(EditMode::Append));

    let normal_ids: Vec<&str> = normal_actions.iter().map(|(id, _)| *id).collect();
    let edit_ids: Vec<&str> = edit_actions.iter().map(|(id, _)| *id).collect();

    // Edit mode should not have normal mode specific actions
    assert!(!edit_ids.contains(&"edit_message"));
    assert!(!edit_ids.contains(&"next_branch"));

    // Both should have common actions
    assert!(normal_ids.contains(&"quit"));
    assert!(edit_ids.contains(&"quit"));
}

#[test]
fn input_handler_context_sensitivity() {
    let config = Config::default();
    let handler = InputHandler::new(&config);
    let message_id = Uuid::new_v4();

    // Same key, different contexts should potentially produce different actions
    let key = KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE);

    let action_with_focus = handler.handle_input(key, Mode::Normal, Some(message_id));
    let action_without_focus = handler.handle_input(key, Mode::Normal, None);
    let _action_in_edit_mode = handler.handle_input(key, Mode::Edit(EditMode::Append), None);

    // With focus should produce action, without focus should not (edit requires focus)
    assert!(action_with_focus.is_some());
    assert!(action_without_focus.is_none());

    // Edit mode might not have this mapping
    // (This tests that context matters)
}

#[test]
fn essential_actions_are_actually_provided() {
    let registry = ActionRegistry::new();

    let essentials = registry.get_essential_schemas_for_mode(Mode::Normal);
    let all_actions = registry.get_schemas_for_mode(Mode::Normal);

    let all_action_ids: Vec<&str> = all_actions.iter().map(|(id, _)| *id).collect();

    // All essential actions should be in the full action list
    for (essential_id, _) in essentials {
        assert!(
            all_action_ids.contains(&essential_id),
            "Essential action {} not found in mode actions",
            essential_id
        );
    }
}

#[test]
fn actions_have_meaningful_identifiers() {
    let registry = ActionRegistry::new();
    let actions = registry.get_schemas_for_mode(Mode::Normal);

    for (id, schema) in actions {
        // IDs should be non-empty and descriptive
        assert!(!id.is_empty());
        assert!(!schema.name().is_empty());
        assert!(!schema.description().is_empty());

        // IDs should be consistent format (lowercase with underscores)
        assert!(
            id.chars()
                .all(|c| c.is_lowercase() || c == '_' || c.is_ascii_digit())
        );
    }
}
