use frond_core::{Action, Branch, Dialogue, Tree};
use uuid::Uuid;

#[test]
fn creates_tree_with_name() {
    let tree = Tree::new("My Tree");
    assert_eq!(tree.name(), "My Tree");
    assert!(tree.branches().is_empty());
}

#[test]
fn creates_tree_from_branch() {
    let branch = Branch::new("main");
    let tree = Tree::from_branch(branch.clone());
    assert_eq!(tree.name(), "main");
    assert_eq!(tree.branches().len(), 1);
    assert_eq!(tree.branches()[0].id(), branch.id());
}

#[test]
fn adds_and_removes_branch() {
    let mut dialogue = Dialogue::new("Test");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "tree".to_string(),
        })
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "branch".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    assert_eq!(tree.branches().len(), 1);

    let branch_id = tree.branches()[0].id();
    dialogue
        .apply_action(Action::RemoveBranch {
            tree_id,
            branch_id,
            removed_branch: tree.branches()[0].clone(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    assert!(tree.branches().is_empty());
}

#[test]
fn adds_multiple_branches() {
    let mut dialogue = Dialogue::new("Test");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "tree".to_string(),
        })
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "branch1".to_string(),
        })
        .unwrap();
    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "branch2".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    assert_eq!(tree.branches().len(), 2);
}

#[test]
fn clears_branches() {
    let mut dialogue = Dialogue::new("Test");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "tree".to_string(),
        })
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "branch".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    dialogue
        .apply_action(Action::ClearBranches {
            tree_id,
            removed_branches: tree.branches().to_vec(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    assert!(tree.branches().is_empty());
}

#[test]
fn renames_tree() {
    let mut dialogue = Dialogue::new("Test");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "tree".to_string(),
        })
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    dialogue
        .apply_action(Action::RenameTree {
            tree_id,
            old_name: "tree".to_string(),
            new_name: "renamed".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    assert_eq!(tree.name(), "renamed");
}

#[test]
fn sets_and_clears_tree_description() {
    let mut dialogue = Dialogue::new("Test");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "tree".to_string(),
        })
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    dialogue
        .apply_action(Action::SetTreeDescription {
            tree_id,
            old_description: None,
            new_description: "desc".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    assert_eq!(tree.description(), Some("desc"));

    dialogue
        .apply_action(Action::ClearTreeDescription {
            tree_id,
            old_description: "desc".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    assert_eq!(tree.description(), None);
}

#[test]
fn gets_branch_by_id() {
    let mut dialogue = Dialogue::new("Test");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "tree".to_string(),
        })
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "branch".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    let branch_id = tree.branches()[0].id();

    let found = tree.branches().iter().find(|b| b.id() == branch_id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "branch");

    let not_found = tree.branches().iter().find(|b| b.id() == Uuid::new_v4());
    assert!(not_found.is_none());
}

#[test]
fn gets_branch_by_id_mut() {
    let mut dialogue = Dialogue::new("Test");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "tree".to_string(),
        })
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "branch".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    let branch_id = tree.branches()[0].id();

    let found = tree.branches().iter().find(|b| b.id() == branch_id);
    assert!(found.is_some());

    let not_found = tree.branches().iter().find(|b| b.id() == Uuid::new_v4());
    assert!(not_found.is_none());
}

#[test]
fn gets_branch_index_by_id() {
    let mut dialogue = Dialogue::new("Test");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "tree".to_string(),
        })
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "branch".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    let branch_id = tree.branches()[0].id();

    let idx = tree.branches().iter().position(|b| b.id() == branch_id);
    assert_eq!(idx, Some(0));

    let not_found = tree
        .branches()
        .iter()
        .position(|b| b.id() == Uuid::new_v4());
    assert!(not_found.is_none());
}

#[test]
fn fork_branch_from_message_success() {
    let mut dialogue = Dialogue::new("Test");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "tree".to_string(),
        })
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "main".to_string(),
        })
        .unwrap();

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "First".to_string(),
        })
        .unwrap();
    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "Second".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    let branch = tree
        .branches()
        .iter()
        .find(|b| b.id() == branch_id)
        .unwrap();
    let fork_point_id = branch.messages()[1].id();

    dialogue
        .apply_action(Action::ForkBranch {
            tree_id,
            branch_id,
            from_message_id: fork_point_id,
            new_branch_name: "alt".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    let fork = tree.branches().iter().find(|b| b.name() == "alt").unwrap();
    assert_eq!(fork.messages().len(), 2);
    assert_eq!(fork.messages()[0].content(), "First");
    assert_eq!(fork.messages()[1].content(), "Second");
}

#[test]
fn fork_branch_from_message_branch_not_found() {
    let mut dialogue = Dialogue::new("Test");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "tree".to_string(),
        })
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    let result = dialogue.apply_action(Action::ForkBranch {
        tree_id,
        branch_id: Uuid::new_v4(),
        from_message_id: Uuid::new_v4(),
        new_branch_name: "alt".to_string(),
    });
    assert!(result.is_err());
}

#[test]
fn fork_branch_from_message_message_not_found() {
    let mut dialogue = Dialogue::new("Test");

    dialogue
        .apply_action(Action::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "tree".to_string(),
        })
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    dialogue
        .apply_action(Action::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "main".to_string(),
        })
        .unwrap();

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "Hello".to_string(),
        })
        .unwrap();

    let result = dialogue.apply_action(Action::ForkBranch {
        tree_id,
        branch_id,
        from_message_id: Uuid::new_v4(),
        new_branch_name: "alt".to_string(),
    });
    assert!(result.is_err());
}

#[test]
fn dialogue_rename_tree_through_action() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    dialogue
        .apply_action(Action::RenameTree {
            tree_id,
            old_name: "main".to_string(),
            new_name: "renamed".to_string(),
        })
        .unwrap();

    assert_eq!(dialogue.get_tree_by_id(tree_id).unwrap().name(), "renamed");
}

#[test]
fn dialogue_rename_tree_not_found_through_action() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let result = dialogue.apply_action(Action::RenameTree {
        tree_id: Uuid::new_v4(),
        old_name: "main".to_string(),
        new_name: "renamed".to_string(),
    });
    assert!(result.is_err());
}

#[test]
fn dialogue_fork_branch_through_action() {
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

    dialogue
        .apply_action(Action::ForkBranch {
            tree_id,
            branch_id,
            from_message_id: msg2_id,
            new_branch_name: "forked_branch".to_string(),
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 2);

    let forked_branch = tree
        .branches()
        .iter()
        .find(|b| b.name() == "forked_branch")
        .unwrap();
    assert_eq!(forked_branch.messages().len(), 2);
}

#[test]
fn dialogue_fork_branch_tree_not_found_through_action() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let result = dialogue.apply_action(Action::ForkBranch {
        tree_id: Uuid::new_v4(),
        branch_id: Uuid::new_v4(),
        from_message_id: Uuid::new_v4(),
        new_branch_name: "forked".to_string(),
    });
    assert!(result.is_err());
}

#[test]
fn dialogue_fork_branch_branch_not_found_through_action() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let result = dialogue.apply_action(Action::ForkBranch {
        tree_id,
        branch_id: Uuid::new_v4(),
        from_message_id: Uuid::new_v4(),
        new_branch_name: "forked".to_string(),
    });
    assert!(result.is_err());
}

#[test]
fn dialogue_undo_fork_branch_through_action() {
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

    dialogue
        .apply_action(Action::ForkBranch {
            tree_id,
            branch_id,
            from_message_id: msg2_id,
            new_branch_name: "forked_branch".to_string(),
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 2);

    dialogue.undo_action().unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 1);
}

#[test]
fn dialogue_undo_fork_branch_tree_not_found_through_action() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let result = dialogue.apply_action(Action::ForkBranch {
        tree_id: Uuid::new_v4(),
        branch_id: Uuid::new_v4(),
        from_message_id: Uuid::new_v4(),
        new_branch_name: "forked".to_string(),
    });
    assert!(result.is_err());
}

#[test]
fn dialogue_undo_fork_branch_with_nonexistent_branch() {
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

    dialogue
        .apply_action(Action::ForkBranch {
            tree_id,
            branch_id,
            from_message_id: msg2_id,
            new_branch_name: "forked_branch".to_string(),
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 2);

    dialogue.undo_action().unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 1);
}
