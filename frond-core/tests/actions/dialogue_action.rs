use frond_core::{Action, Dialogue, DialogueAction, Tree};

#[test]
fn rename_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let action = Action::Dialogue(DialogueAction::RenameDialogue {
        old_name: "main".to_string(),
        new_name: "renamed".to_string(),
    });

    dialogue.apply_action(action).unwrap();
    assert_eq!(dialogue.name(), "renamed");
}

#[test]
fn set_dialogue_description() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let action = Action::Dialogue(DialogueAction::SetDialogueDescription {
        old_description: None,
        new_description: "A test dialogue".to_string(),
    });

    dialogue.apply_action(action).unwrap();
    assert_eq!(dialogue.description(), Some("A test dialogue"));
}

#[test]
fn clear_dialogue_description() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    // First set a description
    let set_action = Action::Dialogue(DialogueAction::SetDialogueDescription {
        old_description: None,
        new_description: "A test dialogue".to_string(),
    });
    dialogue.apply_action(set_action).unwrap();

    // Then clear it
    let clear_action = Action::Dialogue(DialogueAction::ClearDialogueDescription {
        old_description: "A test dialogue".to_string(),
    });
    dialogue.apply_action(clear_action).unwrap();

    assert_eq!(dialogue.description(), None);
}

#[test]
fn archive_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    assert!(!dialogue.is_archived());

    let action = Action::Dialogue(DialogueAction::ArchiveDialogue {
        was_archived: false,
    });

    dialogue.apply_action(action).unwrap();
    assert!(dialogue.is_archived());
}

#[test]
fn unarchive_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    // Archive first
    let archive_action = Action::Dialogue(DialogueAction::ArchiveDialogue {
        was_archived: false,
    });
    dialogue.apply_action(archive_action).unwrap();

    // Then unarchive
    let unarchive_action =
        Action::Dialogue(DialogueAction::UnarchiveDialogue { was_archived: true });
    dialogue.apply_action(unarchive_action).unwrap();

    assert!(!dialogue.is_archived());
}

#[test]
fn trash_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    assert!(!dialogue.is_trashed());

    let action = Action::Dialogue(DialogueAction::TrashDialogue { was_trashed: false });

    dialogue.apply_action(action).unwrap();
    assert!(dialogue.is_trashed());
}

#[test]
fn restore_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    // Trash first
    let trash_action = Action::Dialogue(DialogueAction::TrashDialogue { was_trashed: false });
    dialogue.apply_action(trash_action).unwrap();

    // Then restore
    let restore_action = Action::Dialogue(DialogueAction::RestoreDialogue { was_trashed: true });
    dialogue.apply_action(restore_action).unwrap();

    assert!(!dialogue.is_trashed());
}

#[test]
fn add_tag() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    assert!(dialogue.tags().is_empty());

    let action = Action::Dialogue(DialogueAction::AddTag {
        tag: "important".to_string(),
    });

    dialogue.apply_action(action).unwrap();
    assert_eq!(dialogue.tags(), &vec!["important".to_string()]);
}

#[test]
fn remove_tag() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    // Add a tag first
    let add_action = Action::Dialogue(DialogueAction::AddTag {
        tag: "important".to_string(),
    });
    dialogue.apply_action(add_action).unwrap();

    // Remove the tag
    let remove_action = Action::Dialogue(DialogueAction::RemoveTag {
        tag: "important".to_string(),
    });
    dialogue.apply_action(remove_action).unwrap();

    assert!(dialogue.tags().is_empty());
}

#[test]
fn clear_tags() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    // Add multiple tags
    let add_action1 = Action::Dialogue(DialogueAction::AddTag {
        tag: "important".to_string(),
    });
    let add_action2 = Action::Dialogue(DialogueAction::AddTag {
        tag: "urgent".to_string(),
    });
    dialogue.apply_action(add_action1).unwrap();
    dialogue.apply_action(add_action2).unwrap();

    // Clear all tags
    let clear_action = Action::Dialogue(DialogueAction::ClearTags {
        removed_tags: vec!["important".to_string(), "urgent".to_string()],
    });
    dialogue.apply_action(clear_action).unwrap();

    assert!(dialogue.tags().is_empty());
}

#[test]
fn add_tree() {
    let mut dialogue = Dialogue::new("Test Dialogue");

    let action = Action::Dialogue(DialogueAction::AddTree {
        tree_id: uuid::Uuid::new_v4(),
        tree_name: "New Tree".to_string(),
    });

    dialogue.apply_action(action).unwrap();
    assert_eq!(dialogue.trees().len(), 1);
    assert_eq!(dialogue.trees()[0].name(), "New Tree");
}

#[test]
fn remove_tree() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let tree_clone = tree.clone();
    let mut dialogue = Dialogue::from_tree(tree);

    let action = Action::Dialogue(DialogueAction::RemoveTree {
        tree_id,
        removed_tree: tree_clone,
    });

    dialogue.apply_action(action).unwrap();
    assert!(dialogue.trees().is_empty());
    assert!(dialogue.get_tree_by_id(tree_id).is_none());
}

#[test]
fn clear_trees() {
    let tree1 = Tree::new("tree1");
    let tree2 = Tree::new("tree2");
    let trees_clone = vec![tree1.clone(), tree2.clone()];
    let mut dialogue = Dialogue::from_trees("dialogue", vec![tree1, tree2]);

    let action = Action::Dialogue(DialogueAction::ClearTrees {
        removed_trees: trees_clone,
    });

    dialogue.apply_action(action).unwrap();
    assert!(dialogue.trees().is_empty());
}

#[test]
fn multiple_dialogue_actions() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    // Rename dialogue
    let rename_action = Action::Dialogue(DialogueAction::RenameDialogue {
        old_name: "main".to_string(),
        new_name: "renamed".to_string(),
    });
    dialogue.apply_action(rename_action).unwrap();

    // Set description
    let desc_action = Action::Dialogue(DialogueAction::SetDialogueDescription {
        old_description: None,
        new_description: "A test dialogue".to_string(),
    });
    dialogue.apply_action(desc_action).unwrap();

    // Add tag
    let tag_action = Action::Dialogue(DialogueAction::AddTag {
        tag: "important".to_string(),
    });
    dialogue.apply_action(tag_action).unwrap();

    // Archive
    let archive_action = Action::Dialogue(DialogueAction::ArchiveDialogue {
        was_archived: false,
    });
    dialogue.apply_action(archive_action).unwrap();

    // Verify all changes
    assert_eq!(dialogue.name(), "renamed");
    assert_eq!(dialogue.description(), Some("A test dialogue"));
    assert_eq!(dialogue.tags(), &vec!["important".to_string()]);
    assert!(dialogue.is_archived());
}
