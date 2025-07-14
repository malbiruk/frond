use frond_core::{Action, Branch, Dialogue, Message, Role, Tree, TreeError};
use uuid::Uuid;

crate::test_core_entity!(Tree, branch, branches, Branch, Branch::new("test"));

#[test]
fn creates_tree_from_branch() {
    let branch = Branch::new("main");
    let tree = Tree::from_branch(branch.clone());
    assert_eq!(tree.name(), "main");
    assert_eq!(tree.branches().len(), 1);
    assert_eq!(tree.branches()[0].id(), branch.id());
}

#[test]
fn fork_branch_from_message_success() {
    let mut tree = Tree::new("My Tree");
    let mut branch = Branch::new("main");
    let msg1 = Message::new("First", Role::User);
    let msg2 = Message::new("Second", Role::Assistant);
    branch.add_message(msg1.clone());
    branch.add_message(msg2.clone());
    let branch_id = branch.id();
    let fork_point_id = msg2.id();

    tree.add_branch(branch);

    let forked = tree
        .fork_branch_from_message(branch_id, fork_point_id, "forked")
        .expect("Should fork branch");

    assert_eq!(forked.messages().len(), 2);
    assert_eq!(forked.messages()[0].content(), "First");
    assert_eq!(forked.messages()[1].content(), "Second");
    assert_eq!(forked.name(), "forked");
}

#[test]
fn fork_branch_from_message_branch_not_found() {
    let mut tree = Tree::new("My Tree");
    let fake_branch_id = Uuid::new_v4();
    let fake_message_id = Uuid::new_v4();

    let result = tree.fork_branch_from_message(fake_branch_id, fake_message_id, "forked");
    assert!(matches!(result, Err(TreeError::BranchNotFound(_))));
}

#[test]
fn fork_branch_from_message_message_not_found() {
    let mut tree = Tree::new("My Tree");
    let branch = Branch::new("main");
    let branch_id = branch.id();
    tree.add_branch(branch);

    let fake_message_id = Uuid::new_v4();
    let result = tree.fork_branch_from_message(branch_id, fake_message_id, "forked");
    assert!(matches!(result, Err(TreeError::Branch(_))));
}

#[test]
fn dialogue_rename_tree_through_action() {
    let tree = Tree::new("original");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let action = Action::RenameTree {
        tree_id,
        old_name: "original".to_string(),
        new_name: "renamed".to_string(),
    };

    dialogue.apply_action(action).unwrap();

    // Verify the tree was renamed
    let renamed_tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(renamed_tree.name(), "renamed");
}

#[test]
fn dialogue_rename_tree_not_found_through_action() {
    let tree = Tree::new("original");
    let mut dialogue = Dialogue::from_tree(tree);
    let fake_tree_id = Uuid::new_v4();

    let action = Action::RenameTree {
        tree_id: fake_tree_id,
        old_name: "original".to_string(),
        new_name: "renamed".to_string(),
    };

    let result = dialogue.apply_action(action);
    assert!(result.is_err());
}

#[test]
fn dialogue_fork_branch_through_action() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

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
        new_branch_name: "forked".to_string(),
    };

    dialogue.apply_action(action).unwrap();

    // Verify the branch was forked
    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 2);

    // Find the forked branch by name
    let forked_branch = tree
        .branches()
        .iter()
        .find(|b| b.name() == "forked")
        .expect("Forked branch should exist");
    assert_eq!(forked_branch.messages().len(), 2);
}

#[test]
fn dialogue_fork_branch_tree_not_found_through_action() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);
    let fake_tree_id = Uuid::new_v4();

    let action = Action::ForkBranch {
        tree_id: fake_tree_id,
        branch_id: Uuid::new_v4(),
        from_message_id: Uuid::new_v4(),
        new_branch_name: "forked".to_string(),
    };

    let result = dialogue.apply_action(action);
    assert!(result.is_err());
}

#[test]
fn dialogue_fork_branch_branch_not_found_through_action() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);
    let fake_branch_id = Uuid::new_v4();

    let action = Action::ForkBranch {
        tree_id,
        branch_id: fake_branch_id,
        from_message_id: Uuid::new_v4(),
        new_branch_name: "forked".to_string(),
    };

    let result = dialogue.apply_action(action);
    assert!(result.is_err());
}

#[test]
fn dialogue_undo_fork_branch_through_action() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let mut branch = Branch::new("main_branch");
    let msg = Message::new("Hello", Role::User);
    let msg_id = msg.id();
    branch.add_message(msg);
    let branch_id = branch.id();

    dialogue.trees_mut()[0].add_branch(branch);

    let action = Action::ForkBranch {
        tree_id,
        branch_id,
        from_message_id: msg_id,
        new_branch_name: "forked".to_string(),
    };

    dialogue.apply_action(action).unwrap();

    // Verify the branch was forked
    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 2);

    dialogue.undo_action().unwrap();

    // Verify the fork was undone
    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 1);
}

#[test]
fn dialogue_undo_fork_branch_tree_not_found_through_action() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);
    let fake_tree_id = Uuid::new_v4();

    let action = Action::ForkBranch {
        tree_id: fake_tree_id,
        branch_id: Uuid::new_v4(),
        from_message_id: Uuid::new_v4(),
        new_branch_name: "forked".to_string(),
    };

    let result = dialogue.apply_action(action);
    assert!(result.is_err());
}

#[test]
fn dialogue_undo_fork_branch_with_nonexistent_branch() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let mut branch = Branch::new("main_branch");
    let msg = Message::new("Hello", Role::User);
    let msg_id = msg.id();
    branch.add_message(msg);
    let branch_id = branch.id();

    dialogue.trees_mut()[0].add_branch(branch);

    let action = Action::ForkBranch {
        tree_id,
        branch_id,
        from_message_id: msg_id,
        new_branch_name: "nonexistent".to_string(),
    };

    dialogue.apply_action(action).unwrap();

    // Verify the branch was forked
    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 2);

    // Manually remove the branch to test undo with nonexistent branch
    let forked_branch_id = tree
        .branches()
        .iter()
        .find(|b| b.name() == "nonexistent")
        .unwrap()
        .id();
    dialogue.trees_mut()[0].remove_branch_by_id(forked_branch_id);

    // Should not error when trying to undo fork of nonexistent branch
    dialogue.undo_action().unwrap();
    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 1);
}
