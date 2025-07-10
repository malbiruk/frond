use frond_core::{Dialogue, Tree};

crate::test_core_entity!(Dialogue, tree, trees, Tree, Tree::new("test"));

#[test]
fn creates_dialogue_from_tree() {
    let tree = Tree::new("main");
    let dialogue = Dialogue::from_tree(tree.clone());
    assert_eq!(dialogue.name(), "main");
    assert_eq!(dialogue.trees().len(), 1);
    assert_eq!(dialogue.trees()[0].id(), tree.id());
}

#[test]
fn archive_and_unarchive_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    assert!(!dialogue.is_archived());
    dialogue.archive();
    assert!(dialogue.is_archived());
    dialogue.unarchive();
    assert!(!dialogue.is_archived());
}

#[test]
fn trash_and_restore_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    assert!(!dialogue.is_trashed());
    dialogue.trash();
    assert!(dialogue.is_trashed());
    dialogue.restore();
    assert!(!dialogue.is_trashed());
}

#[test]
fn add_and_remove_tags() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    assert!(dialogue.tags().is_empty());

    dialogue.add_tag("foo".to_string());
    dialogue.add_tag("bar".to_string());
    assert_eq!(dialogue.tags(), &vec!["foo".to_string(), "bar".to_string()]);

    dialogue.remove_tag("foo");
    assert_eq!(dialogue.tags(), &vec!["bar".to_string()]);

    dialogue.clear_tags();
    assert!(dialogue.tags().is_empty());
}
