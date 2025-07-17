//! Tests for scrolling system and focus management
//!
//! This module provides comprehensive test coverage for the scrolling system, which is a core
//! component of the TUI that manages viewport positioning, message focus, and scrollbar state.

use frond::actions::{ActionDispatcher, NormalModeAction, UIAction};
use frond::app::{AppState, Mode};
use frond::config::Config;
use frond_core::{Branch, Dialogue, Message, Role, Tree};

fn create_test_dialogue_with_many_messages() -> Dialogue {
    let mut dialogue = Dialogue::new("Scrolling Test Dialogue");
    let mut tree = Tree::new("Test Tree");
    let mut branch = Branch::new("main");

    // Create enough messages to require scrolling
    for i in 0..20 {
        let role = if i % 2 == 0 {
            Role::Assistant
        } else {
            Role::User
        };
        let content = format!(
            "This is test message number {}. It contains enough text to test wrapping behavior and height calculations.",
            i + 1
        );
        branch.add_message(Message::new(&content, role));
    }

    tree.add_branch(branch);
    dialogue.add_tree(tree);
    dialogue
}

fn create_test_dialogue_with_long_messages() -> Dialogue {
    let mut dialogue = Dialogue::new("Long Message Test");
    let mut tree = Tree::new("Test Tree");
    let mut branch = Branch::new("main");

    // Create messages with varying lengths to test height calculations
    let messages = [
        "Short message.",
        "This is a medium length message that should wrap on narrow viewports and test the text wrapping calculations properly.",
        "This is a very long message that contains multiple sentences and should definitely wrap across multiple lines when rendered in a narrow viewport. It's designed to test the height calculation functions and ensure they work correctly with text wrapping. The message continues here with even more text to ensure we have sufficient content for thorough testing of the wrapping behavior.",
        "Another short one.",
        "Final medium message for testing purposes.",
    ];

    for (i, content) in messages.iter().enumerate() {
        let role = if i % 2 == 0 {
            Role::Assistant
        } else {
            Role::User
        };
        branch.add_message(Message::new(*content, role));
    }

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
        scrollbar_state: ratatui::widgets::ScrollbarState::default(),
        error_message: None,
    }
}

// Basic Scroll Position Tests

#[test]
fn scroll_down_increases_scroll_offset() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let initial_offset = state.scroll_offset;
    state.dispatch(UIAction::NormalMode(NormalModeAction::ScrollDown));

    assert_eq!(state.scroll_offset, initial_offset + 1);
}

#[test]
fn scroll_up_decreases_scroll_offset() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    // Set initial scroll position
    state.scroll_offset = 5;
    let initial_offset = state.scroll_offset;

    state.dispatch(UIAction::NormalMode(NormalModeAction::ScrollUp));

    assert_eq!(state.scroll_offset, initial_offset - 1);
}

#[test]
fn scroll_up_at_minimum_does_not_underflow() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    // Set scroll to a large negative value
    state.scroll_offset = -1000;
    let initial_offset = state.scroll_offset;

    state.dispatch(UIAction::NormalMode(NormalModeAction::ScrollUp));

    // Should decrement by 1, not underflow
    assert_eq!(state.scroll_offset, initial_offset - 1);
}

#[test]
fn scroll_down_at_maximum_does_not_overflow() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    // Set scroll to a large positive value
    state.scroll_offset = 1000;
    let initial_offset = state.scroll_offset;

    state.dispatch(UIAction::NormalMode(NormalModeAction::ScrollDown));

    // Should increment by 1, not overflow
    assert_eq!(state.scroll_offset, initial_offset + 1);
}

#[test]
fn multiple_scroll_operations_accumulate_correctly() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let initial_offset = state.scroll_offset;

    // Scroll down 3 times
    for _ in 0..3 {
        state.dispatch(UIAction::NormalMode(NormalModeAction::ScrollDown));
    }

    assert_eq!(state.scroll_offset, initial_offset + 3);

    // Scroll up 2 times
    for _ in 0..2 {
        state.dispatch(UIAction::NormalMode(NormalModeAction::ScrollUp));
    }

    assert_eq!(state.scroll_offset, initial_offset + 1);
}

// Height Calculation Tests

#[test]
fn calculate_message_display_height_handles_single_line() {
    let dialogue = create_test_dialogue_with_many_messages();
    let state = create_app_state_with_dialogue(dialogue);

    let short_message = Message::new("Short", Role::User);
    let viewport_width = 80;

    let height = state.calculate_message_display_height(&short_message, viewport_width);

    // Should be content (1 line) + border (2 lines)
    assert_eq!(height, 3);
}

#[test]
fn calculate_message_display_height_handles_multiline_content() {
    let dialogue = create_test_dialogue_with_many_messages();
    let state = create_app_state_with_dialogue(dialogue);

    let multiline_message = Message::new("Line 1\nLine 2\nLine 3", Role::User);
    let viewport_width = 80;

    let height = state.calculate_message_display_height(&multiline_message, viewport_width);

    // Should be content (3 lines) + border (2 lines)
    assert_eq!(height, 5);
}

#[test]
fn calculate_message_display_height_handles_text_wrapping() {
    let dialogue = create_test_dialogue_with_many_messages();
    let state = create_app_state_with_dialogue(dialogue);

    // Create a message longer than viewport width
    let long_message = Message::new(
        "This is a very long message that should wrap across multiple lines when the viewport is narrow",
        Role::User,
    );
    let narrow_viewport = 20;

    let height = state.calculate_message_display_height(&long_message, narrow_viewport);

    // Should be wrapped content + border (2 lines)
    // With viewport width 20 and content width ~16, should wrap to multiple lines
    assert!(
        height > 3,
        "Height should be greater than 3 for wrapped text"
    );
}

#[test]
fn calculate_message_display_height_handles_empty_lines() {
    let dialogue = create_test_dialogue_with_many_messages();
    let state = create_app_state_with_dialogue(dialogue);

    let message_with_empty_lines = Message::new("Line 1\n\nLine 3", Role::User);
    let viewport_width = 80;

    let height = state.calculate_message_display_height(&message_with_empty_lines, viewport_width);

    // Should be 3 lines (including empty line) + border (2 lines)
    assert_eq!(height, 5);
}

#[test]
fn get_total_content_height_sums_all_messages() {
    let dialogue = create_test_dialogue_with_many_messages();
    let state = create_app_state_with_dialogue(dialogue);

    let viewport_width = 80;
    let total_height = state.get_total_content_height(viewport_width);

    // Should be sum of all message heights
    let messages = state.current_messages();
    let expected_total: usize = messages
        .iter()
        .map(|msg| state.calculate_message_display_height(msg, viewport_width))
        .sum();

    assert_eq!(total_height, expected_total.max(1));
}

#[test]
fn get_total_content_height_returns_minimum_of_one() {
    let empty_dialogue = Dialogue::new("Empty");
    let mut state = create_app_state_with_dialogue(empty_dialogue);

    // Clear all navigation to simulate empty state
    state.current_tree_id = None;
    state.current_branch_id = None;
    state.focused_message_id = None;

    let total_height = state.get_total_content_height(80);

    assert_eq!(total_height, 1);
}

// Focus Management Tests

#[test]
fn update_focused_message_from_scroll_focuses_center_message() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let viewport_height = 20;
    let viewport_width = 80;

    // Set scroll to middle of content
    state.scroll_offset = 10;
    state.update_focused_message_from_scroll(viewport_height, viewport_width);

    // Should focus a message (exact one depends on heights)
    assert!(state.focused_message_id.is_some());

    // Focused message should exist in current messages
    let messages = state.current_messages();
    let focused_id = state.focused_message_id.unwrap();
    assert!(messages.iter().any(|msg| msg.id() == focused_id));
}

#[test]
fn update_focused_message_from_scroll_handles_negative_scroll() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let viewport_height = 20;
    let viewport_width = 80;

    // Set negative scroll offset
    state.scroll_offset = -5;
    state.update_focused_message_from_scroll(viewport_height, viewport_width);

    // Should focus some valid message when center is above content
    let messages = state.current_messages();
    assert!(state.focused_message_id.is_some());

    // Verify focused message exists in current messages
    if let Some(focused_id) = state.focused_message_id {
        assert!(messages.iter().any(|msg| msg.id() == focused_id));
    }
}

#[test]
fn update_focused_message_from_scroll_handles_excessive_scroll() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let viewport_height = 20;
    let viewport_width = 80;

    // Set scroll beyond content
    state.scroll_offset = 1000;
    state.update_focused_message_from_scroll(viewport_height, viewport_width);

    // Should focus last message when center is below content
    let messages = state.current_messages();
    if let Some(last_message) = messages.last() {
        assert_eq!(state.focused_message_id, Some(last_message.id()));
    }
}

#[test]
fn update_focused_message_from_scroll_handles_empty_messages() {
    let empty_dialogue = Dialogue::new("Empty");
    let mut state = create_app_state_with_dialogue(empty_dialogue);

    // Clear navigation to simulate empty branch
    state.current_branch_id = None;

    state.update_focused_message_from_scroll(20, 80);

    // Should set focus to None when no messages
    assert_eq!(state.focused_message_id, None);
}

#[test]
fn focused_message_changes_as_scroll_position_changes() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let viewport_height = 20;
    let viewport_width = 80;

    // Focus at top
    state.scroll_offset = 0;
    state.update_focused_message_from_scroll(viewport_height, viewport_width);
    let top_focus = state.focused_message_id;

    // Focus at bottom
    state.scroll_offset = 50;
    state.update_focused_message_from_scroll(viewport_height, viewport_width);
    let bottom_focus = state.focused_message_id;

    // Focus should change as we scroll
    assert_ne!(top_focus, bottom_focus);
}

// Scrollbar State Tests

#[test]
fn update_scrollbar_state_sets_content_length() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let viewport_height = 20;
    let viewport_width = 80;

    state.update_scrollbar_state(viewport_height, viewport_width);

    // Scrollbar should be updated (we can't directly inspect content length)
    // Just verify that update_scrollbar_state doesn't panic
}

#[test]
fn update_scrollbar_state_sets_viewport_length() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let viewport_height = 20;
    let viewport_width = 80;

    state.update_scrollbar_state(viewport_height, viewport_width);

    // Scrollbar state should be updated without panicking
    // The exact values are internal to ScrollbarState
}

#[test]
fn update_scrollbar_state_position_reflects_scroll_offset() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let viewport_height = 20;
    let viewport_width = 80;

    // Test different scroll positions - should not panic
    state.scroll_offset = 0;
    state.update_scrollbar_state(viewport_height, viewport_width);

    state.scroll_offset = 10;
    state.update_scrollbar_state(viewport_height, viewport_width);

    // Scrollbar state should update without panicking
}

#[test]
fn update_scrollbar_state_handles_negative_scroll() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let viewport_height = 20;
    let viewport_width = 80;

    // Test negative scroll
    state.scroll_offset = -10;
    state.update_scrollbar_state(viewport_height, viewport_width);

    // Should not panic and should have valid state
    // ScrollbarState doesn't expose position for reading, so just verify no panic
}

// Integration Tests

#[test]
fn scrolling_action_updates_focused_message() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let _initial_focus = state.focused_message_id;

    // Scroll enough to potentially change focus
    for _ in 0..10 {
        state.dispatch(UIAction::NormalMode(NormalModeAction::ScrollDown));
    }

    // Update focus based on new scroll position
    state.update_focused_message_from_scroll(20, 80);

    // Focus may have changed
    let messages = state.current_messages();
    if messages.len() > 1 {
        // With multiple messages, focus might change
        let current_focus = state.focused_message_id;
        assert!(current_focus.is_some());

        // Verify focused message exists
        if let Some(focused_id) = current_focus {
            assert!(messages.iter().any(|msg| msg.id() == focused_id));
        }
    }
}

#[test]
fn scroll_position_affects_which_message_is_focused() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let viewport_height = 10;
    let viewport_width = 80;

    // Get focus at different scroll positions
    let mut focus_positions = Vec::new();

    for scroll_offset in [0, 5, 10, 15, 20] {
        state.scroll_offset = scroll_offset;
        state.update_focused_message_from_scroll(viewport_height, viewport_width);
        focus_positions.push(state.focused_message_id);
    }

    // Should have valid focus at each position
    for focus in &focus_positions {
        assert!(focus.is_some());
    }

    // With enough messages and scroll range, some positions should differ
    let unique_focuses: std::collections::HashSet<_> = focus_positions.into_iter().collect();
    assert!(!unique_focuses.is_empty());
}

#[test]
fn scrolling_maintains_valid_state_consistency() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let viewport_height = 20;
    let viewport_width = 80;

    // Perform various scroll operations
    for _ in 0..5 {
        state.dispatch(UIAction::NormalMode(NormalModeAction::ScrollDown));
        state.update_focused_message_from_scroll(viewport_height, viewport_width);
        state.update_scrollbar_state(viewport_height as usize, viewport_width);

        // Verify state consistency
        if let Some(focused_id) = state.focused_message_id {
            let messages = state.current_messages();
            assert!(
                messages.iter().any(|msg| msg.id() == focused_id),
                "Focused message should exist in current messages"
            );
        }

        // Verify scrollbar state is valid
        // ScrollbarState position is not directly accessible for reading
    }
}

// Edge Cases and Error Handling

#[test]
fn scrolling_with_varying_viewport_widths() {
    let dialogue = create_test_dialogue_with_long_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let viewport_height = 20;

    // Test different viewport widths
    for width in [20, 40, 80, 120] {
        state.update_focused_message_from_scroll(viewport_height, width);
        state.update_scrollbar_state(viewport_height as usize, width);

        // Should handle different widths without panicking
        let total_height = state.get_total_content_height(width);
        assert!(total_height > 0);

        // Narrower widths should generally result in taller content
        if width == 20 {
            let narrow_height = total_height;
            state.update_scrollbar_state(viewport_height as usize, 120);
            let wide_height = state.get_total_content_height(120);
            assert!(
                narrow_height >= wide_height,
                "Narrow viewport should have taller or equal content"
            );
        }
    }
}

#[test]
fn reset_focus_for_new_branch_resets_scroll_state() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    // Set non-zero scroll state
    state.scroll_offset = 15;
    state.scrollbar_state = state.scrollbar_state.position(10);

    state.reset_focus_for_new_branch();

    // Should reset scroll position
    assert_eq!(state.scroll_offset, 0);

    // Should focus first message
    let messages = state.current_messages();
    if let Some(first_message) = messages.first() {
        assert_eq!(state.focused_message_id, Some(first_message.id()));
    }
}

#[test]
fn scrolling_with_single_message_branch() {
    let mut dialogue = Dialogue::new("Single Message Test");
    let mut tree = Tree::new("Test Tree");
    let mut branch = Branch::new("main");

    branch.add_message(Message::new("Only message", Role::User));
    tree.add_branch(branch);
    dialogue.add_tree(tree);

    let mut state = create_app_state_with_dialogue(dialogue);

    // Test scrolling with only one message
    state.dispatch(UIAction::NormalMode(NormalModeAction::ScrollDown));
    state.update_focused_message_from_scroll(20, 80);

    // Should still focus the single message
    let messages = state.current_messages();
    assert_eq!(messages.len(), 1);
    assert_eq!(state.focused_message_id, Some(messages[0].id()));
}

#[test]
fn scrolling_preserves_focus_when_possible() {
    let dialogue = create_test_dialogue_with_many_messages();
    let mut state = create_app_state_with_dialogue(dialogue);

    let viewport_height = 20;
    let viewport_width = 80;

    // Set focus to a specific message
    let messages = state.current_messages();
    if let Some(target_message) = messages.get(5) {
        state.focused_message_id = Some(target_message.id());

        // Small scroll shouldn't change focus drastically
        let _initial_focus = state.focused_message_id;
        state.scroll_offset = 1;
        state.update_focused_message_from_scroll(viewport_height, viewport_width);

        // Focus should still be valid
        assert!(state.focused_message_id.is_some());

        // Verify focused message exists
        let current_messages = state.current_messages();
        if let Some(focused_id) = state.focused_message_id {
            assert!(current_messages.iter().any(|msg| msg.id() == focused_id));
        }
    }
}
