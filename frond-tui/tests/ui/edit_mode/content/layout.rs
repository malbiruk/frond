//! Tests for edit mode content layout calculations
//!
//! These tests verify layout behavior for progressive edit mode, focusing on
//! how textarea positioning adapts to content size and history presence.

use frond::ui::edit_mode::content::layout::calculate_adaptive_progressive_layout;
use ratatui::layout::Rect;

// === Basic Layout Tests ===

#[test]
fn small_textarea_centers_in_viewport() {
    let area = Rect::new(0, 0, 80, 24);
    let textarea_height = 4;
    let history_height = 0; // No history

    let layout = calculate_adaptive_progressive_layout(area, textarea_height, history_height);

    // Textarea should be centered vertically
    let expected_space_above = (24 - 4) / 2; // 10
    assert_eq!(layout.textarea.y, area.y + expected_space_above);
    assert_eq!(layout.textarea.height, textarea_height);
}

#[test]
fn large_textarea_fills_viewport() {
    let area = Rect::new(0, 0, 80, 24);
    let textarea_height = 30; // Larger than viewport
    let history_height = 6;

    let layout = calculate_adaptive_progressive_layout(area, textarea_height, history_height);

    // Should handle gracefully when textarea exceeds viewport
    // The layout should not cause crashes or invalid areas
    assert!(layout.textarea.height > 0);
    assert!(layout.textarea.y >= area.y);
}

#[test]
fn medium_textarea_grows_symmetrically() {
    let area = Rect::new(0, 0, 80, 20);
    let textarea_height = 8;
    let history_height = 0;

    let layout = calculate_adaptive_progressive_layout(area, textarea_height, history_height);

    // Should center with equal space above/below (or close due to rounding)
    let space_above = layout.textarea.y - area.y;
    let space_below = (area.y + area.height) - (layout.textarea.y + layout.textarea.height);

    // Allow for rounding difference of 1
    assert!(space_above.abs_diff(space_below) <= 1);
}

// === History Positioning Tests ===

#[test]
fn small_history_positions_above_textarea() {
    let area = Rect::new(0, 0, 80, 20);
    let textarea_height = 4;
    let history_height = 3; // Small history

    let layout = calculate_adaptive_progressive_layout(area, textarea_height, history_height);

    // History should be positioned right above textarea (no gap)
    assert_eq!(
        layout.history_above.y + layout.history_above.height,
        layout.textarea.y
    );
    assert_eq!(layout.history_above.height, history_height);
}

#[test]
fn large_history_uses_available_space() {
    let area = Rect::new(0, 0, 80, 20);
    let textarea_height = 4;
    let history_height = 15; // Large history

    let layout = calculate_adaptive_progressive_layout(area, textarea_height, history_height);

    // History should use all available space above textarea
    let available_space_above = (20 - 4) / 2; // 8
    assert_eq!(layout.history_above.height, available_space_above);
    assert_eq!(layout.history_above.y, area.y); // Starts from top
}

#[test]
fn no_history_leaves_empty_space_above() {
    let area = Rect::new(0, 0, 80, 20);
    let textarea_height = 6;
    let history_height = 0; // No history

    let layout = calculate_adaptive_progressive_layout(area, textarea_height, history_height);

    // Should have empty history area
    assert_eq!(layout.history_above.height, 0);
    // Textarea should still be centered
    let expected_y = area.y + (20 - 6) / 2;
    assert_eq!(layout.textarea.y, expected_y);
}

// === Edge Cases ===

#[test]
fn minimal_viewport_handles_gracefully() {
    let area = Rect::new(0, 0, 20, 5);
    let textarea_height = 2;
    let history_height = 1;

    let layout = calculate_adaptive_progressive_layout(area, textarea_height, history_height);

    // Should not panic and produce valid areas
    assert!(layout.textarea.height > 0);
    assert!(layout.history_above.height <= area.height);
    assert!(layout.textarea.y >= area.y);
    assert!(layout.textarea.y + layout.textarea.height <= area.y + area.height);
}

#[test]
fn zero_height_areas_handled_correctly() {
    let area = Rect::new(0, 0, 80, 1);
    let textarea_height = 1;
    let history_height = 0;

    let layout = calculate_adaptive_progressive_layout(area, textarea_height, history_height);

    // Should handle minimal space without crashing
    assert_eq!(layout.textarea.height, 1);
    assert_eq!(layout.history_above.height, 0);
    assert_eq!(layout.history_below.height, 0);
}

// === Consistency Tests ===

#[test]
fn layout_areas_dont_overlap() {
    let test_cases = vec![
        (Rect::new(0, 0, 80, 24), 4, 3),
        (Rect::new(0, 0, 40, 15), 8, 5),
        (Rect::new(10, 5, 60, 20), 6, 2),
    ];

    for (area, textarea_height, history_height) in test_cases {
        let layout = calculate_adaptive_progressive_layout(area, textarea_height, history_height);

        // History above should not overlap with textarea
        if layout.history_above.height > 0 {
            assert!(
                layout.history_above.y + layout.history_above.height <= layout.textarea.y,
                "History above overlaps with textarea for area {:?}",
                area
            );
        }

        // Textarea should not overlap with history below
        if layout.history_below.height > 0 {
            assert!(
                layout.textarea.y + layout.textarea.height <= layout.history_below.y,
                "Textarea overlaps with history below for area {:?}",
                area
            );
        }

        // All areas should be within bounds
        assert!(layout.history_above.y >= area.y);
        assert!(layout.textarea.y >= area.y);
        assert!(layout.history_below.y >= area.y);
    }
}

#[test]
fn layout_consistent_across_positions() {
    let size = (80, 20);
    let textarea_height = 6;
    let history_height = 4;

    let positions = vec![(0, 0), (10, 5), (50, 10)];

    for (x, y) in positions {
        let area = Rect::new(x, y, size.0, size.1);
        let layout = calculate_adaptive_progressive_layout(area, textarea_height, history_height);

        // Relative dimensions should be same regardless of position
        assert_eq!(layout.textarea.height, textarea_height);
        assert_eq!(layout.textarea.width, area.width);

        // Should maintain proper relative positioning
        let space_above = layout.textarea.y - area.y;
        let space_below = (area.y + area.height) - (layout.textarea.y + layout.textarea.height);
        assert!(space_above.abs_diff(space_below) <= 1); // Allow rounding difference
    }
}
