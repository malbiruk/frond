use frond_core::{Dialogue, Tree};

crate::test_core_entity!(Dialogue, tree, trees, Tree, Tree::new("test"));

#[test]
fn creates_dialogue_from_tree() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let dialogue = Dialogue::from_tree(tree);

    // Verify dialogue inherits tree name
    assert_eq!(dialogue.name(), "main");

    // Verify dialogue contains the tree
    assert_eq!(dialogue.trees().len(), 1);
    let contained_tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(contained_tree.name(), "main");
}

#[test]
fn archive_and_unarchive_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    // Verify initial state
    assert!(!dialogue.is_archived());

    // Test archiving
    dialogue.archive();
    assert!(dialogue.is_archived());

    // Test unarchiving
    dialogue.unarchive();
    assert!(!dialogue.is_archived());
}

#[test]
fn trash_and_restore_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    // Verify initial state
    assert!(!dialogue.is_trashed());

    // Test trashing
    dialogue.trash();
    assert!(dialogue.is_trashed());

    // Test restoring
    dialogue.restore();
    assert!(!dialogue.is_trashed());
}

#[test]
fn add_and_remove_tags() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    // Verify initial state
    assert!(dialogue.tags().is_empty());

    // Test adding tags
    dialogue.add_tag("foo".to_string());
    dialogue.add_tag("bar".to_string());
    assert_eq!(dialogue.tags(), &vec!["foo".to_string(), "bar".to_string()]);

    // Test removing specific tag
    dialogue.remove_tag("foo");
    assert_eq!(dialogue.tags(), &vec!["bar".to_string()]);

    // Test clearing all tags
    dialogue.clear_tags();
    assert!(dialogue.tags().is_empty());
}
