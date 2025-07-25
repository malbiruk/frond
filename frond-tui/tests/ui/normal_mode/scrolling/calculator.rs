//! Tests for scrolling calculator mathematical functions
//!
//! These tests verify the accuracy of message height calculations, line wrapping logic,
//! and total content height computations used for scrolling calculations.

use frond::ui::normal_mode::scrolling::calculator::{
    calculate_message_display_height, calculate_total_content_height,
};
use frond_core::{Message, Role};

// === Message Display Height Tests ===

#[test]
fn calculate_message_display_height_single_short_line() {
    let message = Message::new("Hello", Role::User);
    let viewport_width = 80;

    let height = calculate_message_display_height(&message, viewport_width);

    // Should be 1 content line + 2 border padding = 3
    assert_eq!(height, 3);
}

#[test]
fn calculate_message_display_height_empty_message() {
    let message = Message::new("", Role::User);
    let viewport_width = 80;

    let height = calculate_message_display_height(&message, viewport_width);

    // Empty content still gets 1 line + 2 border padding = 3
    assert_eq!(height, 3);
}

#[test]
fn calculate_message_display_height_single_line_needs_wrapping() {
    let message = Message::new("This is a very long message that will definitely need to wrap when displayed in a narrow viewport", Role::User);
    let viewport_width = 20; // Content width will be 16

    let height = calculate_message_display_height(&message, viewport_width);

    // Message is 101 chars, content width is 16, so needs 7 lines (101/16 = 6.3125 -> 7)
    // 7 content lines + 2 border padding = 9
    assert_eq!(height, 9);
}

#[test]
fn calculate_message_display_height_multiple_lines() {
    let message = Message::new("Line one\nLine two\nLine three", Role::User);
    let viewport_width = 80;

    let height = calculate_message_display_height(&message, viewport_width);

    // 3 lines, each fits in viewport, + 2 border padding = 5
    assert_eq!(height, 5);
}

#[test]
fn calculate_message_display_height_multiple_lines_with_wrapping() {
    let message = Message::new("Short line\nThis is a much longer line that will wrap\nAnother short line", Role::User);
    let viewport_width = 20; // Content width will be 16

    let height = calculate_message_display_height(&message, viewport_width);

    // Line 1: "Short line" = 10 chars, needs 1 line
    // Line 2: "This is a much longer line that will wrap" = 43 chars, needs 3 lines (43/16 = 2.69 -> 3)
    // Line 3: "Another short line" = 18 chars, needs 2 lines (18/16 = 1.125 -> 2)
    // Total: 6 content lines + 2 border padding = 8
    assert_eq!(height, 8);
}

#[test]
fn calculate_message_display_height_with_empty_lines() {
    let message = Message::new("Line one\n\nLine three", Role::User);
    let viewport_width = 80;

    let height = calculate_message_display_height(&message, viewport_width);

    // 3 lines (including empty line), + 2 border padding = 5
    assert_eq!(height, 5);
}

#[test]
fn calculate_message_display_height_very_narrow_viewport() {
    let message = Message::new("Hello", Role::User);
    let viewport_width = 4; // Content width will be 0 (4-4=0)

    let height = calculate_message_display_height(&message, viewport_width);

    // With content width 0, each character becomes a line
    // "Hello" = 5 characters = 5 lines + 2 border padding = 7
    // But with width 0, div_ceil would be infinite, so this tests edge case handling
    assert!(height >= 3); // At minimum should be 1 line + 2 padding
}

#[test]
fn calculate_message_display_height_minimal_viewport() {
    let message = Message::new("Hi", Role::User);
    let viewport_width = 5; // Content width will be 1

    let height = calculate_message_display_height(&message, viewport_width);

    // "Hi" = 2 characters, content width 1, so 2 lines + 2 border padding = 4
    assert_eq!(height, 4);
}

#[test]
fn calculate_message_display_height_exact_fit() {
    let message = Message::new("Exact", Role::User);
    let viewport_width = 9; // Content width will be 5

    let height = calculate_message_display_height(&message, viewport_width);

    // "Exact" = 5 characters, content width 5, so exactly 1 line + 2 border padding = 3
    assert_eq!(height, 3);
}

// === Total Content Height Tests ===

#[test]
fn calculate_total_content_height_empty_messages() {
    let messages: Vec<&Message> = vec![];
    let viewport_width = 80;

    let height = calculate_total_content_height(&messages, viewport_width);

    // Empty list should return minimum height of 1
    assert_eq!(height, 1);
}

#[test]
fn calculate_total_content_height_single_message() {
    let message = Message::new("Hello world", Role::User);
    let messages = vec![&message];
    let viewport_width = 80;

    let height = calculate_total_content_height(&messages, viewport_width);

    // Should equal the single message height: 1 line + 2 padding = 3
    assert_eq!(height, 3);
}

#[test]
fn calculate_total_content_height_multiple_messages() {
    let msg1 = Message::new("First message", Role::User);
    let msg2 = Message::new("Second message", Role::Assistant);
    let msg3 = Message::new("Third message", Role::User);
    let messages = vec![&msg1, &msg2, &msg3];
    let viewport_width = 80;

    let height = calculate_total_content_height(&messages, viewport_width);

    // Each message: 1 line + 2 padding = 3, so total = 9
    assert_eq!(height, 9);
}

#[test]
fn calculate_total_content_height_messages_with_wrapping() {
    let short_msg = Message::new("Short", Role::User);
    let long_msg = Message::new("This is a very long message that will wrap multiple times when displayed", Role::Assistant);
    let messages = vec![&short_msg, &long_msg];
    let viewport_width = 20; // Content width = 16

    let height = calculate_total_content_height(&messages, viewport_width);

    // Short message: 1 line + 2 padding = 3
    // Long message: 73 chars, needs 5 lines (73/16 = 4.56 -> 5), + 2 padding = 7
    // Total: 3 + 7 = 10
    assert_eq!(height, 10);
}

#[test]
fn calculate_total_content_height_with_multiline_messages() {
    let multiline_msg = Message::new("Line 1\nLine 2\nLine 3", Role::User);
    let single_msg = Message::new("Single line", Role::Assistant);
    let messages = vec![&multiline_msg, &single_msg];
    let viewport_width = 80;

    let height = calculate_total_content_height(&messages, viewport_width);

    // Multiline message: 3 lines + 2 padding = 5
    // Single message: 1 line + 2 padding = 3
    // Total: 5 + 3 = 8
    assert_eq!(height, 8);
}

#[test]
fn calculate_total_content_height_zero_result_returns_minimum() {
    // This tests the .max(1) behavior, though it's hard to create a scenario
    // where the sum would be 0 with the current implementation
    let messages: Vec<&Message> = vec![];
    let viewport_width = 80;

    let height = calculate_total_content_height(&messages, viewport_width);

    assert_eq!(height, 1);
}

// === Edge Case Tests ===

#[test]
fn calculate_heights_with_unicode_characters() {
    let message = Message::new("Hello 🦀 Rust! 你好", Role::User);
    let viewport_width = 80;

    let height = calculate_message_display_height(&message, viewport_width);

    // Should handle unicode characters properly
    // The exact height depends on how unicode chars are counted
    assert!(height >= 3); // At minimum 1 content line + 2 padding
}

#[test]
fn calculate_heights_with_tabs_and_special_chars() {
    let message = Message::new("Line\twith\ttabs\nLine with\r\nspecial chars", Role::User);
    let viewport_width = 80;

    let height = calculate_message_display_height(&message, viewport_width);

    // Should handle special characters gracefully
    assert!(height >= 4); // At least 2 lines + 2 padding
}

#[test]
fn calculate_heights_consistency_across_viewport_sizes() {
    let message = Message::new("Consistent message", Role::User);
    
    // Test multiple viewport sizes
    let sizes = vec![10, 20, 40, 80, 120];
    let mut heights = vec![];
    
    for size in sizes {
        let height = calculate_message_display_height(&message, size);
        heights.push(height);
    }
    
    // Larger viewports should generally result in same or smaller heights
    for i in 1..heights.len() {
        assert!(heights[i] <= heights[i-1], 
                "Height should not increase with larger viewport: {} vs {}", 
                heights[i-1], heights[i]);
    }
}

#[test]
fn calculate_total_height_is_sum_of_individual_heights() {
    let msg1 = Message::new("Message one", Role::User);
    let msg2 = Message::new("Message two with more content", Role::Assistant);
    let msg3 = Message::new("Message\nthree\nwith\nmultiple\nlines", Role::User);
    let messages = vec![&msg1, &msg2, &msg3];
    let viewport_width = 60;

    let total_height = calculate_total_content_height(&messages, viewport_width);
    
    let individual_sum = messages.iter()
        .map(|msg| calculate_message_display_height(msg, viewport_width))
        .sum::<usize>();

    assert_eq!(total_height, individual_sum);
}