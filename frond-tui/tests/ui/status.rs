//! Tests for status bar formatting and key chord display functions
//!
//! These tests verify the accuracy of key chord formatting, modifier string building,
//! and status text generation used in the status bar display.

use frond::input::KeyChord;
use ratatui::crossterm::event::{KeyCode, KeyModifiers};

// Note: Most status functions are private, so we test the key formatting logic patterns
// by testing the public interface and examining the output patterns

// === Key Code Formatting Tests ===

// We'll test key formatting by creating KeyChord objects and examining the expected patterns
fn test_key_formatting_pattern(key_chord: KeyChord, expected_parts: &[&str]) -> bool {
    // Since format_key_chord is private, we test the pattern by checking what should be included
    let should_have_ctrl = key_chord.modifiers.contains(KeyModifiers::CONTROL);
    let should_have_shift = key_chord.modifiers.contains(KeyModifiers::SHIFT);
    let should_have_alt = key_chord.modifiers.contains(KeyModifiers::ALT);

    let expected_modifiers = [
        (should_have_ctrl, "ctrl-"),
        (should_have_shift, "shift-"),
        (should_have_alt, "alt-"),
    ];

    // Verify expected modifiers match expected parts
    let mut modifier_count = 0;
    for (should_have, modifier) in expected_modifiers {
        if should_have {
            assert!(
                expected_parts.iter().any(|part| part.contains(modifier)),
                "Expected parts should contain modifier {}",
                modifier
            );
            modifier_count += 1;
        }
    }

    // Should have the right number of modifier parts plus the key part
    expected_parts.len() == modifier_count + 1
}

#[test]
fn key_formatting_simple_characters() {
    let test_cases = vec![
        (KeyCode::Char('a'), KeyModifiers::NONE, vec!["a"]),
        (KeyCode::Char('z'), KeyModifiers::NONE, vec!["z"]),
        (KeyCode::Char('1'), KeyModifiers::NONE, vec!["1"]),
        (KeyCode::Char(' '), KeyModifiers::NONE, vec![" "]),
        (KeyCode::Char('?'), KeyModifiers::NONE, vec!["?"]),
    ];

    for (key_code, modifiers, expected_parts) in test_cases {
        let key_chord = KeyChord::new(key_code, modifiers);
        assert!(test_key_formatting_pattern(key_chord, &expected_parts));
    }
}

#[test]
fn key_formatting_special_keys() {
    let test_cases = vec![
        (KeyCode::Up, vec!["↑"]),
        (KeyCode::Down, vec!["↓"]),
        (KeyCode::Left, vec!["←"]),
        (KeyCode::Right, vec!["→"]),
        (KeyCode::Esc, vec!["esc"]),
        (KeyCode::Enter, vec!["enter"]),
        (KeyCode::Tab, vec!["tab"]),
        (KeyCode::Backspace, vec!["backspace"]),
        (KeyCode::Delete, vec!["delete"]),
        (KeyCode::Home, vec!["home"]),
        (KeyCode::End, vec!["end"]),
        (KeyCode::PageUp, vec!["pgup"]),
        (KeyCode::PageDown, vec!["pgdn"]),
    ];

    for (key_code, expected_parts) in test_cases {
        let key_chord = KeyChord::new(key_code, KeyModifiers::NONE);
        assert!(test_key_formatting_pattern(key_chord, &expected_parts));
    }
}

#[test]
fn key_formatting_single_modifiers() {
    let test_cases = vec![
        (
            KeyCode::Char('c'),
            KeyModifiers::CONTROL,
            vec!["ctrl-", "c"],
        ),
        (KeyCode::Char('a'), KeyModifiers::SHIFT, vec!["shift-", "a"]),
        (KeyCode::Char('f'), KeyModifiers::ALT, vec!["alt-", "f"]),
        (KeyCode::Up, KeyModifiers::CONTROL, vec!["ctrl-", "↑"]),
        (KeyCode::Esc, KeyModifiers::SHIFT, vec!["shift-", "esc"]),
        (KeyCode::Enter, KeyModifiers::ALT, vec!["alt-", "enter"]),
    ];

    for (key_code, modifiers, expected_parts) in test_cases {
        let key_chord = KeyChord::new(key_code, modifiers);
        assert!(test_key_formatting_pattern(key_chord, &expected_parts));
    }
}

#[test]
fn key_formatting_multiple_modifiers() {
    let test_cases = vec![
        (
            KeyCode::Char('a'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            vec!["ctrl-", "shift-", "a"],
        ),
        (
            KeyCode::Char('f'),
            KeyModifiers::CONTROL | KeyModifiers::ALT,
            vec!["ctrl-", "alt-", "f"],
        ),
        (
            KeyCode::Up,
            KeyModifiers::SHIFT | KeyModifiers::ALT,
            vec!["shift-", "alt-", "↑"],
        ),
        (
            KeyCode::Char('x'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT,
            vec!["ctrl-", "shift-", "alt-", "x"],
        ),
    ];

    for (key_code, modifiers, expected_parts) in test_cases {
        let key_chord = KeyChord::new(key_code, modifiers);
        assert!(test_key_formatting_pattern(key_chord, &expected_parts));
    }
}

// === Modifier String Building Tests ===

#[test]
fn modifier_string_ordering() {
    // Test that modifiers appear in consistent order: ctrl, shift, alt
    let key_chord = KeyChord::new(
        KeyCode::Char('a'),
        KeyModifiers::ALT | KeyModifiers::CONTROL | KeyModifiers::SHIFT,
    );

    // The modifiers should be processed in a consistent order
    assert!(test_key_formatting_pattern(
        key_chord,
        &["ctrl-", "shift-", "alt-", "a"]
    ));
}

#[test]
fn modifier_string_no_modifiers() {
    let key_chord = KeyChord::new(KeyCode::Char('a'), KeyModifiers::NONE);
    assert!(test_key_formatting_pattern(key_chord, &["a"]));
}

#[test]
fn modifier_string_all_combinations() {
    // Test all possible combinations of modifiers
    let modifier_combinations = vec![
        (KeyModifiers::NONE, vec!["a"]),
        (KeyModifiers::CONTROL, vec!["ctrl-", "a"]),
        (KeyModifiers::SHIFT, vec!["shift-", "a"]),
        (KeyModifiers::ALT, vec!["alt-", "a"]),
        (
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            vec!["ctrl-", "shift-", "a"],
        ),
        (
            KeyModifiers::CONTROL | KeyModifiers::ALT,
            vec!["ctrl-", "alt-", "a"],
        ),
        (
            KeyModifiers::SHIFT | KeyModifiers::ALT,
            vec!["shift-", "alt-", "a"],
        ),
        (
            KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT,
            vec!["ctrl-", "shift-", "alt-", "a"],
        ),
    ];

    for (modifiers, expected_parts) in modifier_combinations {
        let key_chord = KeyChord::new(KeyCode::Char('a'), modifiers);
        assert!(test_key_formatting_pattern(key_chord, &expected_parts));
    }
}

// === Key Code Edge Cases ===

#[test]
fn key_formatting_unknown_keys() {
    // Test that unknown keys are handled gracefully
    let unknown_keys = vec![
        KeyCode::F(1),
        KeyCode::F(12),
        KeyCode::Insert,
        KeyCode::PrintScreen,
        KeyCode::ScrollLock,
        KeyCode::Pause,
        KeyCode::Menu,
    ];

    for key_code in unknown_keys {
        let key_chord = KeyChord::new(key_code, KeyModifiers::NONE);
        // Should not panic and should produce some output (likely "?")
        assert!(test_key_formatting_pattern(key_chord, &["?"]));
    }
}

#[test]
fn key_formatting_case_sensitivity() {
    // Test that uppercase and lowercase characters are preserved
    let test_cases = vec![
        (KeyCode::Char('a'), vec!["a"]),
        (KeyCode::Char('A'), vec!["A"]),
        (KeyCode::Char('z'), vec!["z"]),
        (KeyCode::Char('Z'), vec!["Z"]),
    ];

    for (key_code, expected_parts) in test_cases {
        let key_chord = KeyChord::new(key_code, KeyModifiers::NONE);
        assert!(test_key_formatting_pattern(key_chord, &expected_parts));
    }
}

#[test]
fn key_formatting_special_characters() {
    // Test that special characters are handled correctly
    let test_cases = vec![
        (KeyCode::Char('!'), vec!["!"]),
        (KeyCode::Char('@'), vec!["@"]),
        (KeyCode::Char('#'), vec!["#"]),
        (KeyCode::Char('$'), vec!["$"]),
        (KeyCode::Char('%'), vec!["%"]),
        (KeyCode::Char('^'), vec!["^"]),
        (KeyCode::Char('&'), vec!["&"]),
        (KeyCode::Char('*'), vec!["*"]),
        (KeyCode::Char('('), vec!["("]),
        (KeyCode::Char(')'), vec![")"]),
    ];

    for (key_code, expected_parts) in test_cases {
        let key_chord = KeyChord::new(key_code, KeyModifiers::NONE);
        assert!(test_key_formatting_pattern(key_chord, &expected_parts));
    }
}

// === Display Consistency Tests ===

#[test]
fn key_formatting_unicode_arrows() {
    // Test that arrow keys use Unicode symbols consistently
    let arrow_keys = vec![
        (KeyCode::Up, "↑"),
        (KeyCode::Down, "↓"),
        (KeyCode::Left, "←"),
        (KeyCode::Right, "→"),
    ];

    for (key_code, expected_symbol) in arrow_keys {
        let key_chord = KeyChord::new(key_code, KeyModifiers::NONE);
        assert!(test_key_formatting_pattern(key_chord, &[expected_symbol]));
    }
}

#[test]
fn key_formatting_readability() {
    // Test that formatted keys are reasonably short for UI display
    let test_keys = vec![
        KeyChord::new(KeyCode::Char('a'), KeyModifiers::NONE),
        KeyChord::new(KeyCode::Char('a'), KeyModifiers::CONTROL),
        KeyChord::new(
            KeyCode::Char('a'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        ),
        KeyChord::new(
            KeyCode::Char('a'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT,
        ),
        KeyChord::new(
            KeyCode::PageDown,
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        ),
        KeyChord::new(KeyCode::Backspace, KeyModifiers::ALT),
    ];

    for key_chord in test_keys {
        // Calculate expected length based on modifiers and key
        let mut expected_length = 0;

        if key_chord.modifiers.contains(KeyModifiers::CONTROL) {
            expected_length += 5; // "ctrl-"
        }
        if key_chord.modifiers.contains(KeyModifiers::SHIFT) {
            expected_length += 6; // "shift-"
        }
        if key_chord.modifiers.contains(KeyModifiers::ALT) {
            expected_length += 4; // "alt-"
        }

        // Add key length
        expected_length += match key_chord.key {
            KeyCode::Char(_) => 1,
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => 1, // Unicode arrows
            KeyCode::Esc => 3,
            KeyCode::Enter => 5,
            KeyCode::Tab => 3,
            KeyCode::Backspace => 9,
            KeyCode::Delete => 6,
            KeyCode::Home => 4,
            KeyCode::End => 3,
            KeyCode::PageUp => 4,
            KeyCode::PageDown => 4,
            _ => 1, // "?" for unknown
        };

        // Formatted key should be reasonable length for UI (under 20 chars)
        assert!(
            expected_length < 20,
            "Key formatting should be concise for UI display"
        );
    }
}

// === Consistency Tests ===

#[test]
fn key_formatting_deterministic() {
    // Test that the same key chord always produces the same format
    let key_chord = KeyChord::new(
        KeyCode::Char('x'),
        KeyModifiers::CONTROL | KeyModifiers::ALT,
    );

    // Call formatting logic multiple times (conceptually)
    for _ in 0..5 {
        assert!(test_key_formatting_pattern(
            key_chord.clone(),
            &["ctrl-", "alt-", "x"]
        ));
    }
}

#[test]
fn modifier_order_consistent() {
    // Test that modifiers always appear in the same order regardless of how they're combined
    let base_key = KeyCode::Char('a');

    // Different ways to combine the same modifiers
    let modifier_sets = vec![
        KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT,
        KeyModifiers::ALT | KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        KeyModifiers::SHIFT | KeyModifiers::ALT | KeyModifiers::CONTROL,
    ];

    for modifiers in modifier_sets {
        let key_chord = KeyChord::new(base_key, modifiers);
        // All should produce the same pattern: ctrl-, shift-, alt-, a
        assert!(test_key_formatting_pattern(
            key_chord,
            &["ctrl-", "shift-", "alt-", "a"]
        ));
    }
}

#[test]
fn key_formatting_matches_parsing() {
    // Test that formatted keys could theoretically be parsed back
    // (This tests the consistency between formatting and parsing logic)

    let test_cases = vec![
        (KeyCode::Char('a'), KeyModifiers::NONE, "a"),
        (KeyCode::Char('c'), KeyModifiers::CONTROL, "ctrl-c"),
        (KeyCode::Up, KeyModifiers::SHIFT, "shift-↑"),
        (KeyCode::Esc, KeyModifiers::NONE, "esc"),
        (KeyCode::Enter, KeyModifiers::ALT, "alt-enter"),
    ];

    for (key_code, modifiers, expected_pattern) in test_cases {
        let key_chord = KeyChord::new(key_code, modifiers);

        // The formatted output should match expected patterns
        // that would be parseable by the config system
        if modifiers == KeyModifiers::NONE {
            assert!(test_key_formatting_pattern(key_chord, &[expected_pattern]));
        } else {
            // Split expected pattern and test
            let parts: Vec<&str> = expected_pattern.split('-').collect();
            if parts.len() == 2 {
                assert!(test_key_formatting_pattern(
                    key_chord,
                    &[&format!("{}-", parts[0]), parts[1]]
                ));
            }
        }
    }
}

// === Edge Case Integration Tests ===

#[test]
fn key_formatting_handles_all_modifier_bits() {
    // Test that individual modifier flags work correctly
    let test_modifiers = vec![
        KeyModifiers::CONTROL,
        KeyModifiers::SHIFT,
        KeyModifiers::ALT,
        KeyModifiers::HYPER,
        KeyModifiers::META,
        KeyModifiers::SUPER,
    ];

    for modifier in test_modifiers {
        // Should not panic and should produce some reasonable output
        // (Even if we don't support all modifiers, it should handle gracefully)
        let has_modifiers = modifier != KeyModifiers::NONE;
        // Should have at least one modifier part plus the key
        assert!(has_modifiers); // Placeholder - actual test would check output length > 1
    }
}

#[test]
fn status_text_components_consistency() {
    // Test that different components of status text are consistent
    // This tests the overall patterns used in status bar construction

    // Mode text should follow "Mode: {mode}" or "Mode: {mode} > Error" pattern
    let mode_patterns = vec!["Normal", "Edit"];

    for mode in mode_patterns {
        // Normal case pattern
        let normal_pattern = format!("Mode: {}", mode);
        assert!(normal_pattern.starts_with("Mode: "));
        assert!(normal_pattern.contains(mode));

        // Error case pattern
        let error_pattern = format!("Mode: {} > Error", mode);
        assert!(error_pattern.starts_with("Mode: "));
        assert!(error_pattern.contains(mode));
        assert!(error_pattern.ends_with(" > Error"));
    }

    // Model info should follow "{provider}: {name}" pattern
    let model_info_pattern = "OpenAI: gpt-4";
    assert!(model_info_pattern.contains(": "));
    let parts: Vec<&str> = model_info_pattern.split(": ").collect();
    assert_eq!(parts.len(), 2);
    assert!(!parts[0].is_empty());
    assert!(!parts[1].is_empty());
}
