//! Tests for reducer logic and business logic
//!
//! This module provides comprehensive test coverage for the reducer system, which handles
//! the core business logic of the application including mode transitions, navigation,
//! content operations, and error handling.

use frond::actions::{CommonAction, EditModeAction, NormalModeAction, UIAction};
use frond::app::state::ModelInfo;
use frond::app::{AppState, Mode, reducer};
use frond::config::Config;
use frond_core::{Branch, Dialogue, Message, Role, Tree};

fn create_test_dialogue_with_multiple_branches() -> Dialogue {
    let mut dialogue = Dialogue::new("Multi-Branch Test");
    let mut tree = Tree::new("Test Tree");

    // Create first branch
    let mut branch1 = Branch::new("main");
    branch1.add_message(Message::new("First message in main", Role::User));
    branch1.add_message(Message::new("Second message in main", Role::Assistant));
    tree.add_branch(branch1);

    // Create second branch
    let mut branch2 = Branch::new("alternative");
    branch2.add_message(Message::new("First message in alt", Role::User));
    tree.add_branch(branch2);

    dialogue.add_tree(tree);
    dialogue
}

fn create_test_dialogue_with_multiple_trees() -> Dialogue {
    let mut dialogue = Dialogue::new("Multi-Tree Test");

    // First tree
    let mut tree1 = Tree::new("Tree One");
    let mut branch1 = Branch::new("main");
    branch1.add_message(Message::new("Message in tree one", Role::User));
    tree1.add_branch(branch1);
    dialogue.add_tree(tree1);

    // Second tree
    let mut tree2 = Tree::new("Tree Two");
    let mut branch2 = Branch::new("main");
    branch2.add_message(Message::new("Message in tree two", Role::User));
    tree2.add_branch(branch2);
    dialogue.add_tree(tree2);

    dialogue
}

fn create_app_state_with_dialogue(dialogue: Dialogue) -> AppState {
    let tree_id = dialogue.trees().get(0).map(|t| t.id());
    let branch_id = tree_id.and_then(|tid| {
        dialogue
            .get_tree_by_id(tid)
            .and_then(|t| t.branches().get(0))
            .map(|b| b.id())
    });
    let message_id = branch_id.and_then(|bid| {
        dialogue
            .get_branch_by_id(bid)
            .and_then(|b| b.messages().get(0))
            .map(|m| m.id())
    });

    AppState {
        dialogue,
        mode: Mode::Normal,
        config: Config::default(),
        model_info: ModelInfo::default(),
        current_tree_id: tree_id,
        current_branch_id: branch_id,
        focused_message_id: message_id,
        scroll_offset: 0,
        scrollbar_state: ratatui::widgets::ScrollbarState::default(),
        pending_scrolling_request: None,
        error_message: None,
        edit_textarea: None,
        highlight_cache: std::collections::HashMap::new(),
    }
}

// Mode Transition Tests

#[test]
fn enter_edit_mode_with_valid_message_transitions_correctly() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    let message_id = state.focused_message_id.unwrap();
    let action = UIAction::NormalMode(NormalModeAction::EnterEditMode);

    reducer::reduce(&mut state, action);

    match state.mode {
        Mode::Edit => {
            // Edit mode should be active and focused message should match
            assert_eq!(state.focused_message_id, Some(message_id));
        }
        _ => panic!("Expected Edit mode"),
    }

    assert!(state.error_message.is_none());
}

#[test]
fn enter_edit_mode_with_nil_message_shows_error() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    state.focused_message_id = None;
    let action = UIAction::NormalMode(NormalModeAction::EnterEditMode);

    reducer::reduce(&mut state, action);

    assert_eq!(state.mode, Mode::Normal);
    assert!(state.error_message.is_some());
    assert!(
        state
            .error_message
            .as_ref()
            .unwrap()
            .contains("No message selected")
    );
}

#[test]
fn enter_append_mode_transitions_correctly() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    let action = UIAction::NormalMode(NormalModeAction::EnterAppendMode);

    reducer::reduce(&mut state, action);

    assert_eq!(state.mode, Mode::Edit);
    assert!(state.error_message.is_none());
}

#[test]
fn exit_current_mode_returns_to_normal() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    // First enter edit mode
    state.mode = Mode::Edit;
    state.error_message = Some("Test error".to_string());

    let action = UIAction::EditMode(EditModeAction::ExitCurrentMode);

    reducer::reduce(&mut state, action);

    assert_eq!(state.mode, Mode::Normal);
    assert!(state.error_message.is_none());
}

#[test]
fn enter_command_palette_shows_not_implemented_error() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    let action = UIAction::NormalMode(NormalModeAction::EnterCommandPalette);

    reducer::reduce(&mut state, action);

    assert_eq!(state.mode, Mode::Normal);
    assert!(state.error_message.is_some());
    assert!(
        state
            .error_message
            .as_ref()
            .unwrap()
            .contains("not implemented")
    );
}

// Branch/Tree Navigation Tests

#[test]
fn next_branch_cycles_through_branches() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    let initial_branch_id = state.current_branch_id.unwrap();
    let action = UIAction::NormalMode(NormalModeAction::NextBranch);

    reducer::reduce(&mut state, action.clone());

    let new_branch_id = state.current_branch_id.unwrap();
    assert_ne!(initial_branch_id, new_branch_id);

    // Verify we can get the new branch
    assert!(state.current_branch().is_some());

    // Cycling again should return to first branch
    reducer::reduce(&mut state, action);
    assert_eq!(state.current_branch_id.unwrap(), initial_branch_id);
}

#[test]
fn prev_branch_cycles_through_branches() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    let initial_branch_id = state.current_branch_id.unwrap();
    let action = UIAction::NormalMode(NormalModeAction::PrevBranch);

    reducer::reduce(&mut state, action);

    let new_branch_id = state.current_branch_id.unwrap();
    assert_ne!(initial_branch_id, new_branch_id);

    // Verify we can get the new branch
    assert!(state.current_branch().is_some());
}

#[test]
fn next_tree_cycles_through_trees() {
    let dialogue = create_test_dialogue_with_multiple_trees();
    let mut state = create_app_state_with_dialogue(dialogue);

    let initial_tree_id = state.current_tree_id.unwrap();
    let action = UIAction::NormalMode(NormalModeAction::NextTree);

    reducer::reduce(&mut state, action.clone());

    let new_tree_id = state.current_tree_id.unwrap();
    assert_ne!(initial_tree_id, new_tree_id);

    // Verify we can get the new tree
    assert!(state.current_tree().is_some());

    // Cycling again should return to first tree
    reducer::reduce(&mut state, action);
    assert_eq!(state.current_tree_id.unwrap(), initial_tree_id);
}

#[test]
fn prev_tree_cycles_through_trees() {
    let dialogue = create_test_dialogue_with_multiple_trees();
    let mut state = create_app_state_with_dialogue(dialogue);

    let initial_tree_id = state.current_tree_id.unwrap();
    let action = UIAction::NormalMode(NormalModeAction::PrevTree);

    reducer::reduce(&mut state, action);

    let new_tree_id = state.current_tree_id.unwrap();
    assert_ne!(initial_tree_id, new_tree_id);

    // Verify we can get the new tree
    assert!(state.current_tree().is_some());
}

#[test]
fn branch_navigation_resets_focus_and_scroll() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    // Set non-zero scroll state
    state.scroll_offset = 10;

    let action = UIAction::NormalMode(NormalModeAction::NextBranch);
    reducer::reduce(&mut state, action);

    // Should reset scroll position
    assert_eq!(state.scroll_offset, 0);

    // Should focus first message of new branch
    let messages = state.current_messages();
    if let Some(first_message) = messages.first() {
        assert_eq!(state.focused_message_id, Some(first_message.id()));
    }
}

#[test]
fn delete_message_removes_from_branch() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    let initial_count = state.current_messages().len();
    let message_id = state.focused_message_id.unwrap();
    let action = UIAction::NormalMode(NormalModeAction::DeleteMessage);

    reducer::reduce(&mut state, action);

    let new_count = state.current_messages().len();
    assert_eq!(new_count, initial_count - 1);

    // Message should no longer exist
    let messages = state.current_messages();
    assert!(!messages.iter().any(|m| m.id() == message_id));

    assert!(state.error_message.is_none());
}

#[test]
fn delete_message_updates_focus_correctly() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    let action = UIAction::NormalMode(NormalModeAction::DeleteMessage);

    reducer::reduce(&mut state, action);

    // Focus should be updated to a valid message
    if !state.current_messages().is_empty() {
        assert!(state.focused_message_id.is_some());
        let focused_id = state.focused_message_id.unwrap();
        let messages = state.current_messages();
        assert!(messages.iter().any(|m| m.id() == focused_id));
    }
}

#[test]
fn fork_branch_creates_new_branch() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    let initial_branch_count = state.current_tree().unwrap().branches().len();
    let action = UIAction::NormalMode(NormalModeAction::ForkBranch);

    reducer::reduce(&mut state, action);

    let new_branch_count = state.current_tree().unwrap().branches().len();
    assert_eq!(new_branch_count, initial_branch_count + 1);

    assert!(state.error_message.is_none());
}

#[test]
fn hide_message_updates_visibility() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    let action = UIAction::NormalMode(NormalModeAction::HideMessage);

    reducer::reduce(&mut state, action);

    // Should complete without error (visibility is internal to frond-core)
    assert!(state.error_message.is_none());
}

#[test]
fn show_message_updates_visibility() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    let action = UIAction::NormalMode(NormalModeAction::ShowMessage);

    reducer::reduce(&mut state, action);

    // Should complete without error (visibility is internal to frond-core)
    assert!(state.error_message.is_none());
}

// Scroll Action Tests

#[test]
fn scroll_up_decreases_offset() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    state.scroll_offset = 5;
    let action = UIAction::NormalMode(NormalModeAction::ScrollUp);

    reducer::reduce(&mut state, action);

    assert_eq!(state.scroll_offset, 4);
}

#[test]
fn scroll_down_increases_offset() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    state.scroll_offset = 5;
    let action = UIAction::NormalMode(NormalModeAction::ScrollDown);

    reducer::reduce(&mut state, action);

    assert_eq!(state.scroll_offset, 6);
}

// Integration Tests

#[test]
fn multiple_operations_maintain_state_consistency() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    // Perform multiple operations
    let operations = vec![
        UIAction::NormalMode(NormalModeAction::ScrollDown),
        UIAction::NormalMode(NormalModeAction::NextBranch),
        UIAction::NormalMode(NormalModeAction::ScrollDown),
        UIAction::NormalMode(NormalModeAction::PrevBranch),
    ];

    for action in operations {
        reducer::reduce(&mut state, action);

        // Verify state consistency after each operation
        if let Some(tree_id) = state.current_tree_id {
            assert!(state.dialogue.get_tree_by_id(tree_id).is_some());
        }

        if let Some(branch_id) = state.current_branch_id {
            assert!(state.dialogue.get_branch_by_id(branch_id).is_some());
        }

        if let Some(message_id) = state.focused_message_id {
            let messages = state.current_messages();
            assert!(messages.iter().any(|m| m.id() == message_id));
        }
    }
}

#[test]
fn operations_with_empty_dialogue_handle_gracefully() {
    let empty_dialogue = Dialogue::new("Empty");
    let mut state = AppState {
        dialogue: empty_dialogue,
        mode: Mode::Normal,
        config: Config::default(),
        model_info: ModelInfo::default(),
        current_tree_id: None,
        current_branch_id: None,
        focused_message_id: None,
        scroll_offset: 0,
        scrollbar_state: ratatui::widgets::ScrollbarState::default(),
        pending_scrolling_request: None,
        error_message: None,
        edit_textarea: None,
        highlight_cache: std::collections::HashMap::new(),
    };

    // Operations on empty state should not panic
    let actions = vec![
        UIAction::NormalMode(NormalModeAction::NextBranch),
        UIAction::NormalMode(NormalModeAction::PrevBranch),
        UIAction::NormalMode(NormalModeAction::NextTree),
        UIAction::NormalMode(NormalModeAction::PrevTree),
        UIAction::NormalMode(NormalModeAction::ScrollUp),
        UIAction::NormalMode(NormalModeAction::ScrollDown),
    ];

    for action in actions {
        reducer::reduce(&mut state, action);
        // Should not panic and should maintain valid state
    }
}

#[test]
fn quit_action_does_not_change_state() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    let initial_mode = state.mode;
    let initial_error = state.error_message.clone();
    let action = UIAction::Common(CommonAction::Quit);

    reducer::reduce(&mut state, action);

    // Quit should not change state (handled at app level)
    assert_eq!(state.mode, initial_mode);
    assert_eq!(state.error_message, initial_error);
}

#[test]
fn reducer_handles_all_action_types_without_panic() {
    let dialogue = create_test_dialogue_with_multiple_branches();
    let mut state = create_app_state_with_dialogue(dialogue);

    // Test all action types
    let actions = vec![
        // Common actions
        UIAction::Common(CommonAction::Quit),
        UIAction::Common(CommonAction::ShowHelp(Mode::Normal)),
        // Normal mode actions
        UIAction::NormalMode(NormalModeAction::ScrollUp),
        UIAction::NormalMode(NormalModeAction::ScrollDown),
        UIAction::NormalMode(NormalModeAction::EnterEditMode),
        UIAction::NormalMode(NormalModeAction::EnterAppendMode),
        UIAction::NormalMode(NormalModeAction::EnterCommandPalette),
        UIAction::NormalMode(NormalModeAction::DeleteMessage),
        UIAction::NormalMode(NormalModeAction::NextBranch),
        UIAction::NormalMode(NormalModeAction::PrevBranch),
        // Edit mode actions
        UIAction::EditMode(EditModeAction::ExitCurrentMode),
    ];

    for action in actions {
        reducer::reduce(&mut state, action);
        // Should not panic for any action type
    }
}
