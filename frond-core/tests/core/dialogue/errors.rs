use frond_core::core::error::{BranchError, DialogueError, TreeError};
use frond_core::{Action, Dialogue, Role, Tree};
use uuid::Uuid;

#[test]
fn dispatch_action_apply_with_nonexistent_message() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let action = Action::EditMessage {
        message_id: Uuid::new_v4(),
        old_content: "old".to_string(),
        new_content: "new".to_string(),
    };

    assert!(dialogue.apply_action(action).is_err());
}

#[test]
fn dispatch_action_undo_with_nonexistent_message() {
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
            message_content: "Hello".to_string(),
        })
        .unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let message_id = branch.messages()[0].id();

    let action = Action::EditMessage {
        message_id,
        old_content: "Hello".to_string(),
        new_content: "Updated".to_string(),
    };

    dialogue.apply_action(action).unwrap();
    dialogue.undo_action().unwrap();
}

#[test]
fn branch_actions_with_nonexistent_branch() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let nonexistent_branch_id = Uuid::new_v4();

    let append_action = Action::AppendMessage {
        branch_id: nonexistent_branch_id,
        message_content: "Hello".to_string(),
    };
    assert!(dialogue.apply_action(append_action).is_err());

    let rename_action = Action::RenameBranch {
        branch_id: nonexistent_branch_id,
        old_name: "old".to_string(),
        new_name: "new".to_string(),
    };
    assert!(dialogue.apply_action(rename_action).is_err());
}

#[test]
fn message_actions_with_nonexistent_message() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let nonexistent_message_id = Uuid::new_v4();
    let fake_message = frond_core::Message::new("fake", Role::User);

    let edit_action = Action::EditMessage {
        message_id: nonexistent_message_id,
        old_content: "old".to_string(),
        new_content: "new".to_string(),
    };
    assert!(dialogue.apply_action(edit_action).is_err());

    let delete_action = Action::DeleteMessage {
        message_id: nonexistent_message_id,
        branch_id: Uuid::new_v4(),
        message_index: 0,
        deleted_message: fake_message.clone(),
    };
    assert!(dialogue.apply_action(delete_action).is_err());

    let toggle_action = Action::ToggleMessageRole {
        message_id: nonexistent_message_id,
    };
    assert!(dialogue.apply_action(toggle_action).is_err());
}

#[test]
fn tree_actions_with_nonexistent_tree() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let nonexistent_tree_id = Uuid::new_v4();

    let rename_action = Action::RenameTree {
        tree_id: nonexistent_tree_id,
        old_name: "old".to_string(),
        new_name: "new".to_string(),
    };
    assert!(dialogue.apply_action(rename_action).is_err());

    let fork_action = Action::ForkBranch {
        tree_id: nonexistent_tree_id,
        branch_id: Uuid::new_v4(),
        from_message_id: Uuid::new_v4(),
        new_branch_name: "new_branch".to_string(),
    };
    assert!(dialogue.apply_action(fork_action).is_err());
}

#[test]
fn error_hierarchy_demonstration() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let nonexistent_message_id = Uuid::new_v4();
    let action = Action::EditMessage {
        message_id: nonexistent_message_id,
        old_content: "old".to_string(),
        new_content: "new".to_string(),
    };

    let result = dialogue.apply_action(action);
    assert!(result.is_err());

    let error = result.unwrap_err();
    match error {
        DialogueError::Tree(TreeError::Branch(BranchError::MessageNotFound(id))) => {
            assert_eq!(id, nonexistent_message_id);
        }
        _ => {
            panic!("Expected DialogueError::Tree(TreeError::Branch(BranchError::MessageNotFound))")
        }
    }
}

#[test]
fn delete_message_with_invalid_location() {
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

    let wrong_branch_id = Uuid::new_v4();
    let action = Action::DeleteMessage {
        message_id,
        branch_id: wrong_branch_id,
        message_index: 0,
        deleted_message: message,
    };

    let result = dialogue.apply_action(action);
    assert!(result.is_err());
    match result.unwrap_err() {
        DialogueError::Tree(TreeError::BranchNotFound(id)) => {
            assert_eq!(id, wrong_branch_id);
        }
        _ => panic!("Expected BranchNotFound error"),
    }
}

#[test]
fn delete_message_with_invalid_index() {
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
        message_index: 999,
        deleted_message: message,
    };

    let result = dialogue.apply_action(action);
    assert!(result.is_err());
    match result.unwrap_err() {
        DialogueError::Tree(TreeError::Branch(BranchError::MessageNotFound(id))) => {
            assert_eq!(id, message_id);
        }
        _ => panic!("Expected MessageNotFound error"),
    }
}

#[test]
fn fork_branch_with_invalid_message() {
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

    let nonexistent_message_id = Uuid::new_v4();
    let action = Action::ForkBranch {
        tree_id,
        branch_id,
        from_message_id: nonexistent_message_id,
        new_branch_name: "forked".to_string(),
    };

    let result = dialogue.apply_action(action);
    assert!(result.is_err());
    match result.unwrap_err() {
        DialogueError::Tree(TreeError::Branch(BranchError::MessageNotFound(id))) => {
            assert_eq!(id, nonexistent_message_id);
        }
        _ => panic!("Expected MessageNotFound error"),
    }
}

#[test]
fn fork_branch_with_invalid_branch() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let nonexistent_branch_id = Uuid::new_v4();
    let action = Action::ForkBranch {
        tree_id,
        branch_id: nonexistent_branch_id,
        from_message_id: Uuid::new_v4(),
        new_branch_name: "forked".to_string(),
    };

    let result = dialogue.apply_action(action);
    assert!(result.is_err());
    match result.unwrap_err() {
        DialogueError::Tree(TreeError::BranchNotFound(id)) => {
            assert_eq!(id, nonexistent_branch_id);
        }
        _ => panic!("Expected BranchNotFound error"),
    }
}
