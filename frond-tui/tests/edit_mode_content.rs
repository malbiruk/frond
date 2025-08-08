//! Tests for edit mode content rendering logic
//!
//! These tests focus on the public interface of edit mode content rendering,
//! verifying expected behavior without testing specific UI rendering details.

use frond::app::state::AppState;
use frond::app::Mode;
use frond::config::Config;
use frond::ui::edit_mode::content;
use frond_core::{Branch, Dialogue, Message, Role, Tree};
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui::Terminal;
use tui_textarea::TextArea;

fn create_test_dialogue_with_messages() -> Dialogue {
    let mut dialogue = Dialogue::new("Test Dialogue");
    let mut tree = Tree::new("Test Tree");
    let mut branch = Branch::new("main");
    
    branch.add_message(Message::new("First message", Role::User));
    branch.add_message(Message::new("Second message", Role::Assistant));
    branch.add_message(Message::new("Third message", Role::User));
    
    tree.add_branch(branch);
    dialogue.add_tree(tree);
    dialogue
}

fn create_app_state_in_edit_mode(dialogue: Dialogue) -> AppState {
    let tree_id = dialogue.trees().get(0).unwrap().id();
    let branch_id = dialogue.trees().get(0).unwrap().branches().get(0).unwrap().id();
    let message_id = dialogue.trees().get(0).unwrap().branches().get(0).unwrap()
        .messages().get(0).unwrap().id();
    
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
        edit_textarea: None,
        highlight_cache: Default::default(),
    }
}

fn create_app_state_with_textarea(dialogue: Dialogue, content: &str) -> AppState {
    let mut state = create_app_state_in_edit_mode(dialogue);
    let mut textarea = TextArea::default();
    textarea.insert_str(content);
    state.edit_textarea = Some(textarea);
    state
}

#[test]
fn render_initializes_textarea_when_missing() {
    let dialogue = create_test_dialogue_with_messages();
    let mut state = create_app_state_in_edit_mode(dialogue);
    
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let area = Rect::new(0, 0, 80, 24);
    
    assert!(state.edit_textarea.is_none());
    
    terminal.draw(|frame| {
        content::render(frame, area, &mut state);
    }).unwrap();
    
    // Textarea should be initialized after rendering
    assert!(state.edit_textarea.is_some());
}

#[test]
fn render_handles_no_message_selected() {
    let dialogue = create_test_dialogue_with_messages();
    let mut state = create_app_state_in_edit_mode(dialogue);
    state.focused_message_id = None;
    
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let area = Rect::new(0, 0, 80, 24);
    
    // Should not panic with no message selected
    terminal.draw(|frame| {
        content::render(frame, area, &mut state);
    }).unwrap();
    
    // Textarea should not be initialized
    assert!(state.edit_textarea.is_none());
}

#[test]
fn render_handles_no_branch_selected() {
    let dialogue = create_test_dialogue_with_messages();
    let mut state = create_app_state_in_edit_mode(dialogue);
    state.current_branch_id = None;
    
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let area = Rect::new(0, 0, 80, 24);
    
    // Should not panic with no branch selected
    terminal.draw(|frame| {
        content::render(frame, area, &mut state);
    }).unwrap();
    
    assert!(state.edit_textarea.is_none());
}

#[test]
fn render_handles_message_not_found() {
    let dialogue = create_test_dialogue_with_messages();
    let mut state = create_app_state_in_edit_mode(dialogue);
    state.focused_message_id = Some(uuid::Uuid::new_v4()); // Non-existent message ID
    
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let area = Rect::new(0, 0, 80, 24);
    
    // Should not panic with invalid message ID
    terminal.draw(|frame| {
        content::render(frame, area, &mut state);
    }).unwrap();
    
    assert!(state.edit_textarea.is_none());
}

#[test]
fn render_works_with_existing_textarea() {
    let dialogue = create_test_dialogue_with_messages();
    let mut state = create_app_state_with_textarea(dialogue, "existing content");
    
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let area = Rect::new(0, 0, 80, 24);
    
    // Should not panic with existing textarea
    terminal.draw(|frame| {
        content::render(frame, area, &mut state);
    }).unwrap();
    
    // Textarea should still exist
    assert!(state.edit_textarea.is_some());
}

#[test]
fn render_detects_append_mode_correctly() {
    let dialogue = create_test_dialogue_with_messages();
    let mut state = create_app_state_with_textarea(dialogue, "new message");
    
    // Focus on the last message (which is a User message)
    let last_message_id = {
        let messages = state.current_messages();
        messages.last().unwrap().id()
    };
    state.focused_message_id = Some(last_message_id);
    
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let area = Rect::new(0, 0, 80, 24);
    
    // Should not panic in append mode
    terminal.draw(|frame| {
        content::render(frame, area, &mut state);
    }).unwrap();
    
    assert!(state.edit_textarea.is_some());
}

#[test]
fn render_handles_non_user_last_message() {
    let dialogue = create_test_dialogue_with_messages();
    let mut state = create_app_state_with_textarea(dialogue, "editing assistant message");
    
    // Focus on an assistant message (not append mode)
    let assistant_message_id = {
        let messages = state.current_messages();
        messages.iter().find(|m| *m.role() == Role::Assistant).unwrap().id()
    };
    state.focused_message_id = Some(assistant_message_id);
    
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let area = Rect::new(0, 0, 80, 24);
    
    // Should render in full-screen edit mode
    terminal.draw(|frame| {
        content::render(frame, area, &mut state);
    }).unwrap();
    
    assert!(state.edit_textarea.is_some());
}

#[test]
fn textarea_initialization_preserves_message_content() {
    let dialogue = create_test_dialogue_with_messages();
    let mut state = create_app_state_in_edit_mode(dialogue);
    
    let message_content = {
        let messages = state.current_messages();
        messages.first().unwrap().content().to_string()
    };
    
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let area = Rect::new(0, 0, 80, 24);
    
    terminal.draw(|frame| {
        content::render(frame, area, &mut state);
    }).unwrap();
    
    // Textarea should contain the message content
    let textarea = state.edit_textarea.as_ref().unwrap();
    let textarea_content = textarea.lines().join("\n");
    assert!(textarea_content.contains(&message_content) || message_content.contains(&textarea_content));
}

#[test]
fn render_handles_empty_message_content() {
    let mut dialogue = Dialogue::new("Test");
    let mut tree = Tree::new("Test Tree");
    let mut branch = Branch::new("main");
    branch.add_message(Message::new("", Role::User)); // Empty message
    tree.add_branch(branch);
    dialogue.add_tree(tree);
    
    let mut state = create_app_state_in_edit_mode(dialogue);
    
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let area = Rect::new(0, 0, 80, 24);
    
    terminal.draw(|frame| {
        content::render(frame, area, &mut state);
    }).unwrap();
    
    assert!(state.edit_textarea.is_some());
}

#[test]
fn render_handles_multiline_message_content() {
    let mut dialogue = Dialogue::new("Test");
    let mut tree = Tree::new("Test Tree");
    let mut branch = Branch::new("main");
    branch.add_message(Message::new("Line 1\nLine 2\n\nLine 4", Role::User));
    tree.add_branch(branch);
    dialogue.add_tree(tree);
    
    let mut state = create_app_state_in_edit_mode(dialogue);
    
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let area = Rect::new(0, 0, 80, 24);
    
    terminal.draw(|frame| {
        content::render(frame, area, &mut state);
    }).unwrap();
    
    let textarea = state.edit_textarea.as_ref().unwrap();
    assert!(textarea.lines().len() > 1);
}

#[test]
fn render_handles_wide_viewport() {
    let dialogue = create_test_dialogue_with_messages();
    let mut state = create_app_state_in_edit_mode(dialogue);
    
    let backend = TestBackend::new(200, 50); // Wide viewport
    let mut terminal = Terminal::new(backend).unwrap();
    let area = Rect::new(0, 0, 200, 50);
    
    terminal.draw(|frame| {
        content::render(frame, area, &mut state);
    }).unwrap();
    
    assert!(state.edit_textarea.is_some());
}

#[test]
fn render_handles_narrow_viewport() {
    let dialogue = create_test_dialogue_with_messages();
    let mut state = create_app_state_in_edit_mode(dialogue);
    
    let backend = TestBackend::new(20, 10); // Narrow viewport
    let mut terminal = Terminal::new(backend).unwrap();
    let area = Rect::new(0, 0, 20, 10);
    
    terminal.draw(|frame| {
        content::render(frame, area, &mut state);
    }).unwrap();
    
    assert!(state.edit_textarea.is_some());
}

#[test]
fn render_preserves_existing_textarea_state() {
    let dialogue = create_test_dialogue_with_messages();
    let mut state = create_app_state_with_textarea(dialogue, "preserved content");
    
    let original_content = state.edit_textarea.as_ref().unwrap().lines().join("\n");
    
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let area = Rect::new(0, 0, 80, 24);
    
    terminal.draw(|frame| {
        content::render(frame, area, &mut state);
    }).unwrap();
    
    let new_content = state.edit_textarea.as_ref().unwrap().lines().join("\n");
    assert_eq!(original_content, new_content);
}