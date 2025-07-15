use frond_core::{Action, Dialogue, Tree};

#[test]
fn creates_dialogue_with_name() {
    let dialogue = Dialogue::new("foo");
    assert_eq!(dialogue.name(), "foo");
    assert!(dialogue.trees().is_empty());
}

#[test]
fn adds_and_removes_tree() {
    let mut dialogue = Dialogue::new("foo");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "test".to_string(),
        })
        .unwrap();
    assert_eq!(dialogue.trees().len(), 1);

    let tree_id = dialogue.trees()[0].id();
    dialogue
        .apply_action(Action::RemoveTree {
            tree_id,
            removed_tree: dialogue.trees()[0].clone(),
        })
        .unwrap();
    assert!(dialogue.trees().is_empty());
}

#[test]
fn clears_trees() {
    let mut dialogue = Dialogue::new("foo");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "test".to_string(),
        })
        .unwrap();

    dialogue
        .apply_action(Action::ClearTrees {
            removed_trees: dialogue.trees().to_vec(),
        })
        .unwrap();
    assert!(dialogue.trees().is_empty());
}

#[test]
fn renames_dialogue() {
    let mut dialogue = Dialogue::new("foo");

    dialogue
        .apply_action(Action::RenameDialogue {
            old_name: "foo".to_string(),
            new_name: "bar".to_string(),
        })
        .unwrap();
    assert_eq!(dialogue.name(), "bar");
}

#[test]
fn sets_and_clears_dialogue_description() {
    let mut dialogue = Dialogue::new("foo");

    dialogue
        .apply_action(Action::SetDialogueDescription {
            old_description: None,
            new_description: "desc".to_string(),
        })
        .unwrap();
    assert_eq!(dialogue.description(), Some("desc"));

    dialogue
        .apply_action(Action::ClearDialogueDescription {
            old_description: "desc".to_string(),
        })
        .unwrap();
    assert_eq!(dialogue.description(), None);
}

#[test]
fn gets_tree_by_id() {
    let mut dialogue = Dialogue::new("foo");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "test".to_string(),
        })
        .unwrap();

    let tree_id = dialogue.trees()[0].id();
    let found = dialogue.get_tree_by_id(tree_id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().id(), tree_id);

    let not_found = dialogue.get_tree_by_id(uuid::Uuid::new_v4());
    assert!(not_found.is_none());
}

#[test]
fn creates_dialogue_from_tree() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let dialogue = Dialogue::from_tree(tree);

    assert_eq!(dialogue.name(), "main");
    assert_eq!(dialogue.trees().len(), 1);
    let contained_tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(contained_tree.name(), "main");
}

#[test]
fn archive_and_unarchive_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    assert!(!dialogue.is_archived());

    dialogue
        .apply_action(Action::ArchiveDialogue {
            was_archived: false,
        })
        .unwrap();
    assert!(dialogue.is_archived());

    dialogue
        .apply_action(Action::UnarchiveDialogue { was_archived: true })
        .unwrap();
    assert!(!dialogue.is_archived());
}

#[test]
fn trash_and_restore_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    assert!(!dialogue.is_trashed());

    dialogue
        .apply_action(Action::TrashDialogue { was_trashed: false })
        .unwrap();
    assert!(dialogue.is_trashed());

    dialogue
        .apply_action(Action::RestoreDialogue { was_trashed: true })
        .unwrap();
    assert!(!dialogue.is_trashed());
}

#[test]
fn add_and_remove_tags() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    assert!(dialogue.tags().is_empty());

    dialogue
        .apply_action(Action::AddTag {
            tag: "foo".to_string(),
        })
        .unwrap();
    dialogue
        .apply_action(Action::AddTag {
            tag: "bar".to_string(),
        })
        .unwrap();
    assert_eq!(dialogue.tags(), &vec!["foo".to_string(), "bar".to_string()]);

    dialogue
        .apply_action(Action::RemoveTag {
            tag: "foo".to_string(),
        })
        .unwrap();
    assert_eq!(dialogue.tags(), &vec!["bar".to_string()]);

    dialogue
        .apply_action(Action::ClearTags {
            removed_tags: vec!["bar".to_string()],
        })
        .unwrap();
    assert!(dialogue.tags().is_empty());
}
