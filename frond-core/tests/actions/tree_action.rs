use frond_core::{Action, Branch, BranchAction, Dialogue, Tree, TreeAction};

#[test]
fn rename_tree() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let action = Action::Tree(TreeAction::RenameTree {
        tree_id,
        old_name: "main".to_string(),
        new_name: "renamed_tree".to_string(),
    });

    dialogue.apply_action(action).unwrap();

    let renamed_tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(renamed_tree.name(), "renamed_tree");
}

#[test]
fn set_tree_description() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let action = Action::Tree(TreeAction::SetTreeDescription {
        tree_id,
        old_description: None,
        new_description: "A test tree".to_string(),
    });

    dialogue.apply_action(action).unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.description(), Some("A test tree"));
}

#[test]
fn clear_tree_description() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    // First set a description
    let set_action = Action::Tree(TreeAction::SetTreeDescription {
        tree_id,
        old_description: None,
        new_description: "A test tree".to_string(),
    });
    dialogue.apply_action(set_action).unwrap();

    // Then clear it
    let clear_action = Action::Tree(TreeAction::ClearTreeDescription {
        tree_id,
        old_description: "A test tree".to_string(),
    });
    dialogue.apply_action(clear_action).unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.description(), None);
}

#[test]
fn fork_branch() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    // Add a branch first
    let add_branch_action = Action::Tree(TreeAction::AddBranch {
        tree_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "main_branch".to_string(),
    });
    dialogue.apply_action(add_branch_action).unwrap();

    let branch_id = dialogue.get_tree_by_id(tree_id).unwrap().branches()[0].id();

    // Add messages to the branch
    let append_action1 = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_content: "Hello".to_string(),
    });
    let append_action2 = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_content: "Hi there".to_string(),
    });
    dialogue.apply_action(append_action1).unwrap();
    dialogue.apply_action(append_action2).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let msg2_id = branch.messages()[1].id();

    let fork_action = Action::Tree(TreeAction::ForkBranch {
        tree_id,
        branch_id,
        from_message_id: msg2_id,
        new_branch_name: "forked_branch".to_string(),
    });

    dialogue.apply_action(fork_action).unwrap();

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
fn add_branch() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let action = Action::Tree(TreeAction::AddBranch {
        tree_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "new_branch".to_string(),
    });

    dialogue.apply_action(action).unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 1);
    assert_eq!(tree.branches()[0].name(), "new_branch");
}

#[test]
fn remove_branch() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    // Add a branch first
    let add_action = Action::Tree(TreeAction::AddBranch {
        tree_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "test_branch".to_string(),
    });
    dialogue.apply_action(add_action).unwrap();

    // Get the actual branch ID created by the action
    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let branch_id = tree.branches()[0].id();
    let branch_clone = tree.branches()[0].clone();

    // Remove the branch
    let remove_action = Action::Tree(TreeAction::RemoveBranch {
        tree_id,
        branch_id,
        removed_branch: branch_clone,
    });
    dialogue.apply_action(remove_action).unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 0);
    assert!(dialogue.get_branch_by_id(branch_id).is_none());
}

#[test]
fn clear_branches() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    // Add some branches
    let branch1 = Branch::new("branch1");
    let branch2 = Branch::new("branch2");
    let branches_clone = vec![branch1.clone(), branch2.clone()];

    let add_action1 = Action::Tree(TreeAction::AddBranch {
        tree_id,
        branch_id: branch1.id(),
        branch_name: "branch1".to_string(),
    });
    let add_action2 = Action::Tree(TreeAction::AddBranch {
        tree_id,
        branch_id: branch2.id(),
        branch_name: "branch2".to_string(),
    });
    dialogue.apply_action(add_action1).unwrap();
    dialogue.apply_action(add_action2).unwrap();

    // Clear all branches
    let clear_action = Action::Tree(TreeAction::ClearBranches {
        tree_id,
        removed_branches: branches_clone,
    });
    dialogue.apply_action(clear_action).unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 0);
}

#[test]
fn cross_tree_actions_work() {
    let tree1 = Tree::new("tree1");
    let tree2 = Tree::new("tree2");
    let tree1_id = tree1.id();
    let tree2_id = tree2.id();
    let mut dialogue = Dialogue::from_trees("dialogue", vec![tree1, tree2]);

    let add_branch_action1 = Action::Tree(TreeAction::AddBranch {
        tree_id: tree1_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "branch1".to_string(),
    });

    let add_branch_action2 = Action::Tree(TreeAction::AddBranch {
        tree_id: tree2_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "branch2".to_string(),
    });

    dialogue.apply_action(add_branch_action1).unwrap();
    dialogue.apply_action(add_branch_action2).unwrap();

    let tree1 = dialogue.get_tree_by_id(tree1_id).unwrap();
    let tree2 = dialogue.get_tree_by_id(tree2_id).unwrap();

    assert_eq!(tree1.branches().len(), 1);
    assert_eq!(tree1.branches()[0].name(), "branch1");
    assert_eq!(tree2.branches().len(), 1);
    assert_eq!(tree2.branches()[0].name(), "branch2");
}
