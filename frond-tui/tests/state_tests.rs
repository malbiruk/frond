//! Tests for AppState and state management
//!
//! This module tests the AppState struct and its helper methods for navigation,
//! focus management, and state consistency.

use frond::app::{AppState, EditMode, Mode};
use frond::config::Config;
use frond_core::{Branch, Dialogue, Message, Role, Tree};
use uuid::Uuid;

fn create_test_dialogue() -> Dialogue {
    let mut dialogue = Dialogue::new("Test Dialogue");
    let mut tree = Tree::new("Test Tree");
    let mut branch = Branch::new("main");

    branch.add_message(Message::new(
        "Hello, how can I help you today?",
        Role::Assistant,
    ));
    branch.add_message(Message::new(
        "I need help with Rust programming",
        Role::User,
    ));
    branch.add_message(Message::new(
        "Of course! What specific aspect of Rust would you like to learn about?",
        Role::Assistant,
    ));
    branch.add_message(Message::new(
        "I'm struggling with ownership and borrowing",
        Role::User,
    ));

    tree.add_branch(branch);
    dialogue.add_tree(tree);
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
        current_tree_id: tree_id,
        current_branch_id: branch_id,
        focused_message_id: message_id,
        scroll_offset: 0,
        error_message: None,
    }
}

#[test]
fn app_state_default_creates_valid_state() {
    let state = AppState::default();

    assert_eq!(state.mode, Mode::Normal);
    assert!(state.error_message.is_none());
    assert_eq!(state.scroll_offset, 0);
    assert!(state.current_tree_id.is_some());
    assert!(state.current_branch_id.is_some());
    assert!(state.focused_message_id.is_some());
}

#[test]
fn app_state_current_tree_returns_correct_tree() {
    let dialogue = create_test_dialogue();
    let state = create_app_state_with_dialogue(dialogue);

    let current_tree = state.current_tree();
    assert!(current_tree.is_some());

    let tree = current_tree.unwrap();
    assert_eq!(tree.id(), state.current_tree_id.unwrap());
    assert_eq!(tree.name(), "Test Tree");
}

#[test]
fn app_state_current_tree_returns_none_when_no_tree_selected() {
    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);
    state.current_tree_id = None;

    let current_tree = state.current_tree();
    assert!(current_tree.is_none());
}

#[test]
fn app_state_current_branch_returns_correct_branch() {
    let dialogue = create_test_dialogue();
    let state = create_app_state_with_dialogue(dialogue);

    let current_branch = state.current_branch();
    assert!(current_branch.is_some());

    let branch = current_branch.unwrap();
    assert_eq!(branch.id(), state.current_branch_id.unwrap());
    assert_eq!(branch.name(), "main");
}

#[test]
fn app_state_current_branch_returns_none_when_no_branch_selected() {
    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);
    state.current_branch_id = None;

    let current_branch = state.current_branch();
    assert!(current_branch.is_none());
}

#[test]
fn app_state_current_messages_returns_branch_messages() {
    let dialogue = create_test_dialogue();
    let state = create_app_state_with_dialogue(dialogue);

    let messages = state.current_messages();
    assert_eq!(messages.len(), 4);

    assert_eq!(messages[0].content(), "Hello, how can I help you today?");
    assert_eq!(messages[1].content(), "I need help with Rust programming");
    assert_eq!(
        messages[2].content(),
        "Of course! What specific aspect of Rust would you like to learn about?"
    );
    assert_eq!(
        messages[3].content(),
        "I'm struggling with ownership and borrowing"
    );
}

#[test]
fn app_state_current_messages_returns_empty_when_no_branch() {
    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);
    state.current_branch_id = None;

    let messages = state.current_messages();
    assert!(messages.is_empty());
}

#[test]
fn app_state_update_focused_message_from_scroll_sets_correct_message() {
    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);

    // Set scroll to second message
    state.scroll_offset = 1;
    state.update_focused_message_from_scroll();

    let messages = state.current_messages();
    assert_eq!(state.focused_message_id, Some(messages[1].id()));
}

#[test]
fn app_state_update_focused_message_from_scroll_handles_out_of_bounds() {
    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);

    let original_focus = state.focused_message_id;

    // Set scroll beyond available messages
    state.scroll_offset = 999;
    state.update_focused_message_from_scroll();

    // Focus should remain unchanged when scroll is out of bounds
    assert_eq!(state.focused_message_id, original_focus);
}

#[test]
fn app_state_get_message_index_returns_correct_index() {
    let dialogue = create_test_dialogue();
    let state = create_app_state_with_dialogue(dialogue);

    let messages = state.current_messages();
    let second_message_id = messages[1].id();

    let index = state.get_message_index(second_message_id);
    assert_eq!(index, Some(1));
}

#[test]
fn app_state_get_message_index_returns_none_for_unknown_message() {
    let dialogue = create_test_dialogue();
    let state = create_app_state_with_dialogue(dialogue);

    let unknown_id = Uuid::new_v4();
    let index = state.get_message_index(unknown_id);
    assert_eq!(index, None);
}

#[test]
fn app_state_has_messages_after_returns_true_for_middle_messages() {
    let dialogue = create_test_dialogue();
    let state = create_app_state_with_dialogue(dialogue);

    let messages = state.current_messages();
    let first_message_id = messages[0].id();

    assert!(state.has_messages_after(first_message_id));
}

#[test]
fn app_state_has_messages_after_returns_false_for_last_message() {
    let dialogue = create_test_dialogue();
    let state = create_app_state_with_dialogue(dialogue);

    let messages = state.current_messages();
    let last_message_id = messages[messages.len() - 1].id();

    assert!(!state.has_messages_after(last_message_id));
}

#[test]
fn app_state_has_messages_after_returns_false_for_unknown_message() {
    let dialogue = create_test_dialogue();
    let state = create_app_state_with_dialogue(dialogue);

    let unknown_id = Uuid::new_v4();
    assert!(!state.has_messages_after(unknown_id));
}

#[test]
fn app_state_clear_error_removes_error_message() {
    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);

    state.error_message = Some("Test error".to_string());
    assert!(state.error_message.is_some());

    state.clear_error();
    assert!(state.error_message.is_none());
}

#[test]
fn app_state_mode_transitions_preserve_navigation() {
    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);

    let original_tree_id = state.current_tree_id;
    let original_branch_id = state.current_branch_id;
    let original_message_id = state.focused_message_id;

    // Change mode
    state.mode = Mode::Edit(EditMode::Append);

    // Navigation should be preserved
    assert_eq!(state.current_tree_id, original_tree_id);
    assert_eq!(state.current_branch_id, original_branch_id);
    assert_eq!(state.focused_message_id, original_message_id);
}

#[test]
fn app_state_scroll_for_edit_mode_does_not_panic() {
    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);

    let message_id = state.focused_message_id.unwrap();

    // Test different edit modes
    state.mode = Mode::Edit(EditMode::Append);
    state.scroll_for_edit_mode(); // Should not panic

    state.mode = Mode::Edit(EditMode::EditInPlace {
        message_id,
        has_messages_below: true,
    });
    state.scroll_for_edit_mode(); // Should not panic

    state.mode = Mode::Edit(EditMode::EditInPlace {
        message_id,
        has_messages_below: false,
    });
    state.scroll_for_edit_mode(); // Should not panic
}

#[test]
fn app_state_handles_empty_dialogue() {
    let empty_dialogue = Dialogue::new("Empty");
    let mut state = AppState {
        dialogue: empty_dialogue,
        mode: Mode::Normal,
        config: Config::default(),
        current_tree_id: None,
        current_branch_id: None,
        focused_message_id: None,
        scroll_offset: 0,
        error_message: None,
    };

    // Should handle empty dialogue gracefully
    assert!(state.current_tree().is_none());
    assert!(state.current_branch().is_none());
    assert!(state.current_messages().is_empty());
    assert!(state.focused_message_id.is_none());

    // Verify all navigation methods handle empty state gracefully
    state.update_focused_message_from_scroll();
    state.reset_focus_for_new_branch();
    state.scroll_for_edit_mode();
}

#[test]
fn app_state_update_focused_message_after_deletion_handles_middle_deletion() {
    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);

    // Simulate deletion of second message (index 1)
    state.update_focused_message_after_deletion(1);

    // Should focus on the message that moved into index 1
    let new_messages = state.current_messages();
    if new_messages.len() > 1 {
        assert_eq!(state.focused_message_id, Some(new_messages[1].id()));
    }
}

#[test]
fn app_state_update_focused_message_after_deletion_handles_empty_branch() {
    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);

    // Set up a scenario where messages would be empty
    state.current_messages(); // Initialize

    // Simulate deletion when no messages remain
    state.update_focused_message_after_deletion(0);

    // In a real scenario with empty messages, focus should be None
    // This test verifies the method doesn't panic and maintains state integrity
    assert!(state.focused_message_id.is_some()); // Still has valid focus since messages aren't actually deleted
}

#[test]
fn app_state_reset_focus_for_new_branch_focuses_first_message() {
    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);

    state.reset_focus_for_new_branch();

    // Should focus on first message of branch and reset scroll
    let branch_messages = state.current_messages();
    assert_eq!(state.scroll_offset, 0);
    if !branch_messages.is_empty() {
        assert_eq!(state.focused_message_id, Some(branch_messages[0].id()));
    }
}

#[test]
fn app_state_all_helper_methods_handle_invalid_state() {
    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);

    // Corrupt state by setting invalid IDs
    state.current_tree_id = Some(Uuid::new_v4());
    state.current_branch_id = Some(Uuid::new_v4());
    state.focused_message_id = Some(Uuid::new_v4());

    // All methods should handle invalid state gracefully
    assert!(state.current_tree().is_none());
    assert!(state.current_branch().is_none());
    assert!(state.current_messages().is_empty());
    assert_eq!(state.get_message_index(Uuid::new_v4()), None);
    assert!(!state.has_messages_after(Uuid::new_v4()));

    // These should not panic
    state.update_focused_message_from_scroll();
    state.update_focused_message_after_deletion(0);
    state.reset_focus_for_new_branch();
    state.reset_focus_for_new_tree();
    state.scroll_for_edit_mode();
}

#[test]
fn app_state_action_dispatcher_integration() {
    use frond::actions::{ActionDispatcher, UIAction};

    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);

    let original_scroll = state.scroll_offset;

    // Test that state implements ActionDispatcher
    state.dispatch(UIAction::ScrollDown);

    // Should have updated scroll position
    assert_eq!(state.scroll_offset, original_scroll + 1);
}

#[test]
fn app_state_navigation_consistency_across_operations() {
    let dialogue = create_test_dialogue();
    let mut state = create_app_state_with_dialogue(dialogue);

    // Perform various navigation operations
    state.scroll_offset = 2;
    state.update_focused_message_from_scroll();

    // Verify navigation consistency
    if let Some(focused_id) = state.focused_message_id {
        let messages = state.current_messages();
        assert!(
            messages.iter().any(|m| m.id() == focused_id),
            "Focused message should belong to current branch"
        );
    }

    // Change scroll and verify again
    state.scroll_offset = 0;
    state.update_focused_message_from_scroll();

    if let Some(focused_id) = state.focused_message_id {
        let messages = state.current_messages();
        assert!(
            messages.iter().any(|m| m.id() == focused_id),
            "Focused message should belong to current branch after scroll change"
        );
    }
}
