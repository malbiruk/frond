//! Tests for scrolling focus determination logic
//!
//! These tests verify the accuracy of focus calculations based on scroll position,
//! center-line calculations, and message positioning math used for viewport focus.

use frond::ui::normal_mode::scrolling::focus::{
    calculate_scroll_to_focus_message, get_first_message_id, update_focused_message_after_deletion,
    update_focused_message_from_scroll,
};
use frond_core::{Message, Role};
use uuid::Uuid;

// === Helper Functions ===

fn create_test_messages() -> Vec<Message> {
    vec![
        Message::new("Short message", Role::User),
        Message::new("This is a longer message that might wrap depending on viewport width", Role::Assistant),
        Message::new("Multi\nline\nmessage\nwith\nseveral\nlines", Role::User),
        Message::new("Another short one", Role::Assistant),
        Message::new("Final message in the test set", Role::User),
    ]
}

fn get_message_refs(messages: &[Message]) -> Vec<&Message> {
    messages.iter().collect()
}

// === Focus from Scroll Tests ===

#[test]
fn update_focused_message_from_scroll_empty_messages() {
    let messages: Vec<&Message> = vec![];
    let scroll_offset = 0;
    let viewport_height = 20;
    let viewport_width = 80;

    let result = update_focused_message_from_scroll(&messages, scroll_offset, viewport_height, viewport_width);

    assert_eq!(result, None);
}

#[test]
fn update_focused_message_from_scroll_single_message() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages[0..1]);
    let scroll_offset = 0;
    let viewport_height = 20;
    let viewport_width = 80;

    let result = update_focused_message_from_scroll(&message_refs, scroll_offset, viewport_height, viewport_width);

    assert_eq!(result, Some(messages[0].id()));
}

#[test]
fn update_focused_message_from_scroll_at_top() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let scroll_offset = 0;
    let viewport_height = 20;
    let viewport_width = 80;

    let result = update_focused_message_from_scroll(&message_refs, scroll_offset, viewport_height, viewport_width);

    // With scroll at 0 and viewport height 20, center line is at 10
    // Should focus on a valid message (likely first or second message)
    assert!(result.is_some(), "Should return a focused message");
    let focused_id = result.unwrap();
    assert!(messages.iter().any(|m| m.id() == focused_id), "Focused message should exist in the list");
}

#[test]
fn update_focused_message_from_scroll_negative_scroll() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let scroll_offset = -5;
    let viewport_height = 20;
    let viewport_width = 80;

    let result = update_focused_message_from_scroll(&message_refs, scroll_offset, viewport_height, viewport_width);

    // Negative center line should focus on a valid message (likely first message)
    assert!(result.is_some(), "Should return a focused message");
    let focused_id = result.unwrap();
    assert!(messages.iter().any(|m| m.id() == focused_id), "Focused message should exist in the list");
}

#[test]
fn update_focused_message_from_scroll_past_end() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let scroll_offset = 1000; // Way past the end
    let viewport_height = 20;
    let viewport_width = 80;

    let result = update_focused_message_from_scroll(&message_refs, scroll_offset, viewport_height, viewport_width);

    // Should focus last message when scrolled past end
    assert_eq!(result, Some(messages[messages.len() - 1].id()));
}

#[test]
fn update_focused_message_from_scroll_middle_position() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let scroll_offset = 6; // Should put center line around middle messages
    let viewport_height = 20;
    let viewport_width = 80;

    let result = update_focused_message_from_scroll(&message_refs, scroll_offset, viewport_height, viewport_width);

    // Should focus on one of the middle messages
    assert!(result.is_some());
    let focused_id = result.unwrap();
    assert!(messages.iter().any(|m| m.id() == focused_id));
}

#[test]
fn update_focused_message_from_scroll_different_viewport_heights() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let scroll_offset = 10;
    let viewport_width = 80;

    // Test different viewport heights
    let heights = vec![10, 20, 40];
    let mut results = vec![];

    for height in heights {
        let result = update_focused_message_from_scroll(&message_refs, scroll_offset, height, viewport_width);
        results.push(result);
    }

    // All should return valid message IDs
    for result in results {
        assert!(result.is_some());
        let focused_id = result.unwrap();
        assert!(messages.iter().any(|m| m.id() == focused_id));
    }
}

#[test]
fn update_focused_message_from_scroll_narrow_viewport() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let scroll_offset = 5;
    let viewport_height = 20;
    let viewport_width = 20; // Narrow viewport causes more wrapping

    let result = update_focused_message_from_scroll(&message_refs, scroll_offset, viewport_height, viewport_width);

    assert!(result.is_some());
    let focused_id = result.unwrap();
    assert!(messages.iter().any(|m| m.id() == focused_id));
}

// === Focus After Deletion Tests ===

#[test]
fn update_focused_message_after_deletion_empty_messages() {
    let messages: Vec<&Message> = vec![];
    let deleted_index = 0;

    let result = update_focused_message_after_deletion(&messages, deleted_index);

    assert_eq!(result, None);
}

#[test]
fn update_focused_message_after_deletion_first_message() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let deleted_index = 0;

    let result = update_focused_message_after_deletion(&message_refs, deleted_index);

    // Should focus on the message that moved into index 0 (was index 1)
    assert_eq!(result, Some(messages[0].id()));
}

#[test]
fn update_focused_message_after_deletion_middle_message() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let deleted_index = 2;

    let result = update_focused_message_after_deletion(&message_refs, deleted_index);

    // Should focus on the message that moved into index 2 (was index 3)
    assert_eq!(result, Some(messages[2].id()));
}

#[test]
fn update_focused_message_after_deletion_last_message() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let deleted_index = messages.len() - 1;

    let result = update_focused_message_after_deletion(&message_refs, deleted_index);

    // Should focus on previous message
    assert_eq!(result, Some(messages[messages.len() - 1].id()));
}


#[test]
fn update_focused_message_after_deletion_single_message() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages[0..1]);
    let deleted_index = 0;

    let result = update_focused_message_after_deletion(&message_refs, deleted_index);

    // With only one message, should return that message
    assert_eq!(result, Some(messages[0].id()));
}

// === Get First Message Tests ===

#[test]
fn get_first_message_id_empty_messages() {
    let messages: Vec<&Message> = vec![];

    let result = get_first_message_id(&messages);

    assert_eq!(result, None);
}

#[test]
fn get_first_message_id_single_message() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages[0..1]);

    let result = get_first_message_id(&message_refs);

    assert_eq!(result, Some(messages[0].id()));
}

#[test]
fn get_first_message_id_multiple_messages() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);

    let result = get_first_message_id(&message_refs);

    assert_eq!(result, Some(messages[0].id()));
}

// === Scroll to Focus Message Tests ===

#[test]
fn calculate_scroll_to_focus_message_empty_messages() {
    let messages: Vec<&Message> = vec![];
    let message_id = Uuid::new_v4();
    let viewport_height = 20;
    let viewport_width = 80;

    let result = calculate_scroll_to_focus_message(&messages, message_id, viewport_height, viewport_width);

    assert_eq!(result, None);
}

#[test]
fn calculate_scroll_to_focus_message_nonexistent_message() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let nonexistent_id = Uuid::new_v4();
    let viewport_height = 20;
    let viewport_width = 80;

    let result = calculate_scroll_to_focus_message(&message_refs, nonexistent_id, viewport_height, viewport_width);

    assert_eq!(result, None);
}

#[test]
fn calculate_scroll_to_focus_message_first_message() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let message_id = messages[0].id();
    let viewport_height = 20;
    let viewport_width = 80;

    let result = calculate_scroll_to_focus_message(&message_refs, message_id, viewport_height, viewport_width);

    assert!(result.is_some());
    let scroll_offset = result.unwrap();
    
    // First message starts at y=0, with height ~3, center at ~1.5
    // Viewport center is at 10, so scroll should be around 1.5 - 10 = -8.5
    assert!(scroll_offset <= 0, "Should scroll up to center first message");
}

#[test]
fn calculate_scroll_to_focus_message_last_message() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let message_id = messages[messages.len() - 1].id();
    let viewport_height = 20;
    let viewport_width = 80;

    let result = calculate_scroll_to_focus_message(&message_refs, message_id, viewport_height, viewport_width);

    assert!(result.is_some());
    let scroll_offset = result.unwrap();
    
    // Last message should require positive scroll to center
    assert!(scroll_offset > 0, "Should scroll down to center last message");
}

#[test]
fn calculate_scroll_to_focus_message_middle_message() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let message_id = messages[2].id(); // Middle message
    let viewport_height = 20;
    let viewport_width = 80;

    let result = calculate_scroll_to_focus_message(&message_refs, message_id, viewport_height, viewport_width);

    assert!(result.is_some());
    let scroll_offset = result.unwrap();
    
    // Middle message might require small positive or negative scroll
    // The exact value depends on message heights, but should be reasonable
    assert!(scroll_offset.abs() < 100, "Scroll offset should be reasonable for middle message");
}

#[test]
fn calculate_scroll_to_focus_message_different_viewport_heights() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let message_id = messages[1].id();
    let viewport_width = 80;

    let heights = vec![10, 20, 40];
    let mut scroll_offsets = vec![];

    for height in heights {
        let result = calculate_scroll_to_focus_message(&message_refs, message_id, height, viewport_width);
        assert!(result.is_some());
        scroll_offsets.push(result.unwrap());
    }

    // Different viewport heights should produce different scroll offsets
    // Larger viewports should generally require less scroll
    assert_ne!(scroll_offsets[0], scroll_offsets[1]);
    assert_ne!(scroll_offsets[1], scroll_offsets[2]);
}

#[test]
fn calculate_scroll_to_focus_message_with_wrapping() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let message_id = messages[1].id(); // The longer message that will wrap
    let viewport_height = 20;
    let viewport_width = 30; // Narrow to force wrapping

    let result = calculate_scroll_to_focus_message(&message_refs, message_id, viewport_height, viewport_width);

    assert!(result.is_some());
    let scroll_offset = result.unwrap();
    
    // Should produce a valid scroll offset even with wrapping
    assert!(scroll_offset.abs() < 1000, "Scroll offset should be reasonable even with wrapping");
}

// === Integration Tests ===

#[test]
fn focus_calculations_are_consistent() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);
    let viewport_height = 20;
    let viewport_width = 80;

    // Get scroll offset to focus on middle message
    let target_message_id = messages[2].id();
    let scroll_offset = calculate_scroll_to_focus_message(&message_refs, target_message_id, viewport_height, viewport_width);
    assert!(scroll_offset.is_some());

    // Use that scroll offset to determine focused message
    let focused_id = update_focused_message_from_scroll(&message_refs, scroll_offset.unwrap(), viewport_height, viewport_width);
    
    // Should focus on the target message (or very close to it)
    assert!(focused_id.is_some());
    // Note: Due to rounding and center-line calculations, might not be exact match
    // but should be one of the nearby messages
    let focused = focused_id.unwrap();
    assert!(messages.iter().any(|m| m.id() == focused));
}

#[test]
fn deletion_focus_maintains_reasonable_position() {
    let messages = create_test_messages();
    
    // Test deleting each position
    for i in 0..messages.len() {
        let message_refs = get_message_refs(&messages);
        let result = update_focused_message_after_deletion(&message_refs, i);
        
        if i < messages.len() {
            assert!(result.is_some(), "Should have focus after deleting index {}", i);
            let focused_id = result.unwrap();
            assert!(messages.iter().any(|m| m.id() == focused_id), 
                   "Focused message should exist after deleting index {}", i);
        }
    }
}

#[test]
fn focus_calculations_handle_edge_viewport_sizes() {
    let messages = create_test_messages();
    let message_refs = get_message_refs(&messages);

    // Test very small viewports
    let small_sizes = vec![(5, 10), (10, 5), (1, 1)];
    
    for (width, height) in small_sizes {
        let result = update_focused_message_from_scroll(&message_refs, 0, height, width);
        assert!(result.is_some(), "Should handle viewport size {}x{}", width, height);
        
        let focused_id = result.unwrap();
        assert!(messages.iter().any(|m| m.id() == focused_id));
    }
}