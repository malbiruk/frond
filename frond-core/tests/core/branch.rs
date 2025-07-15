use frond_core::{Action, Branch, Dialogue};
use uuid::Uuid;

#[test]
fn creates_branch_with_name() {
    let branch = Branch::new("main");
    assert_eq!(branch.name(), "main");
    assert!(branch.messages().is_empty());
}

#[test]
fn adds_and_removes_message() {
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

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "test".to_string(),
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let branch = tree.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.messages().len(), 1);
    let message_id = branch.messages()[0].id();

    dialogue
        .apply_action(Action::RemoveMessageById {
            branch_id,
            message_id,
            message_index: 0,
            removed_message: branch.messages()[0].clone(),
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let branch = tree.get_branch_by_id(branch_id).unwrap();
    assert!(branch.messages().is_empty());
}

#[test]
fn adds_multiple_messages() {
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

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "first".to_string(),
        })
        .unwrap();
    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "second".to_string(),
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let branch = tree.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.messages().len(), 2);
}

#[test]
fn clears_messages() {
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

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "test".to_string(),
        })
        .unwrap();

    dialogue
        .apply_action(Action::ClearMessages {
            branch_id,
            removed_messages: vec![],
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let branch = tree.get_branch_by_id(branch_id).unwrap();
    assert!(branch.messages().is_empty());
}

#[test]
fn renames_branch() {
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

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::RenameBranch {
            branch_id,
            old_name: "branch".to_string(),
            new_name: "renamed".to_string(),
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let branch = tree.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.name(), "renamed");
}

#[test]
fn sets_and_clears_branch_description() {
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

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::SetBranchDescription {
            branch_id,
            old_description: None,
            new_description: "desc".to_string(),
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let branch = tree.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.description(), Some("desc"));

    dialogue
        .apply_action(Action::ClearBranchDescription {
            branch_id,
            old_description: "desc".to_string(),
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let branch = tree.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.description(), None);
}

#[test]
fn gets_message_by_id() {
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

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "test".to_string(),
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let branch = tree.get_branch_by_id(branch_id).unwrap();
    let message_id = branch.messages()[0].id();

    let found = branch.get_message_by_id(message_id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().content(), "test");

    let not_found = branch.get_message_by_id(Uuid::new_v4());
    assert!(not_found.is_none());
}

#[test]
fn gets_message_by_id_mut() {
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

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "test".to_string(),
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let branch = tree.get_branch_by_id(branch_id).unwrap();
    let message_id = branch.messages()[0].id();

    let found = branch.get_message_by_id(message_id);
    assert!(found.is_some());

    let not_found = branch.get_message_by_id(Uuid::new_v4());
    assert!(not_found.is_none());
}

#[test]
fn gets_message_index_by_id() {
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

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "test".to_string(),
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let branch = tree.get_branch_by_id(branch_id).unwrap();
    let message_id = branch.messages()[0].id();

    let idx = branch.get_message_index_by_id(message_id);
    assert_eq!(idx, Some(0));

    let not_found = branch.get_message_index_by_id(Uuid::new_v4());
    assert!(not_found.is_none());
}

#[test]
fn fork_from_valid_message() {
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

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let branch = tree.get_branch_by_id(branch_id).unwrap();
    let fork_point_id = branch.messages()[1].id();

    dialogue
        .apply_action(Action::ForkBranch {
            tree_id,
            branch_id,
            from_message_id: fork_point_id,
            new_branch_name: "alt".to_string(),
        })
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let fork = tree.branches().iter().find(|b| b.name() == "alt").unwrap();
    assert_eq!(fork.messages().len(), 2);
    assert_eq!(fork.messages()[0].content(), "First");
    assert_eq!(fork.messages()[1].content(), "Second");
    assert_eq!(fork.name(), "alt");
}

#[test]
fn fork_from_invalid_message_returns_error() {
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

    let invalid_id = Uuid::new_v4();
    let result = dialogue.apply_action(Action::ForkBranch {
        tree_id,
        branch_id,
        from_message_id: invalid_id,
        new_branch_name: "alt".to_string(),
    });
    assert!(result.is_err());
}
