use frond_core::core::error::{BranchError, DialogueError, TreeError};
use frond_core::{Action, Branch, Dialogue, Message, Role, Tree};
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

    // Should error for nonexistent message
    assert!(dialogue.apply_action(action).is_err());
}

#[test]
fn dispatch_action_undo_with_nonexistent_message() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let mut branch = Branch::new("test_branch");
    let message = Message::new("Hello", Role::User);
    let message_id = message.id();
    branch.add_message(message);
    dialogue.trees_mut()[0].add_branch(branch);

    let action = Action::EditMessage {
        message_id,
        old_content: "Hello".to_string(),
        new_content: "Updated".to_string(),
    };

    dialogue.apply_action(action).unwrap();
    // Should succeed for valid message
    dialogue.undo_action().unwrap();
}

#[test]
fn branch_actions_with_nonexistent_branch() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let nonexistent_branch_id = Uuid::new_v4();

    // Test AppendMessage with nonexistent branch
    let append_action = Action::AppendMessage {
        branch_id: nonexistent_branch_id,
        message_content: "Hello".to_string(),
    };
    assert!(dialogue.apply_action(append_action).is_err());

    // Test RenameBranch with nonexistent branch
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
    let fake_message = Message::new("fake", Role::User);

    // Test EditMessage with nonexistent message
    let edit_action = Action::EditMessage {
        message_id: nonexistent_message_id,
        old_content: "old".to_string(),
        new_content: "new".to_string(),
    };
    assert!(dialogue.apply_action(edit_action).is_err());

    // Test DeleteMessage with nonexistent message
    let delete_action = Action::DeleteMessage {
        message_id: nonexistent_message_id,
        branch_id: Uuid::new_v4(), // Nonexistent branch
        message_index: 0,          // Invalid index
        deleted_message: fake_message.clone(),
    };
    assert!(dialogue.apply_action(delete_action).is_err());

    // Test ToggleMessageRole with nonexistent message
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

    // Test RenameTree with nonexistent tree
    let rename_action = Action::RenameTree {
        tree_id: nonexistent_tree_id,
        old_name: "old".to_string(),
        new_name: "new".to_string(),
    };
    assert!(dialogue.apply_action(rename_action).is_err());

    // Test ForkBranch with nonexistent tree
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

    // Verify the error follows the proper hierarchy:
    // DialogueError -> TreeError -> BranchError -> MessageNotFound
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
    let mut dialogue = Dialogue::from_tree(tree);

    let mut branch = Branch::new("test_branch");
    let message = Message::new("Hello world", Role::User);
    let message_id = message.id();
    branch.add_message(message.clone());
    dialogue.trees_mut()[0].add_branch(branch);

    // Try to delete with wrong branch_id
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
    let mut dialogue = Dialogue::from_tree(tree);

    let mut branch = Branch::new("test_branch");
    let message = Message::new("Hello world", Role::User);
    let message_id = message.id();
    branch.add_message(message.clone());
    let branch_id = branch.id();
    dialogue.trees_mut()[0].add_branch(branch);

    // Try to delete with wrong message_index
    let action = Action::DeleteMessage {
        message_id,
        branch_id,
        message_index: 999, // Invalid index
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

    let mut branch = Branch::new("test_branch");
    let message = Message::new("Hello world", Role::User);
    branch.add_message(message);
    let branch_id = branch.id();
    dialogue.trees_mut()[0].add_branch(branch);

    // Try to fork from nonexistent message
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
