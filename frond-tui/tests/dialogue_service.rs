//! Tests for DialogueService business logic
//!
//! These tests verify that the DialogueService correctly performs business operations
//! on dialogue data and maintains consistency.

use frond::services::DialogueService;
use frond_core::{Branch, Dialogue, Message, Role, Tree};
use uuid::Uuid;

// === Helper Functions ===

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

    tree.add_branch(branch);
    dialogue.add_tree(tree);
    dialogue
}

fn get_first_message_id(dialogue: &Dialogue) -> Option<Uuid> {
    dialogue
        .trees()
        .get(0)?
        .branches()
        .get(0)?
        .messages()
        .get(0)
        .map(|m| m.id())
}

fn get_first_branch_id(dialogue: &Dialogue) -> Option<Uuid> {
    dialogue.trees().get(0)?.branches().get(0).map(|b| b.id())
}

fn get_first_tree_id(dialogue: &Dialogue) -> Option<Uuid> {
    dialogue.trees().get(0).map(|t| t.id())
}

// === Message Operations ===

#[test]
fn edit_message_updates_content() {
    let mut dialogue = create_test_dialogue();
    let message_id = get_first_message_id(&dialogue).unwrap();
    let original_content = dialogue
        .get_message_by_id(message_id)
        .unwrap()
        .content()
        .to_string();

    let new_content = "Updated message content".to_string();
    let result = DialogueService::edit_message(&mut dialogue, message_id, new_content.clone());

    assert!(result.is_ok(), "Should successfully edit message");

    let updated_message = dialogue.get_message_by_id(message_id).unwrap();
    assert_eq!(updated_message.content(), new_content);
    assert_ne!(updated_message.content(), original_content);
}

#[test]
fn edit_message_preserves_role_and_id() {
    let mut dialogue = create_test_dialogue();
    let message_id = get_first_message_id(&dialogue).unwrap();
    let original_role = *dialogue.get_message_by_id(message_id).unwrap().role();

    let result =
        DialogueService::edit_message(&mut dialogue, message_id, "New content".to_string());

    assert!(result.is_ok());

    let updated_message = dialogue.get_message_by_id(message_id).unwrap();
    assert_eq!(updated_message.id(), message_id);
    assert_eq!(updated_message.role(), &original_role);
}

#[test]
fn edit_message_fails_with_invalid_id() {
    let mut dialogue = create_test_dialogue();
    let invalid_id = Uuid::new_v4();
    let result =
        DialogueService::edit_message(&mut dialogue, invalid_id, "New content".to_string());

    assert!(result.is_err(), "Should fail with invalid message ID");
}

#[test]
fn append_message_adds_message_to_branch() {
    let mut dialogue = create_test_dialogue();
    let branch_id = get_first_branch_id(&dialogue).unwrap();
    let original_count = dialogue
        .get_branch_by_id(branch_id)
        .unwrap()
        .messages()
        .len();

    let result = DialogueService::append_message(
        &mut dialogue,
        branch_id,
        "New message content".to_string(),
    );

    assert!(result.is_ok(), "Should successfully append message");

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(
        branch.messages().len(),
        original_count + 1,
        "Should have one more message"
    );

    let last_message = branch.messages().get(branch.messages().len() - 1).unwrap();
    assert_eq!(last_message.content(), "New message content");
}

#[test]
fn append_message_fails_with_invalid_branch_id() {
    let mut dialogue = create_test_dialogue();
    let invalid_branch_id = Uuid::new_v4();

    let result = DialogueService::append_message(
        &mut dialogue,
        invalid_branch_id,
        "Test message".to_string(),
    );

    assert!(result.is_err(), "Should fail with invalid branch ID");
}

#[test]
fn delete_message_removes_message() {
    let mut dialogue = create_test_dialogue();
    let message_id = get_first_message_id(&dialogue).unwrap();
    let branch_id = get_first_branch_id(&dialogue).unwrap();
    let original_count = dialogue
        .get_branch_by_id(branch_id)
        .unwrap()
        .messages()
        .len();

    let result = DialogueService::delete_message(&mut dialogue, message_id, branch_id);

    assert!(result.is_ok(), "Should successfully delete message");

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(
        branch.messages().len(),
        original_count - 1,
        "Should have one fewer message"
    );

    // Message should no longer exist
    assert!(dialogue.get_message_by_id(message_id).is_none());
}

#[test]
fn delete_message_fails_with_invalid_ids() {
    let mut dialogue = create_test_dialogue();
    let message_id = get_first_message_id(&dialogue).unwrap();
    let branch_id = get_first_branch_id(&dialogue).unwrap();

    // Invalid message ID
    let result1 = DialogueService::delete_message(&mut dialogue, Uuid::new_v4(), branch_id);
    assert!(result1.is_err());

    // Invalid branch ID
    let result2 = DialogueService::delete_message(&mut dialogue, message_id, Uuid::new_v4());
    assert!(result2.is_err());
}

// === Branch Operations ===

#[test]
fn fork_branch_creates_new_branch() {
    let mut dialogue = create_test_dialogue();
    let tree_id = get_first_tree_id(&dialogue).unwrap();
    let branch_id = get_first_branch_id(&dialogue).unwrap();
    let message_id = get_first_message_id(&dialogue).unwrap();
    let original_branch_count = dialogue.get_tree_by_id(tree_id).unwrap().branches().len();

    let result = DialogueService::fork_branch(
        &mut dialogue,
        tree_id,
        branch_id,
        message_id,
        "new_branch".to_string(),
    );

    assert!(result.is_ok(), "Should successfully fork branch");

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(
        tree.branches().len(),
        original_branch_count + 1,
        "Should have one more branch"
    );
}

#[test]
fn fork_branch_fails_with_invalid_ids() {
    let mut dialogue = create_test_dialogue();
    let tree_id = get_first_tree_id(&dialogue).unwrap();
    let branch_id = get_first_branch_id(&dialogue).unwrap();
    let message_id = get_first_message_id(&dialogue).unwrap();

    // Invalid tree ID
    let result1 = DialogueService::fork_branch(
        &mut dialogue,
        Uuid::new_v4(),
        branch_id,
        message_id,
        "test".to_string(),
    );
    assert!(result1.is_err());

    // Invalid branch ID
    let result2 = DialogueService::fork_branch(
        &mut dialogue,
        tree_id,
        Uuid::new_v4(),
        message_id,
        "test".to_string(),
    );
    assert!(result2.is_err());

    // Invalid message ID
    let result3 = DialogueService::fork_branch(
        &mut dialogue,
        tree_id,
        branch_id,
        Uuid::new_v4(),
        "test".to_string(),
    );
    assert!(result3.is_err());
}

// === Message Visibility ===

#[test]
fn hide_message_succeeds() {
    let mut dialogue = create_test_dialogue();
    let message_id = get_first_message_id(&dialogue).unwrap();

    let result = DialogueService::hide_message(&mut dialogue, message_id);

    assert!(result.is_ok(), "Should successfully hide message");
}

#[test]
fn show_message_succeeds() {
    let mut dialogue = create_test_dialogue();
    let message_id = get_first_message_id(&dialogue).unwrap();

    // First hide the message
    DialogueService::hide_message(&mut dialogue, message_id).unwrap();

    // Then show it again
    let result = DialogueService::show_message(&mut dialogue, message_id);

    assert!(result.is_ok(), "Should successfully show message");
}

#[test]
fn visibility_operations_fail_with_invalid_id() {
    let mut dialogue = create_test_dialogue();
    let invalid_id = Uuid::new_v4();

    let hide_result = DialogueService::hide_message(&mut dialogue, invalid_id);
    let show_result = DialogueService::show_message(&mut dialogue, invalid_id);

    assert!(hide_result.is_err(), "Hide should fail with invalid ID");
    assert!(show_result.is_err(), "Show should fail with invalid ID");
}

// === Undo/Redo Operations ===

#[test]
fn undo_reverts_edit_operation() {
    let mut dialogue = create_test_dialogue();
    let message_id = get_first_message_id(&dialogue).unwrap();
    let original_content = dialogue
        .get_message_by_id(message_id)
        .unwrap()
        .content()
        .to_string();

    // Edit the message
    DialogueService::edit_message(&mut dialogue, message_id, "Edited content".to_string()).unwrap();

    // Undo the edit
    let undo_result = DialogueService::undo(&mut dialogue);
    assert!(undo_result.is_ok(), "Should successfully undo");

    // Content should be back to original
    let reverted_message = dialogue.get_message_by_id(message_id).unwrap();
    assert_eq!(reverted_message.content(), original_content);
}

#[test]
fn redo_reapplies_undone_operation() {
    let mut dialogue = create_test_dialogue();
    let message_id = get_first_message_id(&dialogue).unwrap();
    let new_content = "Edited content".to_string();

    // Edit, undo, then redo
    DialogueService::edit_message(&mut dialogue, message_id, new_content.clone()).unwrap();
    DialogueService::undo(&mut dialogue).unwrap();

    let redo_result = DialogueService::redo(&mut dialogue);
    assert!(redo_result.is_ok(), "Should successfully redo");

    // Content should be back to edited version
    let redone_message = dialogue.get_message_by_id(message_id).unwrap();
    assert_eq!(redone_message.content(), new_content);
}

#[test]
fn undo_succeeds_when_no_history() {
    let mut dialogue = create_test_dialogue();

    let result = DialogueService::undo(&mut dialogue);
    // Undo with no history succeeds silently (no-op behavior)
    assert!(
        result.is_ok(),
        "Undo with no history should succeed silently"
    );
}

#[test]
fn redo_succeeds_when_no_future() {
    let mut dialogue = create_test_dialogue();

    let result = DialogueService::redo(&mut dialogue);
    // Redo with no future succeeds silently (no-op behavior)
    assert!(
        result.is_ok(),
        "Redo with no future should succeed silently"
    );
}

// === Error Handling ===

#[test]
fn service_handles_empty_dialogue() {
    let mut empty_dialogue = Dialogue::new("Empty");

    let fake_message_id = Uuid::new_v4();
    let fake_branch_id = Uuid::new_v4();
    let fake_tree_id = Uuid::new_v4();

    // All operations should fail gracefully with empty dialogue
    let edit_result =
        DialogueService::edit_message(&mut empty_dialogue, fake_message_id, "Test".to_string());
    assert!(edit_result.is_err());

    let append_result =
        DialogueService::append_message(&mut empty_dialogue, fake_branch_id, "Test".to_string());
    assert!(append_result.is_err());

    let delete_result =
        DialogueService::delete_message(&mut empty_dialogue, fake_message_id, fake_branch_id);
    assert!(delete_result.is_err());

    let fork_result = DialogueService::fork_branch(
        &mut empty_dialogue,
        fake_tree_id,
        fake_branch_id,
        fake_message_id,
        "test".to_string(),
    );
    assert!(fork_result.is_err());

    let hide_result = DialogueService::hide_message(&mut empty_dialogue, fake_message_id);
    assert!(hide_result.is_err());

    let show_result = DialogueService::show_message(&mut empty_dialogue, fake_message_id);
    assert!(show_result.is_err());
}

// === Business Logic ===

#[test]
fn edit_empty_content_is_allowed() {
    let mut dialogue = create_test_dialogue();
    let message_id = get_first_message_id(&dialogue).unwrap();
    let result = DialogueService::edit_message(&mut dialogue, message_id, "".to_string());

    assert!(result.is_ok(), "Should allow editing to empty content");

    let message = dialogue.get_message_by_id(message_id).unwrap();
    assert_eq!(message.content(), "");
}

#[test]
fn append_empty_content_is_allowed() {
    let mut dialogue = create_test_dialogue();
    let branch_id = get_first_branch_id(&dialogue).unwrap();

    let result = DialogueService::append_message(&mut dialogue, branch_id, "".to_string());

    assert!(result.is_ok(), "Should allow appending empty content");
}

#[test]
fn multiple_operations_on_same_message() {
    let mut dialogue = create_test_dialogue();
    let message_id = get_first_message_id(&dialogue).unwrap();

    // Edit, hide, show, edit again
    DialogueService::edit_message(&mut dialogue, message_id, "First edit".to_string()).unwrap();
    DialogueService::hide_message(&mut dialogue, message_id).unwrap();
    DialogueService::show_message(&mut dialogue, message_id).unwrap();
    DialogueService::edit_message(&mut dialogue, message_id, "Second edit".to_string()).unwrap();

    let final_message = dialogue.get_message_by_id(message_id).unwrap();
    assert_eq!(final_message.content(), "Second edit");
}

#[test]
fn service_operations_are_idempotent_where_appropriate() {
    let mut dialogue = create_test_dialogue();
    let message_id = get_first_message_id(&dialogue).unwrap();

    // Hide twice
    DialogueService::hide_message(&mut dialogue, message_id).unwrap();
    let result2 = DialogueService::hide_message(&mut dialogue, message_id);
    assert!(result2.is_ok(), "Hiding already hidden message should work");

    // Show twice
    DialogueService::show_message(&mut dialogue, message_id).unwrap();
    let result4 = DialogueService::show_message(&mut dialogue, message_id);
    assert!(
        result4.is_ok(),
        "Showing already visible message should work"
    );
}

#[test]
fn service_preserves_dialogue_integrity_across_operations() {
    let mut dialogue = create_test_dialogue();
    let original_tree_count = dialogue.trees().len();
    let branch_id = get_first_branch_id(&dialogue).unwrap();

    // Perform various operations
    DialogueService::append_message(&mut dialogue, branch_id, "Test 1".to_string()).unwrap();
    DialogueService::append_message(&mut dialogue, branch_id, "Test 2".to_string()).unwrap();

    let message_id = dialogue
        .get_branch_by_id(branch_id)
        .unwrap()
        .messages()
        .get(0)
        .unwrap()
        .id();
    DialogueService::edit_message(&mut dialogue, message_id, "Edited".to_string()).unwrap();

    // Dialogue should still be valid
    assert_eq!(dialogue.trees().len(), original_tree_count);
    assert!(dialogue.get_branch_by_id(branch_id).is_some());
}
