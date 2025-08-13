//! Tests for undo/redo functionality and focus management
//!
//! These tests verify that undo/redo operations work correctly, including proper
//! focus restoration and cache invalidation through the existing service layer.

use frond::app::state::{AppState, ScrollingRequest};
use frond::services::DialogueService;
use frond_core::{Branch, Dialogue, Message, Role, Tree};
use uuid::Uuid;

// === Helper Functions ===

fn create_test_dialogue() -> Dialogue {
    let mut dialogue = Dialogue::new("Test Dialogue");
    let mut tree = Tree::new("Test Tree");
    let mut branch = Branch::new("main");

    // Add some test messages
    branch.add_message(Message::new("First message", Role::User));
    branch.add_message(Message::new("Second message", Role::Assistant));
    branch.add_message(Message::new("Third message", Role::User));

    tree.add_branch(branch);
    dialogue.add_tree(tree);
    dialogue
}

fn get_first_message_id(dialogue: &Dialogue) -> Uuid {
    dialogue
        .trees()
        .iter()
        .next()
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .messages()
        .iter()
        .next()
        .unwrap()
        .id()
}

// === Undo/Redo Basic Functionality Tests ===

#[test]
fn undo_reverts_message_append() {
    let mut dialogue = create_test_dialogue();
    let initial_count = dialogue
        .trees()
        .iter()
        .next()
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .messages()
        .len();
    let branch_id = dialogue
        .trees()
        .iter()
        .next()
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .id();

    // Append a new message through service
    DialogueService::append_message(&mut dialogue, branch_id, "New test message".to_string())
        .expect("Append should succeed");

    assert_eq!(
        dialogue
            .trees()
            .iter()
            .next()
            .unwrap()
            .branches()
            .iter()
            .next()
            .unwrap()
            .messages()
            .len(),
        initial_count + 1
    );

    // Undo the append
    let undone_action = DialogueService::undo(&mut dialogue).expect("Undo should succeed");

    // Verify message was removed
    assert_eq!(
        dialogue
            .trees()
            .iter()
            .next()
            .unwrap()
            .branches()
            .iter()
            .next()
            .unwrap()
            .messages()
            .len(),
        initial_count
    );

    // Should return the action that was undone
    assert!(undone_action.is_some());
}

#[test]
fn undo_reverts_message_edit() {
    let mut dialogue = create_test_dialogue();
    let message_id = get_first_message_id(&dialogue);
    let original_content = dialogue
        .get_message_by_id(message_id)
        .unwrap()
        .content()
        .to_string();

    // Edit the message through service
    DialogueService::edit_message(&mut dialogue, message_id, "Modified content".to_string())
        .expect("Edit should succeed");

    assert_eq!(
        dialogue.get_message_by_id(message_id).unwrap().content(),
        "Modified content"
    );

    // Undo the edit
    let undone_action = DialogueService::undo(&mut dialogue).expect("Undo should succeed");

    // Verify content was restored
    assert_eq!(
        dialogue.get_message_by_id(message_id).unwrap().content(),
        original_content
    );
    assert!(undone_action.is_some());
}

#[test]
fn redo_reapplies_undone_append() {
    let mut dialogue = create_test_dialogue();
    let initial_count = dialogue
        .trees()
        .iter()
        .next()
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .messages()
        .len();
    let branch_id = dialogue
        .trees()
        .iter()
        .next()
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .id();

    // Append, undo, then redo
    DialogueService::append_message(&mut dialogue, branch_id, "Test message".to_string())
        .expect("Append should succeed");
    DialogueService::undo(&mut dialogue).expect("Undo should succeed");

    let redone_action = DialogueService::redo(&mut dialogue).expect("Redo should succeed");

    // Verify message was re-added
    assert_eq!(
        dialogue
            .trees()
            .iter()
            .next()
            .unwrap()
            .branches()
            .iter()
            .next()
            .unwrap()
            .messages()
            .len(),
        initial_count + 1
    );
    assert!(redone_action.is_some());
}

#[test]
fn multiple_undo_operations_work_correctly() {
    let mut dialogue = create_test_dialogue();
    let initial_count = dialogue
        .trees()
        .iter()
        .next()
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .messages()
        .len();
    let branch_id = dialogue
        .trees()
        .iter()
        .next()
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .id();

    // Perform multiple operations
    DialogueService::append_message(&mut dialogue, branch_id, "Message 1".to_string())
        .expect("Append 1 should succeed");
    DialogueService::append_message(&mut dialogue, branch_id, "Message 2".to_string())
        .expect("Append 2 should succeed");
    DialogueService::append_message(&mut dialogue, branch_id, "Message 3".to_string())
        .expect("Append 3 should succeed");

    assert_eq!(
        dialogue
            .trees()
            .iter()
            .next()
            .unwrap()
            .branches()
            .iter()
            .next()
            .unwrap()
            .messages()
            .len(),
        initial_count + 3
    );

    // Undo twice
    DialogueService::undo(&mut dialogue).expect("Undo 1 should succeed");
    DialogueService::undo(&mut dialogue).expect("Undo 2 should succeed");

    assert_eq!(
        dialogue
            .trees()
            .iter()
            .next()
            .unwrap()
            .branches()
            .iter()
            .next()
            .unwrap()
            .messages()
            .len(),
        initial_count + 1
    );

    // Redo once
    DialogueService::redo(&mut dialogue).expect("Redo should succeed");

    assert_eq!(
        dialogue
            .trees()
            .iter()
            .next()
            .unwrap()
            .branches()
            .iter()
            .next()
            .unwrap()
            .messages()
            .len(),
        initial_count + 2
    );
}

#[test]
fn undo_on_empty_history_returns_none() {
    let mut dialogue = create_test_dialogue();

    // Try to undo without any operations
    let result = DialogueService::undo(&mut dialogue).expect("Should not error");

    // Should return None indicating no action was undone
    assert!(result.is_none());
}

#[test]
fn redo_on_empty_future_returns_none() {
    let mut dialogue = create_test_dialogue();

    // Try to redo without any undone operations
    let result = DialogueService::redo(&mut dialogue).expect("Should not error");

    // Should return None indicating no action was redone
    assert!(result.is_none());
}

// === Focus Management Tests ===

#[test]
fn focus_message_works_across_trees_and_branches() {
    let mut dialogue = Dialogue::new("Multi-Tree Test");

    // Create first tree with two branches
    let mut tree1 = Tree::new("Tree 1");
    let mut branch1a = Branch::new("branch-1a");
    let mut branch1b = Branch::new("branch-1b");

    branch1a.add_message(Message::new("Tree1 Branch1A Message1", Role::User));
    branch1a.add_message(Message::new("Tree1 Branch1A Message2", Role::Assistant));

    branch1b.add_message(Message::new("Tree1 Branch1B Message1", Role::User));

    tree1.add_branch(branch1a);
    tree1.add_branch(branch1b);

    // Create second tree with one branch
    let mut tree2 = Tree::new("Tree 2");
    let mut branch2a = Branch::new("branch-2a");

    branch2a.add_message(Message::new("Tree2 Branch2A Message1", Role::User));
    branch2a.add_message(Message::new("Tree2 Branch2A Message2", Role::Assistant));
    branch2a.add_message(Message::new("Tree2 Branch2A Message3", Role::User));

    tree2.add_branch(branch2a);

    dialogue.add_tree(tree1);
    dialogue.add_tree(tree2);

    let mut state = AppState {
        dialogue,
        ..AppState::default()
    };

    // Start with first tree, first branch
    let tree1_id = state.dialogue.trees().iter().next().unwrap().id();
    let branch1a_id = state
        .dialogue
        .trees()
        .iter()
        .next()
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .id();
    state.current_tree_id = Some(tree1_id);
    state.current_branch_id = Some(branch1a_id);

    // Get a message from the second tree
    let tree2_id = state.dialogue.trees().iter().nth(1).unwrap().id();
    let branch2a_id = state
        .dialogue
        .trees()
        .iter()
        .nth(1)
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .id();
    let message_in_tree2 = state
        .dialogue
        .trees()
        .iter()
        .nth(1)
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .messages()
        .iter()
        .nth(1)
        .unwrap()
        .id();

    // Focus on message in different tree
    state.focus_message(message_in_tree2);

    // Should switch to the correct tree and branch
    assert_eq!(state.current_tree_id, Some(tree2_id));
    assert_eq!(state.current_branch_id, Some(branch2a_id));

    // Should request scroll to the message
    assert_eq!(
        state.pending_scrolling_request,
        Some(ScrollingRequest::ScrollToMessage(message_in_tree2))
    );
}

#[test]
fn focus_message_handles_nonexistent_message() {
    let dialogue = create_test_dialogue();
    let mut state = AppState {
        dialogue,
        ..AppState::default()
    };

    // Set up current context
    let tree1_id = state.dialogue.trees().iter().next().unwrap().id();
    let branch1a_id = state
        .dialogue
        .trees()
        .iter()
        .next()
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .id();
    state.current_tree_id = Some(tree1_id);
    state.current_branch_id = Some(branch1a_id);

    let original_tree = state.current_tree_id;
    let original_branch = state.current_branch_id;

    // Try to focus on nonexistent message
    let fake_message_id = Uuid::new_v4();
    state.focus_message(fake_message_id);

    // Should not change current context
    assert_eq!(state.current_tree_id, original_tree);
    assert_eq!(state.current_branch_id, original_branch);

    // Should still request scroll (will fail gracefully during rendering)
    assert_eq!(
        state.pending_scrolling_request,
        Some(ScrollingRequest::ScrollToMessage(fake_message_id))
    );
}

// === Cache Management Tests (Basic) ===

#[test]
fn message_highlighting_cache_works_correctly() {
    let dialogue = create_test_dialogue();
    let mut state = AppState {
        dialogue,
        ..AppState::default()
    };

    // Set up current context
    let tree_id = state.dialogue.trees().iter().next().unwrap().id();
    let branch_id = state
        .dialogue
        .trees()
        .iter()
        .next()
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .id();
    state.current_tree_id = Some(tree_id);
    state.current_branch_id = Some(branch_id);

    let message_id = get_first_message_id(&state.dialogue);

    // Initially message should not be cached
    assert!(!state.has_cached_highlight(message_id));

    // Cache some text manually to test the cache functionality
    let test_text = ratatui::text::Text::raw("test highlighting");
    state.cache_highlighted_text(message_id, test_text.clone());

    // Now should be cached
    assert!(state.has_cached_highlight(message_id));

    // Should be able to retrieve cached version
    let cached = state.get_cached_highlighted_text(message_id);
    assert!(cached.is_some());
    assert_eq!(cached.unwrap(), test_text);
}

#[test]
fn height_cache_works_correctly() {
    let dialogue = create_test_dialogue();
    let mut state = AppState {
        dialogue,
        ..AppState::default()
    };

    // Set up current context
    let tree_id = state.dialogue.trees().iter().next().unwrap().id();
    let branch_id = state
        .dialogue
        .trees()
        .iter()
        .next()
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .id();
    state.current_tree_id = Some(tree_id);
    state.current_branch_id = Some(branch_id);

    let message_id = get_first_message_id(&state.dialogue);
    let viewport_width = 80u16;

    // Initially should not be cached
    assert!(
        state
            .get_cached_height(message_id, viewport_width)
            .is_none()
    );

    // Cache a height
    state.cache_height(message_id, viewport_width, 5);

    // Should now be cached
    assert_eq!(state.get_cached_height(message_id, viewport_width), Some(5));

    // Different viewport width should not be cached
    assert!(state.get_cached_height(message_id, 100).is_none());
}

#[test]
fn cache_invalidation_works_correctly() {
    let dialogue = create_test_dialogue();
    let mut state = AppState {
        dialogue,
        ..AppState::default()
    };

    // Set up current context
    let tree_id = state.dialogue.trees().iter().next().unwrap().id();
    let branch_id = state
        .dialogue
        .trees()
        .iter()
        .next()
        .unwrap()
        .branches()
        .iter()
        .next()
        .unwrap()
        .id();
    state.current_tree_id = Some(tree_id);
    state.current_branch_id = Some(branch_id);

    let message_id = get_first_message_id(&state.dialogue);

    // Populate caches manually
    let test_text = ratatui::text::Text::raw("test highlighting");
    state.cache_highlighted_text(message_id, test_text);
    state.cache_height(message_id, 80, 5);

    // Verify caches are populated
    assert!(state.has_cached_highlight(message_id));
    assert!(state.get_cached_height(message_id, 80).is_some());

    // Invalidate message caches
    state.invalidate_message_highlight(message_id);
    state.invalidate_message_height(message_id);

    // Should be cleared
    assert!(!state.has_cached_highlight(message_id));
    assert!(state.get_cached_height(message_id, 80).is_none());
}
