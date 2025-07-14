use frond_core::{Action, Branch, Dialogue, Message, Role, Tree};

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

    // Verify the tree was renamed
    let renamed_tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(renamed_tree.name(), "renamed_tree");
}

#[test]
fn apply_action_fork_branch() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    // Add a branch with messages to the tree
    let mut branch = Branch::new("main_branch");
    let msg1 = Message::new("Hello", Role::User);
    let msg2 = Message::new("Hi there", Role::Assistant);
    let msg2_id = msg2.id();
    branch.add_message(msg1);
    branch.add_message(msg2);
    let branch_id = branch.id();

    dialogue.trees_mut()[0].add_branch(branch);

    let action = Action::ForkBranch {
        tree_id,
        branch_id,
        from_message_id: msg2_id,
        new_branch_name: "forked_branch".to_string(),
    };

    dialogue.apply_action(action).unwrap();

    // Verify the branch was forked
    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 2);

    // Find the forked branch by name
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
    let mut dialogue = Dialogue::from_tree(tree);

    let branch = Branch::new("test_branch");
    let branch_id = branch.id();
    dialogue.trees_mut()[0].add_branch(branch);

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
    let mut dialogue = Dialogue::from_tree(tree);

    let branch = Branch::new("old_name");
    let branch_id = branch.id();
    dialogue.trees_mut()[0].add_branch(branch);

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
    let mut dialogue = Dialogue::from_tree(tree);

    let mut branch = Branch::new("test_branch");
    let message = Message::new("Hello world", Role::User);
    let message_id = message.id();
    branch.add_message(message);
    dialogue.trees_mut()[0].add_branch(branch);

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
    let mut dialogue = Dialogue::from_tree(tree);

    let mut branch = Branch::new("test_branch");
    let message = Message::new("Hello world", Role::User);
    let message_id = message.id();
    branch.add_message(message.clone());
    let branch_id = branch.id();
    dialogue.trees_mut()[0].add_branch(branch);

    // UI layer provides branch_id and message_index
    let message_index = 0; // First message
    let action = Action::DeleteMessage {
        message_id,
        branch_id,
        message_index,
        deleted_message: message,
    };

    dialogue.apply_action(action).unwrap();

    // Verify the message is deleted
    assert!(dialogue.get_message_by_id(message_id).is_none());
}

#[test]
fn apply_action_toggle_message_role() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let mut branch = Branch::new("test_branch");
    let message = Message::new("Hello world", Role::User);
    let message_id = message.id();
    branch.add_message(message);
    dialogue.trees_mut()[0].add_branch(branch);

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

    // Apply two actions, then undo one
    dialogue.apply_action(action1).unwrap();
    dialogue.apply_action(action2).unwrap();
    dialogue.undo_action().unwrap();
    assert_eq!(dialogue.name(), "renamed1");

    // Apply a new action - this should truncate the redo stack
    dialogue.apply_action(action3).unwrap();
    assert_eq!(dialogue.name(), "renamed3");

    // Test that action2 can no longer be redone (stack was truncated)
    dialogue.redo_action().unwrap();
    assert_eq!(dialogue.name(), "renamed3"); // Should remain unchanged
}

#[test]
fn cross_tree_actions_work() {
    let tree1 = Tree::new("tree1");
    let tree2 = Tree::new("tree2");
    let mut dialogue = Dialogue::from_trees("dialogue", vec![tree1, tree2]);

    let branch1 = Branch::new("branch1");
    let branch2 = Branch::new("branch2");
    let branch1_id = branch1.id();
    let branch2_id = branch2.id();

    dialogue.trees_mut()[0].add_branch(branch1);
    dialogue.trees_mut()[1].add_branch(branch2);

    // Test AppendMessage works across trees
    let append_action = Action::AppendMessage {
        branch_id: branch1_id,
        message_content: "Hello from tree1".to_string(),
    };
    dialogue.apply_action(append_action).unwrap();

    // Test RenameBranch works across trees
    let rename_action = Action::RenameBranch {
        branch_id: branch2_id,
        old_name: "branch2".to_string(),
        new_name: "renamed_branch2".to_string(),
    };
    dialogue.apply_action(rename_action).unwrap();

    // Verify the message was added to branch1
    let branch1 = dialogue.get_branch_by_id(branch1_id).unwrap();
    assert_eq!(branch1.messages().len(), 1);
    assert_eq!(branch1.messages()[0].content(), "Hello from tree1");

    // Verify branch2 was renamed
    let branch2 = dialogue.get_branch_by_id(branch2_id).unwrap();
    assert_eq!(branch2.name(), "renamed_branch2");
}
