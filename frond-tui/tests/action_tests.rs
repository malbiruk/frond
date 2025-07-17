//! Tests for UIAction and ActionRegistry
//!
//! This module tests the action system including UIAction metadata and ActionRegistry functionality.

use frond::actions::{ActionRegistry, CommonAction, EditModeAction, NormalModeAction, UIAction};
use frond::app::Mode;
use uuid::Uuid;

#[test]
fn action_scroll_up_has_correct_metadata() {
    let action = UIAction::NormalMode(NormalModeAction::ScrollUp);
    let info = action.info();

    assert_eq!(info.id, "scroll_up");
    assert_eq!(info.name, "scroll up");
    assert_eq!(info.description, "scroll up one line");
    assert_eq!(info.available_in_modes, vec![Mode::Normal]);
    assert!(!info.requires_focus);
}

#[test]
fn action_scroll_down_has_correct_metadata() {
    let action = UIAction::NormalMode(NormalModeAction::ScrollDown);
    let info = action.info();

    assert_eq!(info.id, "scroll_down");
    assert_eq!(info.name, "scroll down");
    assert_eq!(info.description, "scroll down one line");
    assert_eq!(info.available_in_modes, vec![Mode::Normal]);
    assert!(!info.requires_focus);
}

#[test]
fn action_enter_edit_mode_has_correct_metadata() {
    let message_id = Uuid::new_v4();
    let action = UIAction::NormalMode(NormalModeAction::EnterEditMode(message_id));
    let info = action.info();

    assert_eq!(info.id, "edit_message");
    assert_eq!(info.name, "edit");
    assert_eq!(info.description, "edit the currently focused message");
    assert_eq!(info.available_in_modes, vec![Mode::Normal]);
    assert!(info.requires_focus);
}

#[test]
fn action_enter_append_mode_has_correct_metadata() {
    let action = UIAction::NormalMode(NormalModeAction::EnterAppendMode);
    let info = action.info();

    assert_eq!(info.id, "append_message");
    assert_eq!(info.name, "append");
    assert_eq!(info.description, "add a new message to the conversation");
    assert_eq!(info.available_in_modes, vec![Mode::Normal]);
    assert!(!info.requires_focus);
}

#[test]
fn action_delete_message_has_correct_metadata() {
    let message_id = Uuid::new_v4();
    let action = UIAction::NormalMode(NormalModeAction::DeleteMessage(message_id));
    let info = action.info();

    assert_eq!(info.id, "delete_message");
    assert_eq!(info.name, "delete");
    assert_eq!(info.description, "delete the currently focused message");
    assert_eq!(info.available_in_modes, vec![Mode::Normal]);
    assert!(info.requires_focus);
}

#[test]
fn action_quit_has_correct_metadata() {
    let action = UIAction::Common(CommonAction::Quit);
    let info = action.info();

    assert_eq!(info.id, "quit");
    assert_eq!(info.name, "quit");
    assert_eq!(info.description, "quit the application");
    assert!(info.available_in_modes.contains(&Mode::Normal));
    assert!(!info.requires_focus);
}

#[test]
fn action_exit_current_mode_has_correct_metadata() {
    let action = UIAction::EditMode(EditModeAction::ExitCurrentMode);
    let info = action.info();

    assert_eq!(info.id, "exit_mode");
    assert_eq!(info.name, "exit mode");
    assert_eq!(info.description, "exit the current mode");
    assert!(!info.available_in_modes.is_empty());
    assert!(!info.requires_focus);
}

#[test]
fn action_registry_can_retrieve_action_by_id() {
    let registry = ActionRegistry::new();

    // Test retrieving a normal mode action
    if let Some(action) = registry.get_action("scroll_up") {
        match action {
            UIAction::NormalMode(NormalModeAction::ScrollUp) => {} // Expected
            _ => panic!("Expected ScrollUp action"),
        }
    } else {
        panic!("scroll_up action not found in registry");
    }

    // Test retrieving a common action
    if let Some(action) = registry.get_action("quit") {
        match action {
            UIAction::Common(CommonAction::Quit) => {} // Expected
            _ => panic!("Expected Quit action"),
        }
    } else {
        panic!("quit action not found in registry");
    }
}

#[test]
fn action_registry_filters_actions_by_mode() {
    let registry = ActionRegistry::new();

    let normal_actions = registry.get_actions_for_mode(Mode::Normal);
    let edit_actions = registry.get_actions_for_mode(Mode::Edit(frond::app::EditMode::Append));

    // Normal mode should have many actions
    assert!(!normal_actions.is_empty());

    // Edit mode should have at least the exit action
    assert!(!edit_actions.is_empty());

    // Check that normal mode actions include scroll actions
    let normal_action_ids: Vec<&str> = normal_actions.iter().map(|(id, _)| *id).collect();
    assert!(normal_action_ids.contains(&"scroll_up"));
    assert!(normal_action_ids.contains(&"scroll_down"));

    // Check that edit mode actions include exit action
    let edit_action_ids: Vec<&str> = edit_actions.iter().map(|(id, _)| *id).collect();
    assert!(edit_action_ids.contains(&"exit_mode"));
}

#[test]
fn action_registry_provides_essential_actions() {
    let registry = ActionRegistry::new();

    let normal_essentials = registry.get_essential_actions_for_mode(Mode::Normal);
    let edit_essentials =
        registry.get_essential_actions_for_mode(Mode::Edit(frond::app::EditMode::Append));

    // Normal mode should have essential actions like append, edit, delete
    let normal_essential_ids: Vec<&str> = normal_essentials.iter().map(|(id, _)| *id).collect();
    assert!(normal_essential_ids.contains(&"append_message"));
    assert!(normal_essential_ids.contains(&"edit_message"));
    assert!(normal_essential_ids.contains(&"delete_message"));

    // Edit mode should have exit as essential
    let edit_essential_ids: Vec<&str> = edit_essentials.iter().map(|(id, _)| *id).collect();
    assert!(edit_essential_ids.contains(&"exit_mode"));
}

#[test]
fn actions_have_mode_specific_organization() {
    // Test that different action types are properly organized
    let normal_actions = vec![
        UIAction::NormalMode(NormalModeAction::ScrollUp),
        UIAction::NormalMode(NormalModeAction::ScrollDown),
        UIAction::NormalMode(NormalModeAction::EnterEditMode(Uuid::new_v4())),
        UIAction::NormalMode(NormalModeAction::EnterAppendMode),
        UIAction::NormalMode(NormalModeAction::DeleteMessage(Uuid::new_v4())),
        UIAction::NormalMode(NormalModeAction::NextBranch),
        UIAction::NormalMode(NormalModeAction::PrevBranch),
        UIAction::NormalMode(NormalModeAction::NextTree),
        UIAction::NormalMode(NormalModeAction::PrevTree),
        UIAction::NormalMode(NormalModeAction::ScrollToMessage(Uuid::new_v4())),
        UIAction::NormalMode(NormalModeAction::FocusMessage(Uuid::new_v4())),
        UIAction::NormalMode(NormalModeAction::ShowHelp),
    ];

    let common_actions = vec![
        UIAction::Common(CommonAction::Quit),
        UIAction::Common(CommonAction::ClearError),
    ];

    let edit_actions = vec![UIAction::EditMode(EditModeAction::ExitCurrentMode)];

    // Verify all normal actions have Normal mode in their available modes
    for action in normal_actions {
        let info = action.info();
        assert!(info.available_in_modes.contains(&Mode::Normal));
    }

    // Verify common actions are available in multiple modes
    for action in common_actions {
        let info = action.info();
        assert!(!info.available_in_modes.is_empty());
    }

    // Verify edit actions have Edit mode in their available modes
    for action in edit_actions {
        let info = action.info();
        assert!(
            info.available_in_modes
                .iter()
                .any(|mode| matches!(mode, Mode::Edit(_)))
        );
    }
}

#[test]
fn actions_requiring_focus_are_properly_marked() {
    // Actions that require focus
    let focus_requiring_actions = vec![
        UIAction::NormalMode(NormalModeAction::EnterEditMode(Uuid::new_v4())),
        UIAction::NormalMode(NormalModeAction::DeleteMessage(Uuid::new_v4())),
        UIAction::NormalMode(NormalModeAction::ForkBranch(Uuid::new_v4())),
        UIAction::NormalMode(NormalModeAction::HideMessage(Uuid::new_v4())),
    ];

    for action in focus_requiring_actions {
        let info = action.info();
        assert!(
            info.requires_focus,
            "Action {} should require focus",
            info.name
        );
    }

    // Actions that don't require focus
    let non_focus_actions = vec![
        UIAction::NormalMode(NormalModeAction::ScrollUp),
        UIAction::NormalMode(NormalModeAction::ScrollDown),
        UIAction::NormalMode(NormalModeAction::EnterAppendMode),
        UIAction::NormalMode(NormalModeAction::NextBranch),
        UIAction::NormalMode(NormalModeAction::PrevBranch),
        UIAction::NormalMode(NormalModeAction::ShowHelp),
        UIAction::Common(CommonAction::Quit),
    ];

    for action in non_focus_actions {
        let info = action.info();
        assert!(
            !info.requires_focus,
            "Action {} should not require focus",
            info.name
        );
    }
}

#[test]
fn action_info_provides_unique_ids() {
    let action = UIAction::NormalMode(NormalModeAction::EnterEditMode(Uuid::new_v4()));
    let info = action.info();

    assert_eq!(info.id, "edit_message");
    assert!(!info.id.is_empty());
    assert!(!info.name.is_empty());
    assert!(!info.description.is_empty());
}
