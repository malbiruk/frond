use frond_core::{Action, Dialogue, Role, Tree};

#[test]
fn apply_action_rename_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let action = Action::RenameDialogue {
        old_name: "main".to_string(),
        new_name: "renamed".to_string(),
    };

    dialogue.apply_action(action).unwrap();
    assert_eq!(dialogue.name(), "renamed");
}

#[test]
fn apply_action_rename_tree() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let action = Action::RenameTree {
        tree_id,
        old_name: "main".to_string(),
        new_name: "renamed_tree".to_string(),
    };

    dialogue.apply_action(action).unwrap();

    let renamed_tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(renamed_tree.name(), "renamed_tree");
}

#[test]
fn apply_action_fork_branch() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "main_branch".to_string(),
        })
        .unwrap();

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "Hello".to_string(),
        })
        .unwrap();
    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "Hi there".to_string(),
        })
        .unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let msg2_id = branch.messages()[1].id();

    let action = Action::ForkBranch {
        tree_id,
        branch_id,
        from_message_id: msg2_id,
        new_branch_name: "forked_branch".to_string(),
    };

    dialogue.apply_action(action).unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 2);

    let forked_branch = tree
        .branches()
        .iter()
        .find(|b| b.name() == "forked_branch")
        .expect("Forked branch should exist");
    assert_eq!(forked_branch.messages().len(), 2);
}

#[test]
fn apply_action_append_message() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "test_branch".to_string(),
        })
        .unwrap();

    let branch_id = dialogue.trees()[0].branches()[0].id();

    let action = Action::AppendMessage {
        branch_id,
        message_content: "Hello world".to_string(),
    };

    dialogue.apply_action(action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.messages().len(), 1);
    assert_eq!(branch.messages()[0].content(), "Hello world");
    assert_eq!(branch.messages()[0].role(), &Role::User);
}

#[test]
fn apply_action_rename_branch() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "old_name".to_string(),
        })
        .unwrap();

    let branch_id = dialogue.trees()[0].branches()[0].id();

    let action = Action::RenameBranch {
        branch_id,
        old_name: "old_name".to_string(),
        new_name: "new_name".to_string(),
    };

    dialogue.apply_action(action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.name(), "new_name");
}

#[test]
fn apply_action_edit_message() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "test_branch".to_string(),
        })
        .unwrap();

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "Hello world".to_string(),
        })
        .unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let message_id = branch.messages()[0].id();

    let action = Action::EditMessage {
        message_id,
        old_content: "Hello world".to_string(),
        new_content: "Updated message".to_string(),
    };

    dialogue.apply_action(action).unwrap();

    let message = dialogue.get_message_by_id(message_id).unwrap();
    assert_eq!(message.content(), "Updated message");
}

#[test]
fn apply_action_delete_message() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "test_branch".to_string(),
        })
        .unwrap();

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "Hello world".to_string(),
        })
        .unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let message = branch.messages()[0].clone();
    let message_id = message.id();

    let action = Action::DeleteMessage {
        message_id,
        branch_id,
        message_index: 0,
        deleted_message: message,
    };

    dialogue.apply_action(action).unwrap();

    assert!(dialogue.get_message_by_id(message_id).is_none());
}

#[test]
fn apply_action_toggle_message_role() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "test_branch".to_string(),
        })
        .unwrap();

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "Hello world".to_string(),
        })
        .unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let message_id = branch.messages()[0].id();

    let action = Action::ToggleMessageRole { message_id };

    dialogue.apply_action(action).unwrap();

    let message = dialogue.get_message_by_id(message_id).unwrap();
    assert_eq!(message.role(), &Role::Assistant);
}

#[test]
fn apply_action_truncates_stack_on_new_action() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let action1 = Action::RenameDialogue {
        old_name: "main".to_string(),
        new_name: "renamed1".to_string(),
    };

    let action2 = Action::RenameDialogue {
        old_name: "renamed1".to_string(),
        new_name: "renamed2".to_string(),
    };

    let action3 = Action::RenameDialogue {
        old_name: "renamed1".to_string(),
        new_name: "renamed3".to_string(),
    };

    dialogue.apply_action(action1).unwrap();
    dialogue.apply_action(action2).unwrap();
    dialogue.undo_action().unwrap();
    assert_eq!(dialogue.name(), "renamed1");

    dialogue.apply_action(action3).unwrap();
    assert_eq!(dialogue.name(), "renamed3");

    dialogue.redo_action().unwrap();
    assert_eq!(dialogue.name(), "renamed3");
}

#[test]
fn cross_tree_actions_work() {
    let tree1 = Tree::new("tree1");
    let tree2 = Tree::new("tree2");
    let tree1_id = tree1.id();
    let tree2_id = tree2.id();
    let mut dialogue = Dialogue::from_trees("dialogue", vec![tree1, tree2]);

    dialogue
        .apply_action(Action::AddBranch {
            tree_id: tree1_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "branch1".to_string(),
        })
        .unwrap();

    dialogue
        .apply_action(Action::AddBranch {
            tree_id: tree2_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "branch2".to_string(),
        })
        .unwrap();

    let branch1_id = dialogue.get_tree_by_id(tree1_id).unwrap().branches()[0].id();
    let branch2_id = dialogue.get_tree_by_id(tree2_id).unwrap().branches()[0].id();

    let append_action = Action::AppendMessage {
        branch_id: branch1_id,
        message_content: "Hello from tree1".to_string(),
    };
    dialogue.apply_action(append_action).unwrap();

    let rename_action = Action::RenameBranch {
        branch_id: branch2_id,
        old_name: "branch2".to_string(),
        new_name: "renamed_branch2".to_string(),
    };
    dialogue.apply_action(rename_action).unwrap();

    let branch1 = dialogue.get_branch_by_id(branch1_id).unwrap();
    assert_eq!(branch1.messages().len(), 1);
    assert_eq!(branch1.messages()[0].content(), "Hello from tree1");

    let branch2 = dialogue.get_branch_by_id(branch2_id).unwrap();
    assert_eq!(branch2.name(), "renamed_branch2");
}
