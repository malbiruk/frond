use frond_core::{Action, BranchAction, Dialogue, Message, Role, Tree, TreeAction};

#[test]
fn rename_branch() {
    let tree = Tree::new("main");
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    // Add a branch first
    let add_branch_action = Action::Tree(TreeAction::AddBranch {
        tree_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "old_name".to_string(),
    });
    dialogue.apply_action(add_branch_action).unwrap();

    let branch_id = dialogue.get_tree_by_id(tree_id).unwrap().branches()[0].id();

    let action = Action::Branch(BranchAction::RenameBranch {
        branch_id,
        old_name: "old_name".to_string(),
        new_name: "new_name".to_string(),
    });

    dialogue.apply_action(action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.name(), "new_name");
}

#[test]
fn set_branch_description() {
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

    let action = Action::Branch(BranchAction::SetBranchDescription {
        branch_id,
        old_description: None,
        new_description: "A test branch".to_string(),
    });

    dialogue.apply_action(action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.description(), Some("A test branch"));
}

#[test]
fn clear_branch_description() {
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

    // Set description first
    let set_action = Action::Branch(BranchAction::SetBranchDescription {
        branch_id,
        old_description: None,
        new_description: "A test branch".to_string(),
    });
    dialogue.apply_action(set_action).unwrap();

    // Clear description
    let clear_action = Action::Branch(BranchAction::ClearBranchDescription {
        branch_id,
        old_description: "A test branch".to_string(),
    });
    dialogue.apply_action(clear_action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.description(), None);
}

#[test]
fn append_message() {
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

    let message_id = uuid::Uuid::new_v4();
    let action = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id,
        message_content: "Hello world".to_string(),
    });

    dialogue.apply_action(action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.messages().len(), 1);
    assert_eq!(branch.messages()[0].content(), "Hello world");
    assert_eq!(branch.messages()[0].role(), &Role::User);
}

#[test]
fn insert_message_at_index() {
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

    // Add some messages first
    let message_id1 = uuid::Uuid::new_v4();
    let append_action1 = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: message_id1,
        message_content: "First".to_string(),
    });
    let message_id2 = uuid::Uuid::new_v4();
    let append_action2 = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: message_id2,
        message_content: "Third".to_string(),
    });
    dialogue.apply_action(append_action1).unwrap();
    dialogue.apply_action(append_action2).unwrap();

    // Insert a message at index 1
    let message = Message::new("Second", Role::Assistant);
    let insert_action = Action::Branch(BranchAction::InsertMessageAtIndex {
        branch_id,
        message_index: 1,
        message,
    });
    dialogue.apply_action(insert_action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.messages().len(), 3);
    assert_eq!(branch.messages()[0].content(), "First");
    assert_eq!(branch.messages()[1].content(), "Second");
    assert_eq!(branch.messages()[2].content(), "Third");
}

#[test]
fn remove_message_by_id() {
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

    // Add some messages
    let append_action1 = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "First".to_string(),
    });
    let append_action2 = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "Second".to_string(),
    });
    dialogue.apply_action(append_action1).unwrap();
    dialogue.apply_action(append_action2).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let message_to_remove = branch.messages()[0].clone();
    let message_id = message_to_remove.id();

    let remove_action = Action::Branch(BranchAction::RemoveMessageById {
        branch_id,
        message_id,
        message_index: 0,
        removed_message: message_to_remove,
    });
    dialogue.apply_action(remove_action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.messages().len(), 1);
    assert_eq!(branch.messages()[0].content(), "Second");
    assert!(dialogue.get_message_by_id(message_id).is_none());
}

#[test]
fn clear_messages() {
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

    // Add some messages
    let append_action1 = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "First".to_string(),
    });
    let append_action2 = Action::Branch(BranchAction::AppendMessage {
        branch_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "Second".to_string(),
    });
    dialogue.apply_action(append_action1).unwrap();
    dialogue.apply_action(append_action2).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let messages_to_remove = branch.messages().to_vec();

    let clear_action = Action::Branch(BranchAction::ClearMessages {
        branch_id,
        removed_messages: messages_to_remove,
    });
    dialogue.apply_action(clear_action).unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(branch.messages().len(), 0);
}

#[test]
fn multiple_branches_different_trees() {
    let tree1 = Tree::new("tree1");
    let tree2 = Tree::new("tree2");
    let tree1_id = tree1.id();
    let tree2_id = tree2.id();
    let mut dialogue = Dialogue::from_trees("dialogue", vec![tree1, tree2]);

    // Add branches to both trees
    let add_branch1_action = Action::Tree(TreeAction::AddBranch {
        tree_id: tree1_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "branch1".to_string(),
    });
    let add_branch2_action = Action::Tree(TreeAction::AddBranch {
        tree_id: tree2_id,
        branch_id: uuid::Uuid::new_v4(),
        branch_name: "branch2".to_string(),
    });
    dialogue.apply_action(add_branch1_action).unwrap();
    dialogue.apply_action(add_branch2_action).unwrap();

    let branch1_id = dialogue.get_tree_by_id(tree1_id).unwrap().branches()[0].id();
    let branch2_id = dialogue.get_tree_by_id(tree2_id).unwrap().branches()[0].id();

    // Add messages to both branches
    let append_action1 = Action::Branch(BranchAction::AppendMessage {
        branch_id: branch1_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "Hello from tree1".to_string(),
    });
    let append_action2 = Action::Branch(BranchAction::AppendMessage {
        branch_id: branch2_id,
        message_id: uuid::Uuid::new_v4(),
        message_content: "Hello from tree2".to_string(),
    });
    dialogue.apply_action(append_action1).unwrap();
    dialogue.apply_action(append_action2).unwrap();

    let branch1 = dialogue.get_branch_by_id(branch1_id).unwrap();
    let branch2 = dialogue.get_branch_by_id(branch2_id).unwrap();

    assert_eq!(branch1.messages().len(), 1);
    assert_eq!(branch1.messages()[0].content(), "Hello from tree1");
    assert_eq!(branch2.messages().len(), 1);
    assert_eq!(branch2.messages()[0].content(), "Hello from tree2");
}
