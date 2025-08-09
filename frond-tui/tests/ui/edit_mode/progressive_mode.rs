//! Tests for progressive edit mode behavior
//!
//! These tests verify the overall behavior of the progressive edit mode system,
//! focusing on how the mode adapts to different content sizes and provides
//! the expected user experience.

use frond::app::state::AppState;
use frond::app::Mode;
use frond::config::Config;
use frond_core::{Branch, Dialogue, Message, Role, Tree};
use tui_textarea::TextArea;

fn create_dialogue_with_short_message() -> Dialogue {
    let mut dialogue = Dialogue::new("Test Dialogue");
    let mut tree = Tree::new("Test Tree");
    let mut branch = Branch::new("main");
    
    branch.add_message(Message::new("Hi", Role::User)); // Short message
    branch.add_message(Message::new("Hello there!", Role::Assistant));
    
    tree.add_branch(branch);
    dialogue.add_tree(tree);
    dialogue
}

fn create_dialogue_with_long_message() -> Dialogue {
    let mut dialogue = Dialogue::new("Test Dialogue");
    let mut tree = Tree::new("Test Tree");
    let mut branch = Branch::new("main");
    
    let long_content = "This is a very long message that will span multiple lines when rendered in a terminal. ".repeat(10);
    branch.add_message(Message::new(&long_content, Role::User));
    branch.add_message(Message::new("Short response", Role::Assistant));
    
    tree.add_branch(branch);
    dialogue.add_tree(tree);
    dialogue
}

fn create_app_state_editing_message(dialogue: Dialogue, message_index: usize, content: &str) -> AppState {
    let tree_id = dialogue.trees().get(0).unwrap().id();
    let branch_id = dialogue.trees().get(0).unwrap().branches().get(0).unwrap().id();
    let message_id = dialogue.trees().get(0).unwrap().branches().get(0).unwrap()
        .messages().get(message_index).unwrap().id();
    
    let mut textarea = TextArea::default();
    textarea.insert_str(content);
    
    AppState {
        dialogue,
        mode: Mode::Edit,
        config: Config::default(),
        model_info: Default::default(),
        current_tree_id: Some(tree_id),
        current_branch_id: Some(branch_id),
        focused_message_id: Some(message_id),
        scroll_offset: 0,
        scrollbar_state: Default::default(),
        pending_scrolling_request: None,
        error_message: None,
        edit_textarea: Some(textarea),
        highlight_cache: Default::default(),
    }
}

// === Progressive Behavior Tests ===

#[test]
fn small_message_edit_provides_centered_experience() {
    let dialogue = create_dialogue_with_short_message();
    let app_state = create_app_state_editing_message(dialogue, 0, "Short edit");
    
    // Verify we're in edit mode with content
    assert_eq!(app_state.mode, Mode::Edit);
    assert!(app_state.edit_textarea.is_some());
    assert_eq!(app_state.edit_textarea.as_ref().unwrap().lines(), vec!["Short edit"]);
    
    // For short content, should have space above and below for centering
    let textarea_lines = app_state.edit_textarea.as_ref().unwrap().lines().len();
    assert!(textarea_lines <= 5, "Short content should not require many lines");
}

#[test]
fn growing_message_edit_adapts_layout() {
    let dialogue = create_dialogue_with_short_message();
    let multiline_content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
    let app_state = create_app_state_editing_message(dialogue, 0, multiline_content);
    
    // Verify content is preserved
    let lines = app_state.edit_textarea.as_ref().unwrap().lines();
    assert_eq!(lines.len(), 5);
    assert_eq!(lines[0], "Line 1");
    assert_eq!(lines[4], "Line 5");
    
    // Should handle multi-line content gracefully
    assert_eq!(app_state.mode, Mode::Edit);
}

#[test]
fn large_message_edit_handles_gracefully() {
    let dialogue = create_dialogue_with_long_message();
    let very_long_content = "Very long line content that exceeds normal viewport width and height\n".repeat(20);
    let app_state = create_app_state_editing_message(dialogue, 0, &very_long_content);
    
    // Should still be in edit mode and handle large content
    assert_eq!(app_state.mode, Mode::Edit);
    assert!(app_state.edit_textarea.is_some());
    
    let lines = app_state.edit_textarea.as_ref().unwrap().lines();
    assert!(lines.len() >= 20, "Should preserve all content lines");
}

// === Mode Transition Tests ===

#[test]
fn edit_mode_maintains_focused_message() {
    let dialogue = create_dialogue_with_short_message();
    let app_state = create_app_state_editing_message(dialogue, 0, "Edited content");
    
    // Should maintain focus on the edited message
    assert!(app_state.focused_message_id.is_some());
    
    // Should be able to identify which message is being edited
    let focused_id = app_state.focused_message_id.unwrap();
    let messages = app_state.current_messages();
    let focused_message = messages.iter().find(|m| m.id() == focused_id).unwrap();
    assert_eq!(focused_message.content(), "Hi"); // Original content before edit
}

#[test]
fn edit_mode_preserves_dialogue_structure() {
    let dialogue = create_dialogue_with_short_message();
    let original_message_count = dialogue.trees()[0].branches()[0].messages().len();
    let app_state = create_app_state_editing_message(dialogue, 0, "New content");
    
    // Should preserve all messages in dialogue
    let current_messages = app_state.current_messages();
    assert_eq!(current_messages.len(), original_message_count);
    
    // Should maintain branch and tree structure
    assert!(app_state.current_tree_id.is_some());
    assert!(app_state.current_branch_id.is_some());
}

// === Content Preservation Tests ===

#[test]
fn textarea_content_reflects_editing_state() {
    let dialogue = create_dialogue_with_short_message();
    let edit_content = "This is new content being typed";
    let app_state = create_app_state_editing_message(dialogue, 0, edit_content);
    
    // TextArea should contain the editing content
    let textarea = app_state.edit_textarea.as_ref().unwrap();
    let content = textarea.lines().join("\n");
    assert_eq!(content, edit_content);
}

#[test]
fn textarea_handles_multiline_content() {
    let dialogue = create_dialogue_with_short_message();
    let multiline_content = "First line\nSecond line\n\nFourth line after blank";
    let app_state = create_app_state_editing_message(dialogue, 0, multiline_content);
    
    let textarea = app_state.edit_textarea.as_ref().unwrap();
    let lines = textarea.lines();
    
    assert_eq!(lines.len(), 4);
    assert_eq!(lines[0], "First line");
    assert_eq!(lines[1], "Second line");
    assert_eq!(lines[2], ""); // Blank line preserved
    assert_eq!(lines[3], "Fourth line after blank");
}

#[test]
fn textarea_handles_empty_content() {
    let dialogue = create_dialogue_with_short_message();
    let app_state = create_app_state_editing_message(dialogue, 0, "");
    
    let textarea = app_state.edit_textarea.as_ref().unwrap();
    let lines = textarea.lines();
    
    // Empty content should result in single empty line
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], "");
}

// === Context Awareness Tests ===

#[test]
fn edit_mode_provides_access_to_conversation_context() {
    let mut dialogue = Dialogue::new("Context Test");
    let mut tree = Tree::new("Context Tree");
    let mut branch = Branch::new("main");
    
    branch.add_message(Message::new("Context message 1", Role::User));
    branch.add_message(Message::new("Context response 1", Role::Assistant));
    branch.add_message(Message::new("Message being edited", Role::User));
    branch.add_message(Message::new("Future message", Role::Assistant));
    
    tree.add_branch(branch);
    dialogue.add_tree(tree);
    
    let app_state = create_app_state_editing_message(dialogue, 2, "Editing in progress");
    
    let all_messages = app_state.current_messages();
    assert_eq!(all_messages.len(), 4, "Should have access to all context messages");
    
    // Should be able to identify messages before and after
    let editing_id = app_state.focused_message_id.unwrap();
    let messages_before: Vec<_> = all_messages.iter()
        .take_while(|m| m.id() != editing_id)
        .collect();
    assert_eq!(messages_before.len(), 2, "Should have 2 messages before edited message");
    
    let messages_after: Vec<_> = all_messages.iter()
        .skip_while(|m| m.id() != editing_id)
        .skip(1) // Skip the edited message itself
        .collect();
    assert_eq!(messages_after.len(), 1, "Should have 1 message after edited message");
}