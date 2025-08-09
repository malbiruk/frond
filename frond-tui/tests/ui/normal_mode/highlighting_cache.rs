//! Tests for highlighting cache functionality
//!
//! These tests verify the caching behavior of syntax highlighting in AppState,
//! focusing on atomic input/output behavior.

use frond::app::AppState;
use frond_core::{Message, Role};

#[test]
fn highlight_cache_initially_empty() {
    let app_state = AppState::default();
    
    // Cache should be empty initially
    assert!(app_state.highlight_cache.is_empty());
}

#[test]
fn get_highlighted_text_caches_result() {
    let mut app_state = AppState::default();
    let message = Message::new("# Test Message", Role::User);
    
    // Cache should be empty before first call
    assert!(!app_state.has_cached_highlight(message.id()));
    
    // First call should populate cache
    let result1 = app_state.get_highlighted_text(&message);
    assert!(app_state.has_cached_highlight(message.id()));
    
    // Second call should use cache (same result)
    let result2 = app_state.get_highlighted_text(&message);
    
    // Results should be identical
    assert_eq!(result1.lines.len(), result2.lines.len());
}

#[test]
fn highlight_cache_stores_by_message_id() {
    let mut app_state = AppState::default();
    let message1 = Message::new("First message", Role::User);
    let message2 = Message::new("Second message", Role::Assistant);
    
    // Get highlighted text for both messages
    app_state.get_highlighted_text(&message1);
    app_state.get_highlighted_text(&message2);
    
    // Both should be cached
    assert!(app_state.has_cached_highlight(message1.id()));
    assert!(app_state.has_cached_highlight(message2.id()));
    assert_eq!(app_state.highlight_cache.len(), 2);
}

#[test]
fn invalidate_message_highlight_removes_from_cache() {
    let mut app_state = AppState::default();
    let message = Message::new("Test message", Role::User);
    
    // Cache the message
    app_state.get_highlighted_text(&message);
    assert!(app_state.has_cached_highlight(message.id()));
    
    // Invalidate the cache
    app_state.invalidate_message_highlight(message.id());
    assert!(!app_state.has_cached_highlight(message.id()));
}

#[test]
fn invalidate_nonexistent_message_does_not_error() {
    let mut app_state = AppState::default();
    let fake_id = uuid::Uuid::new_v4();
    
    // Should not panic or error
    app_state.invalidate_message_highlight(fake_id);
    assert!(app_state.highlight_cache.is_empty());
}

#[test]
fn clear_highlight_cache_removes_all_entries() {
    let mut app_state = AppState::default();
    let message1 = Message::new("First", Role::User);
    let message2 = Message::new("Second", Role::Assistant);
    
    // Cache multiple messages
    app_state.get_highlighted_text(&message1);
    app_state.get_highlighted_text(&message2);
    assert_eq!(app_state.highlight_cache.len(), 2);
    
    // Clear all
    app_state.clear_highlight_cache();
    assert!(app_state.highlight_cache.is_empty());
}

#[test]
fn highlight_cache_persists_across_multiple_accesses() {
    let mut app_state = AppState::default();
    let message = Message::new("**Bold text**", Role::User);
    
    // Multiple accesses should use same cached result
    let result1 = app_state.get_highlighted_text(&message);
    let result2 = app_state.get_highlighted_text(&message);
    let result3 = app_state.get_highlighted_text(&message);
    
    // All results should be identical
    let content1 = extract_content(&result1);
    let content2 = extract_content(&result2);
    let content3 = extract_content(&result3);
    
    assert_eq!(content1, content2);
    assert_eq!(content2, content3);
    
    // Cache should still have only one entry
    assert_eq!(app_state.highlight_cache.len(), 1);
}

#[test]
fn different_message_content_produces_different_cache_entries() {
    let mut app_state = AppState::default();
    let message1 = Message::new("# Header", Role::User);
    let message2 = Message::new("```code```", Role::User);
    
    app_state.get_highlighted_text(&message1);
    app_state.get_highlighted_text(&message2);
    
    // Should have separate cache entries
    assert_eq!(app_state.highlight_cache.len(), 2);
    
    let cached1 = app_state.get_cached_highlighted_text(message1.id()).unwrap();
    let cached2 = app_state.get_cached_highlighted_text(message2.id()).unwrap();
    
    // Content should be different
    let content1 = extract_content(&cached1);
    let content2 = extract_content(&cached2);
    assert_ne!(content1, content2);
}

// Helper function to extract content from Text for comparison
fn extract_content(text: &ratatui::text::Text) -> String {
    text.lines.iter()
        .flat_map(|line| &line.spans)
        .map(|span| span.content.as_ref())
        .collect::<String>()
}