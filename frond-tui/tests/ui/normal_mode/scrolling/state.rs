//! Tests for scrolling state management functions
//!
//! These tests verify the accuracy of scroll position clamping calculations.
//! Note: ScrollbarState doesn't expose getters, so we only test the clamping function.

use frond::ui::normal_mode::scrolling::state::clamp_scroll_offset;

// === Scroll Offset Clamping Tests ===

#[test]
fn clamp_scroll_offset_within_bounds() {
    let scroll_offset = 10;
    let viewport_height = 20;
    let total_content_height = 100;

    let clamped = clamp_scroll_offset(scroll_offset, viewport_height, total_content_height);

    assert_eq!(clamped, scroll_offset, "Should not clamp offset within bounds");
}

#[test]
fn clamp_scroll_offset_too_negative() {
    let scroll_offset = -100;
    let viewport_height = 20;
    let total_content_height = 100;

    let clamped = clamp_scroll_offset(scroll_offset, viewport_height, total_content_height);

    // Should clamp to minimum scroll (calculated as -(viewport_height - 3))
    let expected_min = -(viewport_height as isize - 3);
    assert_eq!(clamped, expected_min);
}

#[test]
fn clamp_scroll_offset_too_positive() {
    let scroll_offset = 1000;
    let viewport_height = 20;
    let total_content_height = 100;

    let clamped = clamp_scroll_offset(scroll_offset, viewport_height, total_content_height);

    // Should clamp to maximum scroll (total_content_height - 3)
    let expected_max = total_content_height as isize - 3;
    assert_eq!(clamped, expected_max);
}

#[test]
fn clamp_scroll_offset_at_boundaries() {
    let viewport_height = 20;
    let total_content_height = 100;
    let min_scroll = -(viewport_height as isize - 3);
    let max_scroll = total_content_height as isize - 3;

    let clamped_min = clamp_scroll_offset(min_scroll, viewport_height, total_content_height);
    let clamped_max = clamp_scroll_offset(max_scroll, viewport_height, total_content_height);

    assert_eq!(clamped_min, min_scroll, "Should not clamp offset at minimum boundary");
    assert_eq!(clamped_max, max_scroll, "Should not clamp offset at maximum boundary");
}

#[test]
fn clamp_scroll_offset_small_content() {
    let scroll_offset = 10;
    let viewport_height = 20;
    let total_content_height = 5; // Very small content

    let clamped = clamp_scroll_offset(scroll_offset, viewport_height, total_content_height);

    // Should clamp to max scroll (5 - 3 = 2)
    assert_eq!(clamped, 2);
}

#[test]
fn clamp_scroll_offset_zero_content() {
    let scroll_offset = 10;
    let viewport_height = 20;
    let total_content_height = 0;

    let clamped = clamp_scroll_offset(scroll_offset, viewport_height, total_content_height);

    // Should clamp to max scroll (0 - 3 = -3)
    assert_eq!(clamped, -3);
}

#[test]
fn clamp_scroll_offset_minimal_viewport() {
    let scroll_offset = 10;
    let viewport_height = 1;
    let total_content_height = 100;

    let clamped = clamp_scroll_offset(scroll_offset, viewport_height, total_content_height);

    // Min scroll would be -(1 - 3) = 2, max scroll is 97
    // Offset 10 should remain 10
    assert_eq!(clamped, 10);
}

#[test]
fn clamp_scroll_offset_very_small_viewport() {
    let scroll_offset = -10;
    let viewport_height = 3;
    let total_content_height = 100;

    let clamped = clamp_scroll_offset(scroll_offset, viewport_height, total_content_height);

    // Min scroll is -(3 - 3) = 0
    // -10 should be clamped to 0
    assert_eq!(clamped, 0);
}

#[test]
fn clamping_is_idempotent() {
    let viewport_height = 20;
    let total_content_height = 100;

    let test_offsets = vec![-100, -10, 0, 10, 50, 100, 1000];

    for offset in test_offsets {
        let clamped_once = clamp_scroll_offset(offset, viewport_height, total_content_height);
        let clamped_twice = clamp_scroll_offset(clamped_once, viewport_height, total_content_height);
        
        assert_eq!(clamped_once, clamped_twice, 
                  "Clamping should be idempotent for offset {}", offset);
    }
}


#[test]
fn clamp_boundaries_are_mathematically_consistent() {
    let viewport_height = 20;
    let total_content_height = 100;
    
    let min_scroll = -(viewport_height as isize - 3);
    let max_scroll = total_content_height as isize - 3;
    
    // Min should be less than max for valid content
    assert!(min_scroll < max_scroll, "Min scroll should be less than max scroll for valid content");
    
    // Test that values just inside boundaries are not clamped
    if min_scroll < max_scroll {
        let just_above_min = min_scroll + 1;
        let just_below_max = max_scroll - 1;
        
        assert_eq!(clamp_scroll_offset(just_above_min, viewport_height, total_content_height), just_above_min);
        assert_eq!(clamp_scroll_offset(just_below_max, viewport_height, total_content_height), just_below_max);
    }
}