//! Tests for breadcrumb text building and token formatting
//!
//! These tests verify the accuracy of breadcrumb construction logic and token
//! display formatting, focusing on the mathematical token formatting calculations.

use frond::app::AppState;
use frond::app::state::ModelInfo;
use frond::config::Config;
use frond_core::{Branch, Dialogue, Message, Role, Tree};

// Note: Since most breadcrumb functions are private, we'll test through build_token_info
// and format_tokens indirectly by testing the token formatting logic patterns

// === Token Formatting Tests ===

fn create_test_app_state_with_tokens(used: u32, available: u32) -> AppState {
    let mut dialogue = Dialogue::new("Test Dialogue");
    let mut tree = Tree::new("Test Tree");
    let mut branch = Branch::new("main");
    branch.add_message(Message::new("Test message", Role::User));
    tree.add_branch(branch);
    dialogue.add_tree(tree);

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
        mode: frond::app::Mode::Normal,
        config: Config::default(),
        model_info: ModelInfo {
            tokens_used: used,
            tokens_available: available,
        },
        current_tree_id: tree_id,
        current_branch_id: branch_id,
        focused_message_id: message_id,
        scroll_offset: 0,
        scrollbar_state: ratatui::widgets::ScrollbarState::default(),
        pending_scrolling_request: None,
        error_message: None,
        edit_textarea: None,
        highlight_cache: std::collections::HashMap::new(),
    }
}

// We'll test the token formatting logic by examining the pattern in the output
fn extract_token_parts(token_info: &str) -> (String, String) {
    let parts: Vec<&str> = token_info.split('/').collect();
    if parts.len() == 2 {
        (parts[0].to_string(), parts[1].to_string())
    } else {
        ("".to_string(), "".to_string())
    }
}

#[test]
fn token_formatting_small_numbers() {
    let app_state = create_test_app_state_with_tokens(123, 456);
    let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);

    let (used, available) = extract_token_parts(&token_info);

    // Small numbers should be displayed as-is
    assert_eq!(used, "123");
    assert_eq!(available, "456");
    assert_eq!(token_info, "123/456");
}

#[test]
fn token_formatting_exactly_1000() {
    let app_state = create_test_app_state_with_tokens(1000, 2000);
    let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);

    let (used, available) = extract_token_parts(&token_info);

    // 1000 should be formatted as 1.0K, 2000 as 2.0K
    assert_eq!(used, "1.0K");
    assert_eq!(available, "2.0K");
}

#[test]
fn token_formatting_small_k_values() {
    let app_state = create_test_app_state_with_tokens(1500, 9900);
    let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);

    let (used, available) = extract_token_parts(&token_info);

    // Values under 10K should show one decimal place
    assert_eq!(used, "1.5K");
    assert_eq!(available, "9.9K");
}

#[test]
fn token_formatting_large_k_values() {
    let app_state = create_test_app_state_with_tokens(10000, 25000);
    let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);

    let (used, available) = extract_token_parts(&token_info);

    // Values 10K and above should be whole numbers
    assert_eq!(used, "10K");
    assert_eq!(available, "25K");
}

#[test]
fn token_formatting_very_large_numbers() {
    let app_state = create_test_app_state_with_tokens(100000, 999000);
    let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);

    let (used, available) = extract_token_parts(&token_info);

    // Very large numbers should be whole K values
    assert_eq!(used, "100K");
    assert_eq!(available, "999K");
}

#[test]
fn token_formatting_zero_values() {
    let app_state = create_test_app_state_with_tokens(0, 0);
    let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);

    let (used, available) = extract_token_parts(&token_info);

    // Zero should be displayed as-is
    assert_eq!(used, "0");
    assert_eq!(available, "0");
    assert_eq!(token_info, "0/0");
}

#[test]
fn token_formatting_mixed_sizes() {
    let app_state = create_test_app_state_with_tokens(500, 15000);
    let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);

    let (used, available) = extract_token_parts(&token_info);

    // Should format each number according to its own size
    assert_eq!(used, "500"); // Small number, no K
    assert_eq!(available, "15K"); // Large number, whole K
}

#[test]
fn token_formatting_boundary_conditions() {
    // Test numbers right at the boundaries
    let test_cases = vec![
        (999, "999"),    // Just under 1K
        (1000, "1.0K"),  // Exactly 1K
        (1001, "1.0K"),  // Just over 1K (should round to 1.0K)
        (9999, "10.0K"), // Just under 10K (should round to 10.0K)
        (10000, "10K"),  // Exactly 10K
        (10001, "10K"),  // Just over 10K
    ];

    for (tokens, expected) in test_cases {
        let app_state = create_test_app_state_with_tokens(tokens, tokens);
        let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);
        let (used, _) = extract_token_parts(&token_info);

        assert_eq!(used, expected, "Failed for token count {}", tokens);
    }
}

#[test]
fn token_formatting_decimal_precision() {
    // Test that decimal formatting is correct for various values
    let test_cases = vec![
        (1100, "1.1K"),
        (1200, "1.2K"),
        (1500, "1.5K"),
        (1900, "1.9K"),
        (2750, "2.8K"),  // Should round 2.75 to 2.8
        (3333, "3.3K"),  // Should round 3.333 to 3.3
        (9999, "10.0K"), // Should round 9.999 to 10.0
    ];

    for (tokens, expected) in test_cases {
        let app_state = create_test_app_state_with_tokens(tokens, 50000);
        let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);
        let (used, _) = extract_token_parts(&token_info);

        assert_eq!(
            used, expected,
            "Failed decimal formatting for token count {}",
            tokens
        );
    }
}

#[test]
fn token_formatting_large_whole_numbers() {
    // Test that large numbers are formatted correctly as whole K values
    let test_cases = vec![
        (10000, "10K"),
        (15000, "15K"),
        (25000, "25K"),
        (100000, "100K"),
        (500000, "500K"),
        (999000, "999K"),
        (1000000, "1000K"), // Very large, still uses K
    ];

    for (tokens, expected) in test_cases {
        let app_state = create_test_app_state_with_tokens(tokens, 1000000);
        let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);
        let (used, _) = extract_token_parts(&token_info);

        assert_eq!(
            used, expected,
            "Failed whole K formatting for token count {}",
            tokens
        );
    }
}

#[test]
fn token_info_format_consistency() {
    let app_state = create_test_app_state_with_tokens(1500, 25000);
    let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);

    // Should always be in format "used/available"
    assert!(
        token_info.contains("/"),
        "Token info should contain separator"
    );
    assert_eq!(
        token_info.matches("/").count(),
        1,
        "Should have exactly one separator"
    );

    let parts: Vec<&str> = token_info.split('/').collect();
    assert_eq!(parts.len(), 2, "Should have exactly two parts");
    assert!(!parts[0].is_empty(), "Used tokens part should not be empty");
    assert!(
        !parts[1].is_empty(),
        "Available tokens part should not be empty"
    );
}

#[test]
fn token_formatting_rounding_behavior() {
    // Test specific rounding cases to ensure consistency
    let rounding_cases = vec![
        (1050, "1.1K"),  // 1.05 rounds to 1.1
        (1149, "1.1K"),  // 1.149 rounds to 1.1
        (1150, "1.1K"),  // 1.15 rounds to 1.1 (standard rounding)
        (1250, "1.2K"),  // 1.25 rounds to 1.2
        (9950, "9.9K"), // 9.95 rounds to 9.9 (standard rounding)
    ];

    for (tokens, expected) in rounding_cases {
        let app_state = create_test_app_state_with_tokens(tokens, 50000);
        let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);
        let (used, _) = extract_token_parts(&token_info);

        assert_eq!(used, expected, "Failed rounding for token count {}", tokens);
    }
}

// === Integration Tests ===

#[test]
fn token_info_builds_correctly_with_different_model_states() {
    let test_cases = vec![
        (0, 1000),       // No usage
        (500, 1000),     // Half usage
        (1000, 1000),    // Full usage
        (1000, 2000),    // Moderate usage of larger pool
        (50000, 100000), // Large numbers
    ];

    for (used, available) in test_cases {
        let app_state = create_test_app_state_with_tokens(used, available);
        let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);

        // Should always produce a valid format
        assert!(
            token_info.contains("/"),
            "Should contain separator for case {}/{}",
            used,
            available
        );
        assert!(
            !token_info.is_empty(),
            "Should not be empty for case {}/{}",
            used,
            available
        );

        let (used_str, available_str) = extract_token_parts(&token_info);
        assert!(
            !used_str.is_empty(),
            "Used part should not be empty for case {}/{}",
            used,
            available
        );
        assert!(
            !available_str.is_empty(),
            "Available part should not be empty for case {}/{}",
            used,
            available
        );
    }
}

#[test]
fn token_formatting_maintains_readability() {
    // Test that formatted tokens maintain reasonable length for UI display
    let large_numbers = vec![
        999999,  // Should be "999K" (4 chars)
        1000000, // Should be "1000K" (5 chars)
        100,     // Should be "100" (3 chars)
        50000,   // Should be "50K" (3 chars)
    ];

    for tokens in large_numbers {
        let app_state = create_test_app_state_with_tokens(tokens, tokens);
        let token_info = frond::ui::normal_mode::breadcrumb::build_token_info(&app_state);
        let (used, _) = extract_token_parts(&token_info);

        // Formatted tokens should be reasonably short for UI display
        assert!(
            used.len() <= 6,
            "Formatted token '{}' should be 6 chars or less for readability",
            used
        );

        // Should not have unnecessary precision
        if used.contains('.') {
            let decimal_places = used
                .split('.')
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .count();
            assert!(
                decimal_places <= 1,
                "Should have at most 1 decimal place in '{}'",
                used
            );
        }
    }
}
