//! Tests for configuration system
//!
//! This module provides comprehensive test coverage for the configuration system,
//! including key parsing, keybinding resolution, theme configuration, and config validation.

use frond::app::Mode;
use frond::config::{
    Config, Keybindings, Theme, keybindings::parse_key_chord, keybindings::parse_mode,
};
use frond::input::KeyChord;
use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use ratatui::style::Color;
use std::collections::HashMap;

// Key Chord Parsing Tests

#[test]
fn parse_key_chord_handles_simple_characters() {
    let test_cases = vec![
        ("a", KeyCode::Char('a'), KeyModifiers::NONE),
        ("z", KeyCode::Char('z'), KeyModifiers::NONE),
        ("A", KeyCode::Char('A'), KeyModifiers::NONE),
        ("1", KeyCode::Char('1'), KeyModifiers::NONE),
        ("?", KeyCode::Char('?'), KeyModifiers::NONE),
        (" ", KeyCode::Char(' '), KeyModifiers::NONE),
    ];

    for (input, expected_key, expected_modifiers) in test_cases {
        let result = parse_key_chord(input);
        assert!(result.is_some(), "Failed to parse: {}", input);

        let chord = result.unwrap();
        assert_eq!(chord.key, expected_key, "Wrong key for: {}", input);
        assert_eq!(
            chord.modifiers, expected_modifiers,
            "Wrong modifiers for: {}",
            input
        );
    }
}

#[test]
fn parse_key_chord_handles_special_keys() {
    let test_cases = vec![
        ("up", KeyCode::Up),
        ("down", KeyCode::Down),
        ("left", KeyCode::Left),
        ("right", KeyCode::Right),
        ("esc", KeyCode::Esc),
        ("enter", KeyCode::Enter),
        ("tab", KeyCode::Tab),
        ("backspace", KeyCode::Backspace),
        ("delete", KeyCode::Delete),
        ("home", KeyCode::Home),
        ("end", KeyCode::End),
        ("pgup", KeyCode::PageUp),
        ("pgdn", KeyCode::PageDown),
    ];

    for (input, expected_key) in test_cases {
        let result = parse_key_chord(input);
        assert!(result.is_some(), "Failed to parse special key: {}", input);

        let chord = result.unwrap();
        assert_eq!(chord.key, expected_key, "Wrong key for: {}", input);
        assert_eq!(
            chord.modifiers,
            KeyModifiers::NONE,
            "Should have no modifiers for: {}",
            input
        );
    }
}

#[test]
fn parse_key_chord_handles_single_modifiers() {
    let test_cases = vec![
        ("ctrl-c", KeyCode::Char('c'), KeyModifiers::CONTROL),
        ("shift-a", KeyCode::Char('a'), KeyModifiers::SHIFT),
        ("alt-f", KeyCode::Char('f'), KeyModifiers::ALT),
        ("ctrl-up", KeyCode::Up, KeyModifiers::CONTROL),
        ("shift-esc", KeyCode::Esc, KeyModifiers::SHIFT),
        ("alt-enter", KeyCode::Enter, KeyModifiers::ALT),
    ];

    for (input, expected_key, expected_modifiers) in test_cases {
        let result = parse_key_chord(input);
        assert!(result.is_some(), "Failed to parse modified key: {}", input);

        let chord = result.unwrap();
        assert_eq!(chord.key, expected_key, "Wrong key for: {}", input);
        assert_eq!(
            chord.modifiers, expected_modifiers,
            "Wrong modifiers for: {}",
            input
        );
    }
}

#[test]
fn parse_key_chord_handles_multiple_modifiers() {
    let test_cases = vec![
        (
            "ctrl-shift-a",
            KeyCode::Char('a'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        ),
        (
            "ctrl-alt-f",
            KeyCode::Char('f'),
            KeyModifiers::CONTROL | KeyModifiers::ALT,
        ),
        (
            "shift-alt-up",
            KeyCode::Up,
            KeyModifiers::SHIFT | KeyModifiers::ALT,
        ),
    ];

    for (input, expected_key, expected_modifiers) in test_cases {
        let result = parse_key_chord(input);
        assert!(
            result.is_some(),
            "Failed to parse multi-modified key: {}",
            input
        );

        let chord = result.unwrap();
        assert_eq!(chord.key, expected_key, "Wrong key for: {}", input);
        assert_eq!(
            chord.modifiers, expected_modifiers,
            "Wrong modifiers for: {}",
            input
        );
    }
}

#[test]
fn parse_key_chord_handles_invalid_input() {
    let invalid_inputs = vec![
        "",             // Empty string
        "invalid",      // Unknown key
        "ctrl-",        // Modifier without key
        "unknown-a",    // Unknown modifier
        "ctrl-invalid", // Valid modifier, invalid key
    ];

    for input in invalid_inputs {
        let result = parse_key_chord(input);
        assert!(
            result.is_none(),
            "Should fail to parse invalid input: {}",
            input
        );
    }
}

#[test]
fn parse_key_chord_handles_edge_cases() {
    // Test hyphenated key names that might contain dashes
    let result = parse_key_chord("pgup");
    assert!(result.is_some());
    assert_eq!(result.unwrap().key, KeyCode::PageUp);

    // Test case sensitivity
    let result_lower = parse_key_chord("ctrl-a");
    let result_upper = parse_key_chord("ctrl-A");
    assert!(result_lower.is_some());
    assert!(result_upper.is_some());
    assert_ne!(result_lower.unwrap().key, result_upper.unwrap().key);
}

// Mode Parsing Tests

#[test]
fn parse_mode_handles_valid_modes() {
    let test_cases = vec![
        ("normal", Mode::Normal),
        ("edit", Mode::Edit),
    ];

    for (input, expected_mode) in test_cases {
        let result = parse_mode(input);
        assert!(result.is_ok(), "Failed to parse valid mode: {}", input);

        let mode = result.unwrap();
        assert_eq!(mode, expected_mode, "Mode mismatch for input: {}", input);
    }
}

#[test]
fn parse_mode_handles_invalid_modes() {
    let invalid_modes = vec![
        "",
        "invalid",
        "normal-extra",
        "NORMAL", // Case sensitive
    ];

    for input in invalid_modes {
        let result = parse_mode(input);
        assert!(
            result.is_err(),
            "Should fail to parse invalid mode: {}",
            input
        );
    }
}

// Keybinding Tests

#[test]
fn keybindings_default_contains_expected_mappings() {
    let keybindings = Keybindings::default();

    // Test normal mode bindings
    let normal_bindings = keybindings.mode_bindings.get(&Mode::Normal).unwrap();

    assert!(normal_bindings.contains_key("edit_message"));
    assert!(normal_bindings.contains_key("delete_message"));
    assert!(normal_bindings.contains_key("fork_branch"));
    assert!(normal_bindings.contains_key("hide_message"));
    assert!(normal_bindings.contains_key("append_message"));
    assert!(normal_bindings.contains_key("show_help"));
    assert!(normal_bindings.contains_key("scroll_up"));
    assert!(normal_bindings.contains_key("scroll_down"));
    assert!(normal_bindings.contains_key("prev_branch"));
    assert!(normal_bindings.contains_key("next_branch"));
    assert!(normal_bindings.contains_key("quit"));

    // Test specific key mappings
    assert_eq!(
        normal_bindings.get("edit_message").unwrap().key,
        KeyCode::Char('e')
    );
    assert_eq!(
        normal_bindings.get("delete_message").unwrap().key,
        KeyCode::Char('d')
    );
    assert_eq!(normal_bindings.get("scroll_up").unwrap().key, KeyCode::Up);
    assert_eq!(
        normal_bindings.get("scroll_down").unwrap().key,
        KeyCode::Down
    );
}

#[test]
fn keybindings_get_key_for_action_works() {
    let keybindings = Keybindings::default();

    let key = keybindings.get_key_for_action(Mode::Normal, "edit_message");
    assert!(key.is_some());
    assert_eq!(key.unwrap().key, KeyCode::Char('e'));

    let key = keybindings.get_key_for_action(Mode::Normal, "nonexistent_action");
    assert!(key.is_none());
}

#[test]
fn keybindings_get_action_for_key_works() {
    let keybindings = Keybindings::default();

    let action = keybindings.get_action_for_key(Mode::Normal, &KeyChord::char('e'));
    assert_eq!(action, Some("edit_message".to_string()));

    let action = keybindings.get_action_for_key(Mode::Normal, &KeyChord::char('z'));
    assert_eq!(action, None);
}

#[test]
fn keybindings_set_key_for_action_updates_mapping() {
    let mut keybindings = Keybindings::default();

    let new_key = KeyChord::char('x');
    keybindings.set_key_for_action(Mode::Normal, "edit_message".to_string(), new_key.clone());

    let retrieved_key = keybindings.get_key_for_action(Mode::Normal, "edit_message");
    assert_eq!(retrieved_key, Some(&new_key));

    let action = keybindings.get_action_for_key(Mode::Normal, &new_key);
    assert_eq!(action, Some("edit_message".to_string()));
}

#[test]
fn keybindings_from_string_map_parses_correctly() {
    let mut string_map = HashMap::new();
    let mut normal_actions = HashMap::new();
    normal_actions.insert("edit_message".to_string(), "ctrl-e".to_string());
    normal_actions.insert("delete_message".to_string(), "shift-d".to_string());
    string_map.insert("normal".to_string(), normal_actions);

    let result = Keybindings::from_string_map(string_map);
    assert!(result.is_ok());

    let keybindings = result.unwrap();
    let edit_key = keybindings.get_key_for_action(Mode::Normal, "edit_message");
    assert!(edit_key.is_some());
    assert_eq!(edit_key.unwrap().key, KeyCode::Char('e'));
    assert!(edit_key.unwrap().modifiers.contains(KeyModifiers::CONTROL));
}

#[test]
fn keybindings_from_string_map_handles_invalid_input() {
    let mut string_map = HashMap::new();
    let mut invalid_actions = HashMap::new();
    invalid_actions.insert("test_action".to_string(), "invalid-key".to_string());
    string_map.insert("normal".to_string(), invalid_actions);

    let result = Keybindings::from_string_map(string_map);
    assert!(result.is_err());
}

#[test]
fn keybindings_edit_mode_has_exit_mapping() {
    let keybindings = Keybindings::default();

    let edit_mode = Mode::Edit;
    let exit_key = keybindings.get_key_for_action(edit_mode, "exit_mode");
    assert!(exit_key.is_some());
    assert_eq!(exit_key.unwrap().key, KeyCode::Esc);
}

// Config Integration Tests

#[test]
fn config_default_creates_valid_configuration() {
    let config = Config::default();

    // Should have theme
    assert_eq!(config.theme.highlight_color, Color::Blue);
    assert_eq!(config.theme.secondary_color, Color::DarkGray);

    // Should have keybindings
    assert!(config.keybindings.mode_bindings.contains_key(&Mode::Normal));

    // Should have model info
    assert!(!config.model.provider.is_empty());
    assert!(!config.model.name.is_empty());
}

#[test]
fn config_get_key_for_action_works_across_modes() {
    let config = Config::default();

    // Normal mode action
    let key = config.get_key_for_action(Mode::Normal, "edit_message");
    assert!(key.is_some());
    assert_eq!(key.unwrap().key, KeyCode::Char('e'));

    // Edit mode action
    let key = config.get_key_for_action(Mode::Edit, "exit_mode");
    assert!(key.is_some());
    assert_eq!(key.unwrap().key, KeyCode::Esc);

    // Non-existent action
    let key = config.get_key_for_action(Mode::Normal, "nonexistent");
    assert!(key.is_none());
}

#[test]
fn config_set_key_for_action_updates_keybindings() {
    let mut config = Config::default();

    let new_key = KeyChord::ctrl('x');
    config.set_key_for_action(Mode::Normal, "edit_message".to_string(), new_key.clone());

    let retrieved_key = config.get_key_for_action(Mode::Normal, "edit_message");
    assert_eq!(retrieved_key, Some(&new_key));
}

// Theme Tests

#[test]
fn theme_default_has_valid_colors() {
    let theme = Theme::default();

    // Theme colors
    assert_eq!(theme.highlight_color, Color::Blue);
    assert_eq!(theme.secondary_color, Color::DarkGray);
}

#[test]
fn theme_contains_required_fields() {
    let theme = Theme::default();

    // Test that colors are valid (not testing specific values, just that they exist)
    let _highlight = theme.highlight_color;
    let _secondary = theme.secondary_color;
}

// Error Handling and Edge Cases

#[test]
fn keybinding_with_conflicting_keys_handles_gracefully() {
    let mut keybindings = Keybindings::default();

    let key = KeyChord::char('x');

    // Set the same key for different actions
    keybindings.set_key_for_action(Mode::Normal, "action1".to_string(), key.clone());
    keybindings.set_key_for_action(Mode::Normal, "action2".to_string(), key.clone());

    // Should resolve to one of the actions (HashMap behavior with duplicate keys)
    let action = keybindings.get_action_for_key(Mode::Normal, &key);
    assert!(action == Some("action1".to_string()) || action == Some("action2".to_string()));
}

#[test]
fn config_handles_mode_specific_keybindings() {
    let config = Config::default();

    // Same action ID should work in different modes if bound
    let normal_exit = config.get_key_for_action(Mode::Normal, "quit");
    let edit_exit = config.get_key_for_action(Mode::Edit, "exit_mode");

    assert!(normal_exit.is_some());
    assert!(edit_exit.is_some());
    // They should be different keys
    assert_ne!(normal_exit.unwrap().key, edit_exit.unwrap().key);
}

#[test]
fn keybindings_mode_isolation_works() {
    let mut keybindings = Keybindings::default();

    let key = KeyChord::char('t');
    keybindings.set_key_for_action(Mode::Normal, "test_normal".to_string(), key.clone());
    keybindings.set_key_for_action(
        Mode::Edit,
        "test_edit".to_string(),
        key.clone(),
    );

    // Same key should resolve to different actions in different modes
    let normal_action = keybindings.get_action_for_key(Mode::Normal, &key);
    let edit_action = keybindings.get_action_for_key(Mode::Edit, &key);

    assert_eq!(normal_action, Some("test_normal".to_string()));
    assert_eq!(edit_action, Some("test_edit".to_string()));
}

#[test]
fn parse_key_chord_order_independence() {
    // Test that modifier order doesn't matter (if parser supports it)
    let result1 = parse_key_chord("ctrl-shift-a");
    let result2 = parse_key_chord("shift-ctrl-a");

    if result1.is_some() && result2.is_some() {
        let chord1 = result1.unwrap();
        let chord2 = result2.unwrap();
        assert_eq!(chord1.key, chord2.key);
        assert_eq!(chord1.modifiers, chord2.modifiers);
    }
}

#[test]
fn config_model_info_has_reasonable_defaults() {
    let config = Config::default();

    assert!(!config.model.provider.is_empty());
    assert!(!config.model.name.is_empty());
}
