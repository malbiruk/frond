use frond_core::{Action, BranchAction, Dialogue, MessageAction, Role, Tree, TreeAction};

#[test]
fn edit_message() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    // Add a branch first
    let add_branch_action = Action::Tree(TreeAction::AddBranch {
        tree_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "test_branch".to_string(),
    });
    dialogue.apply_action(add_branch_action).unwrap();

    let branch_id = dialogue.get_tree_by_id(tree_id).unwrap().branches()[0].id();

    // Add a message
    let append_action = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "Hello world".to_string(),
    });
    dialogue.apply_action(append_action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let message_id = branch.messages()[0].id();

    let action = Action::Message(MessageAction::EditMessage {
        message_id,
        old_content: "Hello world".to_string(),
        new_content: "Updated message".to_string(),
    });

    dialogue.apply_action(action).unwrap();

    let message = dialogue.get_message_by_id(message_id).unwrap();
    assert_eq!(message.content(), "Updated message");
}

#[test]
fn delete_message() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    // Add a branch first
    let add_branch_action = Action::Tree(TreeAction::AddBranch {
        tree_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "test_branch".to_string(),
    });
    dialogue.apply_action(add_branch_action).unwrap();

    let branch_id = dialogue.get_tree_by_id(tree_id).unwrap().branches()[0].id();

    // Add a message
    let append_action = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "Hello world".to_string(),
    });
    dialogue.apply_action(append_action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let message = branch.messages()[0].clone();
    let message_id = message.id();

    let action = Action::Message(MessageAction::DeleteMessage {
        message_id,
        branch_id,
        message_index: 0,
        deleted_message: message,
    });

    dialogue.apply_action(action).unwrap();

    assert!(dialogue.get_message_by_id(message_id).is_none());
}

#[test]
fn hide_message() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    // Add a branch first
    let add_branch_action = Action::Tree(TreeAction::AddBranch {
        tree_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "test_branch".to_string(),
    });
    dialogue.apply_action(add_branch_action).unwrap();

    let branch_id = dialogue.get_tree_by_id(tree_id).unwrap().branches()[0].id();

    // Add a message
    let append_action = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "Hello world".to_string(),
    });
    dialogue.apply_action(append_action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let message_id = branch.messages()[0].id();

    let action = Action::Message(MessageAction::HideMessage { message_id });

    dialogue.apply_action(action).unwrap();

    let message = dialogue.get_message_by_id(message_id).unwrap();
    assert!(message.is_hidden());
}

#[test]
fn show_message() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    // Add a branch first
    let add_branch_action = Action::Tree(TreeAction::AddBranch {
        tree_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "test_branch".to_string(),
    });
    dialogue.apply_action(add_branch_action).unwrap();

    let branch_id = dialogue.get_tree_by_id(tree_id).unwrap().branches()[0].id();

    // Add a message
    let append_action = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "Hello world".to_string(),
    });
    dialogue.apply_action(append_action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let message_id = branch.messages()[0].id();

    // Hide the message first
    let hide_action = Action::Message(MessageAction::HideMessage { message_id });
    dialogue.apply_action(hide_action).unwrap();

    // Then show it
    let show_action = Action::Message(MessageAction::ShowMessage { message_id });
    dialogue.apply_action(show_action).unwrap();

    let message = dialogue.get_message_by_id(message_id).unwrap();
    assert!(!message.is_hidden());
}

#[test]
fn toggle_message_role() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    // Add a branch first
    let add_branch_action = Action::Tree(TreeAction::AddBranch {
        tree_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "test_branch".to_string(),
    });
    dialogue.apply_action(add_branch_action).unwrap();

    let branch_id = dialogue.get_tree_by_id(tree_id).unwrap().branches()[0].id();

    // Add a message
    let append_action = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "Hello world".to_string(),
    });
    dialogue.apply_action(append_action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let message_id = branch.messages()[0].id();

    // Initially it should be User role
    let message = dialogue.get_message_by_id(message_id).unwrap();
    assert_eq!(message.role(), &Role::User);

    let action = Action::Message(MessageAction::ToggleMessageRole { message_id });
    dialogue.apply_action(action).unwrap();

    let message = dialogue.get_message_by_id(message_id).unwrap();
    assert_eq!(message.role(), &Role::Assistant);
}

#[test]
fn toggle_message_role_twice() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    // Add a branch first
    let add_branch_action = Action::Tree(TreeAction::AddBranch {
        tree_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "test_branch".to_string(),
    });
    dialogue.apply_action(add_branch_action).unwrap();

    let branch_id = dialogue.get_tree_by_id(tree_id).unwrap().branches()[0].id();

    // Add a message
    let append_action = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "Hello world".to_string(),
    });
    dialogue.apply_action(append_action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let message_id = branch.messages()[0].id();

    // Toggle once
    let action1 = Action::Message(MessageAction::ToggleMessageRole { message_id });
    dialogue.apply_action(action1).unwrap();

    // Toggle again
    let action2 = Action::Message(MessageAction::ToggleMessageRole { message_id });
    dialogue.apply_action(action2).unwrap();

    let message = dialogue.get_message_by_id(message_id).unwrap();
    assert_eq!(message.role(), &Role::User);
}

#[test]
fn message_actions_with_context() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    // Add a branch first
    let add_branch_action = Action::Tree(TreeAction::AddBranch {
        tree_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "test_branch".to_string(),
    });
    dialogue.apply_action(add_branch_action).unwrap();

    let branch_id = dialogue.get_tree_by_id(tree_id).unwrap().branches()[0].id();

    // Add multiple messages
    let append_action1 = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "Visible message".to_string(),
    });
    let append_action2 = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "Hidden message".to_string(),
    });
    dialogue.apply_action(append_action1).unwrap();
    dialogue.apply_action(append_action2).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let hidden_msg_id = branch.messages()[1].id();

    // Hide the second message
    let hide_action = Action::Message(MessageAction::HideMessage {
        message_id: hidden_msg_id,
    });
    dialogue.apply_action(hide_action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let context = branch.llm_context();
    assert_eq!(context.len(), 1);
    assert_eq!(context[0].content(), "Visible message");
}
