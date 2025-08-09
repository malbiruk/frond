//! Tests for hierarchical cache behavior
//!
//! These tests verify cache isolation between different branches/trees,
//! cache persistence when switching contexts, and width-dependent behavior.

use frond::app::AppState;
use frond_core::{Branch, Dialogue, Message, Role, Tree};

fn create_dialogue_with_multiple_trees_and_branches() -> Dialogue {
    let mut dialogue = Dialogue::new("Multi-Tree Test");

    // First tree with two branches
    let mut tree1 = Tree::new("Tree One");
    
    let mut branch1a = Branch::new("main");
    branch1a.add_message(Message::new("Tree1 Branch1 Message", Role::User));
    tree1.add_branch(branch1a);
    
    let mut branch1b = Branch::new("alternative");
    branch1b.add_message(Message::new("Tree1 Branch2 Message", Role::User));
    tree1.add_branch(branch1b);
    
    dialogue.add_tree(tree1);

    // Second tree with two branches
    let mut tree2 = Tree::new("Tree Two");
    
    let mut branch2a = Branch::new("main");
    branch2a.add_message(Message::new("Tree2 Branch1 Message", Role::User));
    tree2.add_branch(branch2a);
    
    let mut branch2b = Branch::new("alternative");
    branch2b.add_message(Message::new("Tree2 Branch2 Message", Role::User));
    tree2.add_branch(branch2b);
    
    dialogue.add_tree(tree2);
    
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
    
    AppState {
        dialogue,
        mode: frond::app::Mode::Normal,
        config: frond::config::Config::default(),
        model_info: frond::app::state::ModelInfo::default(),
        current_tree_id: tree_id,
        current_branch_id: branch_id,
        focused_message_id: None,
        scroll_offset: 0,
        scrollbar_state: Default::default(),
        pending_scrolling_request: None,
        error_message: None,
        edit_textarea: None,
        highlight_cache: Default::default(),
        height_cache: Default::default(),
    }
}

// === Cache Isolation Tests ===

#[test]
fn cache_isolated_between_different_branches() {
    let dialogue = create_dialogue_with_multiple_trees_and_branches();
    let mut app_state = create_app_state_with_dialogue(dialogue);
    
    // Get first branch message ID and populate cache
    let first_message_id = app_state.current_messages()[0].id();
    let first_tree_id = app_state.current_tree_id.unwrap();
    let first_branch_id = app_state.current_branch_id.unwrap();
    app_state.populate_caches_for_current_branch(80);
    
    // Verify cache is populated for current branch
    assert!(app_state.has_cached_highlight(first_message_id));
    
    // Switch to second branch in same tree
    let second_branch_id = app_state.current_tree().unwrap().branches().get(1).unwrap().id();
    app_state.current_branch_id = Some(second_branch_id);
    
    // Get second branch message ID
    let second_message_id = app_state.current_messages()[0].id();
    
    // Second branch message should not be cached in current context
    assert!(!app_state.has_cached_highlight(second_message_id));
    
    // First branch message should not be accessible in current context
    assert!(!app_state.has_cached_highlight(first_message_id));
    
    // But the cache entry should exist in the storage
    let first_key = (first_tree_id, first_branch_id, first_message_id);
    assert!(app_state.highlight_cache.contains_key(&first_key));
}

#[test]
fn cache_isolated_between_different_trees() {
    let dialogue = create_dialogue_with_multiple_trees_and_branches();
    let mut app_state = create_app_state_with_dialogue(dialogue);
    
    // Populate cache for first tree
    let first_message_id = app_state.current_messages()[0].id();
    let first_tree_id = app_state.current_tree_id.unwrap();
    let first_branch_id = app_state.current_branch_id.unwrap();
    app_state.populate_caches_for_current_branch(80);
    
    assert!(app_state.has_cached_highlight(first_message_id));
    
    // Switch to second tree
    let second_tree_id = app_state.dialogue.trees().get(1).unwrap().id();
    let second_branch_id = app_state.dialogue.trees().get(1).unwrap().branches().get(0).unwrap().id();
    
    app_state.current_tree_id = Some(second_tree_id);
    app_state.current_branch_id = Some(second_branch_id);
    
    // Get second tree message ID
    let second_message_id = app_state.current_messages()[0].id();
    
    // Second tree message should not be cached yet
    assert!(!app_state.has_cached_highlight(second_message_id));
    
    // First tree message should not be accessible in current context
    assert!(!app_state.has_cached_highlight(first_message_id));
    
    // But the cache entry should exist in the storage
    let first_key = (first_tree_id, first_branch_id, first_message_id);
    assert!(app_state.highlight_cache.contains_key(&first_key));
}

#[test]
fn cache_persists_when_returning_to_previous_branch() {
    let dialogue = create_dialogue_with_multiple_trees_and_branches();
    let mut app_state = create_app_state_with_dialogue(dialogue);
    
    // Store original branch info
    let original_tree_id = app_state.current_tree_id.unwrap();
    let original_branch_id = app_state.current_branch_id.unwrap();
    
    // Populate cache for first branch
    let first_message_id = app_state.current_messages()[0].id();
    app_state.populate_caches_for_current_branch(80);
    
    assert!(app_state.has_cached_highlight(first_message_id));
    
    // Switch to different branch
    let second_branch_id = app_state.current_tree().unwrap().branches().get(1).unwrap().id();
    app_state.current_branch_id = Some(second_branch_id);
    
    // Populate cache for second branch
    app_state.populate_caches_for_current_branch(80);
    
    // Switch back to original branch
    app_state.current_tree_id = Some(original_tree_id);
    app_state.current_branch_id = Some(original_branch_id);
    
    // Cache should still contain the original message
    assert!(app_state.has_cached_highlight(first_message_id));
}

// === Width-Dependent Cache Tests ===

#[test]
fn height_cache_different_widths_stored_separately() {
    let dialogue = create_dialogue_with_multiple_trees_and_branches();
    let mut app_state = create_app_state_with_dialogue(dialogue);
    
    let message_id = app_state.current_messages()[0].id();
    
    // Populate caches for different widths
    app_state.populate_caches_for_current_branch(80);
    app_state.populate_caches_for_current_branch(120);
    
    // Both width variants should be cached
    let tree_id = app_state.current_tree_id.unwrap();
    let branch_id = app_state.current_branch_id.unwrap();
    
    let key_80 = (tree_id, branch_id, message_id, 80);
    let key_120 = (tree_id, branch_id, message_id, 120);
    
    assert!(app_state.height_cache.contains_key(&key_80));
    assert!(app_state.height_cache.contains_key(&key_120));
}

#[test]
fn highlight_cache_shared_across_widths() {
    let dialogue = create_dialogue_with_multiple_trees_and_branches();
    let mut app_state = create_app_state_with_dialogue(dialogue);
    
    let message_id = app_state.current_messages()[0].id();
    
    // Populate cache for first width
    app_state.populate_caches_for_current_branch(80);
    assert!(app_state.has_cached_highlight(message_id));
    
    // Cache should still exist for second width (highlighting is width-independent)
    app_state.populate_caches_for_current_branch(120);
    assert!(app_state.has_cached_highlight(message_id));
    
    // Should have only one highlight cache entry (shared across widths)
    let tree_id = app_state.current_tree_id.unwrap();
    let branch_id = app_state.current_branch_id.unwrap();
    let highlight_key = (tree_id, branch_id, message_id);
    
    assert!(app_state.highlight_cache.contains_key(&highlight_key));
}

// === Cache Management Tests ===

#[test]
fn cache_keys_use_hierarchical_structure() {
    let dialogue = create_dialogue_with_multiple_trees_and_branches();
    let mut app_state = create_app_state_with_dialogue(dialogue);
    
    let message_id = app_state.current_messages()[0].id();
    app_state.populate_caches_for_current_branch(80);
    
    // Verify hierarchical cache structure
    let tree_id = app_state.current_tree_id.unwrap();
    let branch_id = app_state.current_branch_id.unwrap();
    
    let highlight_key = (tree_id, branch_id, message_id);
    let height_key = (tree_id, branch_id, message_id, 80);
    
    assert!(app_state.highlight_cache.contains_key(&highlight_key));
    assert!(app_state.height_cache.contains_key(&height_key));
}

#[test]
fn cache_clears_work_correctly() {
    let dialogue = create_dialogue_with_multiple_trees_and_branches();
    let mut app_state = create_app_state_with_dialogue(dialogue);
    
    let message_id = app_state.current_messages()[0].id();
    app_state.populate_caches_for_current_branch(80);
    
    // Verify caches are populated
    assert!(app_state.has_cached_highlight(message_id));
    assert!(app_state.get_cached_height(message_id, 80).is_some());
    
    // Clear highlight cache
    app_state.clear_highlight_cache();
    assert!(!app_state.has_cached_highlight(message_id));
    assert!(app_state.get_cached_height(message_id, 80).is_some()); // Height cache should remain
    
    // Clear height cache
    app_state.clear_height_cache();
    assert!(app_state.get_cached_height(message_id, 80).is_none());
}

// === Performance Characteristics Tests ===

#[test]
fn cache_hit_vs_miss_behavior() {
    let dialogue = create_dialogue_with_multiple_trees_and_branches();
    let mut app_state = create_app_state_with_dialogue(dialogue);
    
    let message_id = app_state.current_messages()[0].id();
    
    // First access should be a cache miss
    assert!(!app_state.has_cached_highlight(message_id));
    assert!(app_state.get_cached_height(message_id, 80).is_none());
    
    app_state.populate_caches_for_current_branch(80);
    
    // Second access should be a cache hit
    assert!(app_state.has_cached_highlight(message_id));
    assert!(app_state.get_cached_height(message_id, 80).is_some());
    
    // Multiple cache accesses should work
    let cached_text = app_state.get_cached_highlighted_text(message_id);
    assert!(cached_text.is_some());
    
    let cached_text_again = app_state.get_cached_highlighted_text(message_id);
    assert!(cached_text_again.is_some());
}

#[test]
fn different_branches_have_separate_cache_entries() {
    let dialogue = create_dialogue_with_multiple_trees_and_branches();
    let mut app_state = create_app_state_with_dialogue(dialogue);
    
    // Populate cache for first branch
    let first_message_id = app_state.current_messages()[0].id();
    app_state.populate_caches_for_current_branch(80);
    
    let first_tree_id = app_state.current_tree_id.unwrap();
    let first_branch_id = app_state.current_branch_id.unwrap();
    
    // Switch to second branch in same tree
    let second_branch_id = app_state.current_tree().unwrap().branches().get(1).unwrap().id();
    app_state.current_branch_id = Some(second_branch_id);
    
    let second_message_id = app_state.current_messages()[0].id();
    app_state.populate_caches_for_current_branch(80);
    
    // Verify separate cache entries exist
    let first_key = (first_tree_id, first_branch_id, first_message_id);
    let second_key = (first_tree_id, second_branch_id, second_message_id);
    
    assert!(app_state.highlight_cache.contains_key(&first_key));
    assert!(app_state.highlight_cache.contains_key(&second_key));
    assert_ne!(first_key, second_key);
}

#[test]
fn cache_methods_handle_invalid_ids_gracefully() {
    let dialogue = create_dialogue_with_multiple_trees_and_branches();
    let mut app_state = create_app_state_with_dialogue(dialogue);
    
    let fake_id = uuid::Uuid::new_v4();
    
    // Methods should handle invalid IDs gracefully
    assert!(!app_state.has_cached_highlight(fake_id));
    assert!(app_state.get_cached_highlighted_text(fake_id).is_none());
    assert!(app_state.get_cached_height(fake_id, 80).is_none());
    
    // Cache methods with invalid IDs should not panic
    app_state.cache_highlighted_text(fake_id, ratatui::text::Text::raw("test"));
    app_state.cache_height(fake_id, 80, 10);
    
    // Should have cached these entries
    assert!(app_state.has_cached_highlight(fake_id));
    assert!(app_state.get_cached_height(fake_id, 80).is_some());
}