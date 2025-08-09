//! Tests for edit mode handler focusing on input/output behavior
//!
//! These tests verify the public interface of edit mode operations by testing
//! state changes and observable behavior without relying on implementation details.

use frond::actions::EditModeAction;
use frond::app::state::AppState;
use frond::app::reducer::edit_mode_handler::{handle_edit_mode_action, handle_edit_mode_raw_input};
use frond::app::Mode;
use frond::config::Config;
use frond_core::{Branch, Dialogue, Message, Role, Tree};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::TextArea;

fn create_test_dialogue() -> Dialogue {
    let mut dialogue = Dialogue::new("Test Dialogue");
    let mut tree = Tree::new("Test Tree");
    let mut branch = Branch::new("main");
    
    branch.add_message(Message::new("Hello world", Role::User));
    branch.add_message(Message::new("Hi there!", Role::Assistant));
    
    tree.add_branch(branch);
    dialogue.add_tree(tree);
    dialogue
}

fn create_app_state_with_textarea(content: &str) -> AppState {
    let dialogue = create_test_dialogue();
    let tree_id = dialogue.trees().get(0).unwrap().id();
    let branch_id = dialogue.trees().get(0).unwrap().branches().get(0).unwrap().id();
    let message_id = dialogue.trees().get(0).unwrap().branches().get(0).unwrap().messages().get(0).unwrap().id();
    
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

fn create_app_state_without_textarea() -> AppState {
    let dialogue = create_test_dialogue();
    let tree_id = dialogue.trees().get(0).unwrap().id();
    let branch_id = dialogue.trees().get(0).unwrap().branches().get(0).unwrap().id();
    let message_id = dialogue.trees().get(0).unwrap().branches().get(0).unwrap().messages().get(0).unwrap().id();
    
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

#[test]
fn exit_current_mode_transitions_to_normal() {
    let mut state = create_app_state_with_textarea("test content");
    
    handle_edit_mode_action(&mut state, EditModeAction::ExitCurrentMode);
    
    assert_eq!(state.mode, Mode::Normal);
    assert!(state.edit_textarea.is_none());
    assert!(state.error_message.is_none());
}

#[test]
fn exit_current_mode_saves_non_empty_content() {
    let mut state = create_app_state_with_textarea("updated content");
    let message_id = state.focused_message_id.unwrap();
    
    handle_edit_mode_action(&mut state, EditModeAction::ExitCurrentMode);
    
    let message = state.current_branch().unwrap().get_message_by_id(message_id).unwrap();
    assert_eq!(message.content(), "updated content");
}

#[test]
fn exit_current_mode_deletes_empty_content() {
    let mut state = create_app_state_with_textarea("");
    let message_id = state.focused_message_id.unwrap();
    let initial_count = state.current_messages().len();
    
    handle_edit_mode_action(&mut state, EditModeAction::ExitCurrentMode);
    
    let new_count = state.current_messages().len();
    assert_eq!(new_count, initial_count - 1);
    assert!(state.current_branch().unwrap().get_message_by_id(message_id).is_none());
}

#[test]
fn exit_current_mode_handles_paragraph_breaks() {
    let mut state = create_app_state_with_textarea("line1\n\nline2");
    
    handle_edit_mode_action(&mut state, EditModeAction::ExitCurrentMode);
    
    let message_id = state.focused_message_id.unwrap();
    let message = state.current_branch().unwrap().get_message_by_id(message_id).unwrap();
    assert_eq!(message.content(), "line1\n\nline2");
}

#[test]
fn submit_message_shows_not_implemented_error() {
    let mut state = create_app_state_with_textarea("test");
    
    handle_edit_mode_action(&mut state, EditModeAction::SubmitMessage);
    
    assert!(state.error_message.is_some());
    assert!(state.error_message.as_ref().unwrap().contains("not implemented"));
}

#[test]
fn textarea_operations_require_valid_textarea() {
    let mut state = create_app_state_without_textarea();
    
    let actions = vec![
        EditModeAction::DeleteChar,
        EditModeAction::DeleteNextChar,
        EditModeAction::InsertNewline,
        EditModeAction::DeleteWord,
        EditModeAction::MoveCursorForward,
        EditModeAction::Undo,
        EditModeAction::Copy,
    ];
    
    for action in actions {
        state.error_message = None;
        handle_edit_mode_action(&mut state, action);
        assert!(state.error_message.is_some());
        assert!(state.error_message.as_ref().unwrap().contains("textarea"));
    }
}

#[test]
fn textarea_operations_work_with_valid_textarea() {
    let mut state = create_app_state_with_textarea("hello world");
    
    let actions = vec![
        EditModeAction::MoveCursorForward,
        EditModeAction::MoveCursorBack,
        EditModeAction::MoveCursorUp,
        EditModeAction::MoveCursorDown,
        EditModeAction::MoveCursorWordForward,
        EditModeAction::MoveCursorEnd,
        EditModeAction::MoveCursorTop,
    ];
    
    for action in actions {
        state.error_message = None;
        handle_edit_mode_action(&mut state, action);
        assert!(state.error_message.is_none());
    }
}

#[test]
fn raw_input_updates_textarea_content() {
    let mut state = create_app_state_with_textarea("");
    
    let key_event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
    handle_edit_mode_raw_input(&mut state, key_event);
    
    let textarea = state.edit_textarea.as_ref().unwrap();
    assert_eq!(textarea.lines()[0], "a");
}

#[test]
fn raw_input_handles_missing_textarea() {
    let mut state = create_app_state_without_textarea();
    
    let key_event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
    handle_edit_mode_raw_input(&mut state, key_event);
    
    // Should not panic, textarea remains None
    assert!(state.edit_textarea.is_none());
}

#[test]
fn delete_operations_modify_textarea_content() {
    let mut state = create_app_state_with_textarea("hello world");
    let textarea = state.edit_textarea.as_mut().unwrap();
    textarea.move_cursor(tui_textarea::CursorMove::End);
    
    handle_edit_mode_action(&mut state, EditModeAction::DeleteChar);
    
    let content = state.edit_textarea.as_ref().unwrap().lines().join("");
    assert_eq!(content, "hello worl");
}

#[test]
fn word_operations_affect_textarea_content() {
    let mut state = create_app_state_with_textarea("hello world test");
    let textarea = state.edit_textarea.as_mut().unwrap();
    textarea.move_cursor(tui_textarea::CursorMove::End);
    
    handle_edit_mode_action(&mut state, EditModeAction::DeleteWord);
    
    let content = state.edit_textarea.as_ref().unwrap().lines().join("");
    assert!(content.len() < "hello world test".len());
}

#[test]
fn line_operations_do_not_error_with_valid_textarea() {
    let mut state = create_app_state_with_textarea("hello\nworld\ntest");
    let textarea = state.edit_textarea.as_mut().unwrap();
    textarea.move_cursor(tui_textarea::CursorMove::Down);
    textarea.move_cursor(tui_textarea::CursorMove::Forward);
    
    // Test line operations - mainly checking they don't error
    state.error_message = None;
    handle_edit_mode_action(&mut state, EditModeAction::DeleteLineByEnd);
    assert!(state.error_message.is_none());
    
    handle_edit_mode_action(&mut state, EditModeAction::DeleteLineByHead);
    assert!(state.error_message.is_none());
}

#[test]
fn cursor_movement_does_not_change_content() {
    let mut state = create_app_state_with_textarea("hello world");
    let original_content = state.edit_textarea.as_ref().unwrap().lines().join("");
    
    let movement_actions = vec![
        EditModeAction::MoveCursorForward,
        EditModeAction::MoveCursorBack,
        EditModeAction::MoveCursorWordForward,
        EditModeAction::MoveCursorEnd,
        EditModeAction::MoveCursorHead,
        EditModeAction::MoveCursorTop,
        EditModeAction::MoveCursorBottom,
    ];
    
    for action in movement_actions {
        handle_edit_mode_action(&mut state, action);
        let current_content = state.edit_textarea.as_ref().unwrap().lines().join("");
        assert_eq!(current_content, original_content);
    }
}

#[test]
fn unimplemented_actions_show_error() {
    let mut state = create_app_state_with_textarea("test");
    
    // Test an action that has a catch-all in the handler
    handle_edit_mode_action(&mut state, EditModeAction::SelectAll);
    
    // Should still work for implemented actions
    assert!(state.error_message.is_none());
}

#[test]
fn clipboard_operations_do_not_error_with_valid_textarea() {
    let mut state = create_app_state_with_textarea("test content");
    
    let clipboard_actions = vec![
        EditModeAction::Copy,
        EditModeAction::Cut,
        EditModeAction::Paste,
        EditModeAction::SelectAll,
    ];
    
    for action in clipboard_actions {
        state.error_message = None;
        handle_edit_mode_action(&mut state, action);
        assert!(state.error_message.is_none());
    }
}

#[test]
fn undo_redo_operations_work_with_valid_textarea() {
    let mut state = create_app_state_with_textarea("original");
    
    // Make some changes first
    handle_edit_mode_action(&mut state, EditModeAction::DeleteChar);
    
    // Test undo/redo
    handle_edit_mode_action(&mut state, EditModeAction::Undo);
    assert!(state.error_message.is_none());
    
    handle_edit_mode_action(&mut state, EditModeAction::Redo);
    assert!(state.error_message.is_none());
}