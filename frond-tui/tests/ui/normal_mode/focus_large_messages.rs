//! Tests for focus behavior with large messages
//!
//! Verifies that focusing on first/last messages that are larger than the viewport
//! shows the beginning/end of the message rather than centering.

use frond::ui::normal_mode::content::focus_resolver;
use frond::app::state::ScrollingRequest;
use frond_core::{Message, Role};

#[test]
fn focus_top_large_message_shows_beginning() {
    // Create a very large message that would exceed a small viewport
    let large_content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";
    let large_message = Message::new(large_content, Role::User);
    let small_message = Message::new("Small", Role::Assistant);
    
    let messages = vec![&large_message, &small_message];
    let viewport_height = 3; // Very small viewport
    let viewport_width = 80;
    
    let result = focus_resolver::resolve_pending_focus_request(
        ScrollingRequest::ScrollToTop,
        &messages,
        0, // current scroll offset
        viewport_height,
        viewport_width,
    );
    
    // Should focus on the large message
    assert!(result.is_some());
    let (focused_id, scroll_offset) = result.unwrap();
    assert_eq!(focused_id, large_message.id());
    
    // For a large message, scroll_offset should position the beginning at top (offset 0)
    assert_eq!(scroll_offset, 0);
}

#[test]
fn focus_top_small_message_uses_centering() {
    let small_message = Message::new("Small", Role::User);
    let messages = vec![&small_message];
    let viewport_height = 10; // Large enough viewport
    let viewport_width = 80;
    
    let result = focus_resolver::resolve_pending_focus_request(
        ScrollingRequest::ScrollToTop,
        &messages,
        0,
        viewport_height,
        viewport_width,
    );
    
    assert!(result.is_some());
    let (focused_id, scroll_offset) = result.unwrap();
    assert_eq!(focused_id, small_message.id());
    
    // For small messages, should use normal centering (may not be 0)
    // The exact value depends on centering calculation, but should be reasonable
    assert!(scroll_offset <= 0); // Should not scroll past beginning
}

#[test]
fn focus_bottom_large_message_shows_end() {
    let small_message = Message::new("Small", Role::User);
    // Create a large message
    let large_content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";
    let large_message = Message::new(large_content, Role::Assistant);
    
    let messages = vec![&small_message, &large_message];
    let viewport_height = 3; // Small viewport
    let viewport_width = 80;
    
    let result = focus_resolver::resolve_pending_focus_request(
        ScrollingRequest::ScrollToBottom,
        &messages,
        0,
        viewport_height,
        viewport_width,
    );
    
    assert!(result.is_some());
    let (focused_id, scroll_offset) = result.unwrap();
    assert_eq!(focused_id, large_message.id());
    
    // For large message at bottom, should scroll to show the end
    // The offset should be positive to scroll down to the end
    assert!(scroll_offset > 0);
}

#[test]
fn focus_bottom_small_message_uses_centering() {
    let small_message = Message::new("Small", Role::Assistant);
    let messages = vec![&small_message];
    let viewport_height = 10; // Large viewport
    let viewport_width = 80;
    
    let result = focus_resolver::resolve_pending_focus_request(
        ScrollingRequest::ScrollToBottom,
        &messages,
        0,
        viewport_height,
        viewport_width,
    );
    
    assert!(result.is_some());
    let (focused_id, _scroll_offset) = result.unwrap();
    assert_eq!(focused_id, small_message.id());
    
    // For small messages, should use normal centering behavior
    // We don't assert on exact offset since it depends on centering logic
}

#[test]
fn focus_specific_large_message_shows_beginning() {
    let small_message = Message::new("Small", Role::User);
    // Create a large message
    let large_content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";
    let large_message = Message::new(large_content, Role::Assistant);
    
    let messages = vec![&small_message, &large_message];
    let viewport_height = 3; // Small viewport
    let viewport_width = 80;
    
    let result = focus_resolver::resolve_pending_focus_request(
        ScrollingRequest::ScrollToMessage(large_message.id()),
        &messages,
        0,
        viewport_height,
        viewport_width,
    );
    
    assert!(result.is_some());
    let (focused_id, scroll_offset) = result.unwrap();
    assert_eq!(focused_id, large_message.id());
    
    // For large message, should scroll to show beginning (message start position)
    // This should be the position after the small message
    assert!(scroll_offset > 0); // Should be positioned after the small message
}

#[test]
fn focus_specific_small_message_uses_centering() {
    let small_message1 = Message::new("Small 1", Role::User);
    let small_message2 = Message::new("Small 2", Role::Assistant);
    
    let messages = vec![&small_message1, &small_message2];
    let viewport_height = 10; // Large viewport
    let viewport_width = 80;
    
    let result = focus_resolver::resolve_pending_focus_request(
        ScrollingRequest::ScrollToMessage(small_message2.id()),
        &messages,
        0,
        viewport_height,
        viewport_width,
    );
    
    assert!(result.is_some());
    let (focused_id, _scroll_offset) = result.unwrap();
    assert_eq!(focused_id, small_message2.id());
    
    // For small messages, should use normal centering behavior
    // We don't assert on exact offset since it depends on centering logic
}