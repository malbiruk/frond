use frond_core::{Action, Dialogue, Role, Tree};

#[test]
fn undo_action_rename_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let action = Action::RenameDialogue {
        old_name: "main".to_string(),
        new_name: "renamed".to_string(),
    };

    dialogue.apply_action(action).unwrap();
    assert_eq!(dialogue.name(), "renamed");

    dialogue.undo_action().unwrap();
    assert_eq!(dialogue.name(), "main");
}

#[test]
fn undo_action_rename_tree() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let action = Action::RenameTree {
        tree_id,
        old_name: "main".to_string(),
        new_name: "renamed_tree".to_string(),
    };

    dialogue.apply_action(action).unwrap();
    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.name(), "renamed_tree");

    dialogue.undo_action().unwrap();
    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.name(), "main");
}

#[test]
fn undo_action_fork_branch() {
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

    dialogue.undo_action().unwrap();
    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 1);
}

#[test]
fn undo_action_append_message() {
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
    assert_eq!(
        dialogue
            .get_branch_by_id(branch_id)
            .unwrap()
            .messages()
            .len(),
        1
    );

    dialogue.undo_action().unwrap();
    assert_eq!(
        dialogue
            .get_branch_by_id(branch_id)
            .unwrap()
            .messages()
            .len(),
        0
    );
}

#[test]
fn undo_action_rename_branch() {
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
    assert_eq!(
        dialogue.get_branch_by_id(branch_id).unwrap().name(),
        "new_name"
    );

    dialogue.undo_action().unwrap();
    assert_eq!(
        dialogue.get_branch_by_id(branch_id).unwrap().name(),
        "old_name"
    );
}

#[test]
fn undo_action_edit_message() {
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
    assert_eq!(
        dialogue.get_message_by_id(message_id).unwrap().content(),
        "Updated message"
    );

    dialogue.undo_action().unwrap();
    assert_eq!(
        dialogue.get_message_by_id(message_id).unwrap().content(),
        "Hello world"
    );
}

#[test]
fn undo_action_delete_message() {
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

    dialogue.undo_action().unwrap();
    assert!(dialogue.get_message_by_id(message_id).is_some());
}

#[test]
fn undo_action_toggle_message_role() {
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
    assert_eq!(
        dialogue.get_message_by_id(message_id).unwrap().role(),
        &Role::Assistant
    );

    dialogue.undo_action().unwrap();
    assert_eq!(
        dialogue.get_message_by_id(message_id).unwrap().role(),
        &Role::User
    );
}

#[test]
fn redo_action_rename_dialogue() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let action = Action::RenameDialogue {
        old_name: "main".to_string(),
        new_name: "renamed".to_string(),
    };

    dialogue.apply_action(action).unwrap();
    dialogue.undo_action().unwrap();
    assert_eq!(dialogue.name(), "main");

    dialogue.redo_action().unwrap();
    assert_eq!(dialogue.name(), "renamed");
}

#[test]
fn redo_action_with_no_actions_to_redo() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    dialogue.redo_action().unwrap();
    assert_eq!(dialogue.name(), "main");
}

#[test]
fn undo_action_with_no_actions_to_undo() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    dialogue.undo_action().unwrap();
    assert_eq!(dialogue.name(), "main");
}

#[test]
fn delete_message_restores_to_correct_location() {
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
            message_content: "First message".to_string(),
        })
        .unwrap();
    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "Second message".to_string(),
        })
        .unwrap();
    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "Third message".to_string(),
        })
        .unwrap();
    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "Fourth message".to_string(),
        })
        .unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let msg2 = branch.messages()[1].clone();
    let msg2_id = msg2.id();
    let msg3_id = branch.messages()[2].id();

    let delete_action = Action::DeleteMessage {
        message_id: msg2_id,
        branch_id,
        message_index: 1,
        deleted_message: msg2,
    };

    dialogue.apply_action(delete_action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.messages().len(), 3);
    assert_eq!(branch.messages()[1].id(), msg3_id);
    assert_eq!(branch.messages()[1].content(), "Third message");

    dialogue.undo_action().unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.messages().len(), 4);
    assert_eq!(branch.messages()[1].id(), msg2_id);
    assert_eq!(branch.messages()[1].content(), "Second message");
    assert_eq!(branch.messages()[2].id(), msg3_id);
    assert_eq!(branch.messages()[2].content(), "Third message");
}

#[test]
fn multiple_undo_redo_operations() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let action1 = Action::RenameDialogue {
        old_name: "main".to_string(),
        new_name: "step1".to_string(),
    };
    let action2 = Action::RenameDialogue {
        old_name: "step1".to_string(),
        new_name: "step2".to_string(),
    };
    let action3 = Action::RenameDialogue {
        old_name: "step2".to_string(),
        new_name: "step3".to_string(),
    };

    dialogue.apply_action(action1).unwrap();
    assert_eq!(dialogue.name(), "step1");

    dialogue.apply_action(action2).unwrap();
    assert_eq!(dialogue.name(), "step2");

    dialogue.apply_action(action3).unwrap();
    assert_eq!(dialogue.name(), "step3");

    dialogue.undo_action().unwrap();
    assert_eq!(dialogue.name(), "step2");

    dialogue.undo_action().unwrap();
    assert_eq!(dialogue.name(), "step1");

    dialogue.undo_action().unwrap();
    assert_eq!(dialogue.name(), "main");

    dialogue.redo_action().unwrap();
    assert_eq!(dialogue.name(), "step1");

    dialogue.redo_action().unwrap();
    assert_eq!(dialogue.name(), "step2");

    dialogue.redo_action().unwrap();
    assert_eq!(dialogue.name(), "step3");
}
