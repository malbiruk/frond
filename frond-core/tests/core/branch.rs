use frond_core::{Action, Branch, Dialogue, Message, Role, Tree, TreeAction};

#[test]
fn creates_branch_with_name() {
    let branch = Branch::new("main");

    assert_eq!(branch.name(), "main");
    assert!(branch.messages().is_empty());
    assert_eq!(branch.description(), None);
}

#[test]
fn creates_branch_with_empty_description() {
    let branch = Branch::new("feature");

    assert_eq!(branch.name(), "feature");
    assert_eq!(branch.description(), None);
}

#[test]
fn branch_has_unique_id() {
    let branch1 = Branch::new("main");
    let branch2 = Branch::new("main");

    assert_ne!(branch1.id(), branch2.id());
}

#[test]
fn creates_branch_from_messages() {
    let msg1 = Message::new("First", Role::User);
    let msg2 = Message::new("Second", Role::Assistant);
    let messages = vec![msg1, msg2];

    let branch = Branch::from_messages("test", messages);

    assert_eq!(branch.name(), "test");
    assert_eq!(branch.messages().len(), 2);
    assert_eq!(branch.messages()[0].content(), "First");
    assert_eq!(branch.messages()[1].content(), "Second");
}

#[test]
fn creates_empty_branch_from_empty_messages() {
    let messages = vec![];
    let branch = Branch::from_messages("empty", messages);

    assert_eq!(branch.name(), "empty");
    assert!(branch.messages().is_empty());
}

#[test]
fn finds_message_by_id() {
    let msg1 = Message::new("First", Role::User);
    let msg2 = Message::new("Second", Role::Assistant);
    let msg1_id = msg1.id();
    let msg2_id = msg2.id();
    let messages = vec![msg1, msg2];

    let branch = Branch::from_messages("test", messages);

    let found1 = branch.get_message_by_id(msg1_id);
    let found2 = branch.get_message_by_id(msg2_id);

    assert!(found1.is_some());
    assert_eq!(found1.unwrap().content(), "First");
    assert!(found2.is_some());
    assert_eq!(found2.unwrap().content(), "Second");
}

#[test]
fn returns_none_for_nonexistent_message() {
    let branch = Branch::new("main");
    let fake_id = uuid::Uuid::new_v4();

    let found = branch.get_message_by_id(fake_id);
    assert!(found.is_none());
}

#[test]
fn gets_message_index_by_id() {
    let msg1 = Message::new("First", Role::User);
    let msg2 = Message::new("Second", Role::Assistant);
    let msg1_id = msg1.id();
    let msg2_id = msg2.id();
    let messages = vec![msg1, msg2];

    let branch = Branch::from_messages("test", messages);

    let index1 = branch.get_message_index_by_id(msg1_id);
    let index2 = branch.get_message_index_by_id(msg2_id);

    assert_eq!(index1, Some(0));
    assert_eq!(index2, Some(1));
}

#[test]
fn returns_none_for_nonexistent_message_index() {
    let branch = Branch::new("main");
    let fake_id = uuid::Uuid::new_v4();

    let index = branch.get_message_index_by_id(fake_id);
    assert!(index.is_none());
}

#[test]
fn llm_context_includes_all_visible_messages() {
    let msg1 = Message::new("First", Role::User);
    let msg2 = Message::new("Second", Role::Assistant);

    let messages = vec![msg1, msg2];
    let branch = Branch::from_messages("test", messages);

    let context = branch.llm_context();

    assert_eq!(context.len(), 2);
    assert_eq!(context[0].content(), "First");
    assert_eq!(context[1].content(), "Second");
    assert!(!context[0].is_hidden());
    assert!(!context[1].is_hidden());
}

#[test]
fn llm_context_preserves_message_order() {
    let msg1 = Message::new("First", Role::User);
    let msg2 = Message::new("Second", Role::Assistant);
    let msg3 = Message::new("Third", Role::User);

    let messages = vec![msg1, msg2, msg3];
    let branch = Branch::from_messages("test", messages);

    let context = branch.llm_context();

    assert_eq!(context.len(), 3);
    assert_eq!(context[0].content(), "First");
    assert_eq!(context[1].content(), "Second");
    assert_eq!(context[2].content(), "Third");
}

#[test]
fn llm_context_returns_empty_for_empty_branch() {
    let branch = Branch::new("empty");
    let context = branch.llm_context();

    assert!(context.is_empty());
}

#[test]
fn llm_context_with_single_message() {
    let msg = Message::new("Single message", Role::User);
    let messages = vec![msg];
    let branch = Branch::from_messages("test", messages);

    let context = branch.llm_context();
    assert_eq!(context.len(), 1);
    assert_eq!(context[0].content(), "Single message");
    assert_eq!(context[0].role(), &Role::User);
}

#[test]
fn forks_branch_from_message() {
    let msg1 = Message::new("First", Role::User);
    let msg2 = Message::new("Second", Role::Assistant);
    let msg3 = Message::new("Third", Role::User);
    let msg2_id = msg2.id();

    let messages = vec![msg1, msg2, msg3];
    let branch = Branch::from_messages("original", messages);
    let branch_id = branch.id();
    let tree = Tree::from_branch(branch);
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let fork_action = Action::Tree(TreeAction::ForkBranch {
        tree_id,
        branch_id,
        from_message_id: msg2_id,
        new_branch_name: "feature".to_string(),
    });

    dialogue.apply_action(fork_action).unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 2);

    let fork = tree
        .branches()
        .iter()
        .find(|b| b.name() == "feature")
        .unwrap();
    assert_eq!(fork.name(), "feature");
    assert_eq!(fork.messages().len(), 2);
    assert_eq!(fork.messages()[0].content(), "First");
    assert_eq!(fork.messages()[1].content(), "Second");
}

#[test]
fn fork_from_first_message() {
    let msg1 = Message::new("Only", Role::User);
    let msg2 = Message::new("Two", Role::Assistant);
    let msg1_id = msg1.id();

    let messages = vec![msg1, msg2];
    let branch = Branch::from_messages("original", messages);
    let branch_id = branch.id();
    let tree = Tree::from_branch(branch);
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let fork_action = Action::Tree(TreeAction::ForkBranch {
        tree_id,
        branch_id,
        from_message_id: msg1_id,
        new_branch_name: "feature".to_string(),
    });

    dialogue.apply_action(fork_action).unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    assert_eq!(tree.branches().len(), 2);

    let fork = tree
        .branches()
        .iter()
        .find(|b| b.name() == "feature")
        .unwrap();
    assert_eq!(fork.name(), "feature");
    assert_eq!(fork.messages().len(), 1);
    assert_eq!(fork.messages()[0].content(), "Only");
}

#[test]
fn fork_fails_with_invalid_message_id() {
    let msg = Message::new("Test", Role::User);
    let messages = vec![msg];
    let branch = Branch::from_messages("original", messages);
    let branch_id = branch.id();
    let tree = Tree::from_branch(branch);
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let fake_id = uuid::Uuid::new_v4();
    let fork_action = Action::Tree(TreeAction::ForkBranch {
        tree_id,
        branch_id,
        from_message_id: fake_id,
        new_branch_name: "feature".to_string(),
    });

    let result = dialogue.apply_action(fork_action);
    assert!(result.is_err());
}

#[test]
fn fork_from_empty_branch_fails() {
    let branch = Branch::new("empty");
    let branch_id = branch.id();
    let tree = Tree::from_branch(branch);
    let tree_id = tree.id();
    let mut dialogue = Dialogue::from_tree(tree);

    let fake_id = uuid::Uuid::new_v4();
    let fork_action = Action::Tree(TreeAction::ForkBranch {
        tree_id,
        branch_id,
        from_message_id: fake_id,
        new_branch_name: "feature".to_string(),
    });

    let result = dialogue.apply_action(fork_action);
    assert!(result.is_err());
}

#[test]
fn branch_clone_preserves_all_data() {
    let msg = Message::new("Test", Role::User);
    let messages = vec![msg];
    let branch = Branch::from_messages("original", messages);

    let cloned = branch.clone();

    assert_eq!(branch.id(), cloned.id());
    assert_eq!(branch.name(), cloned.name());
    assert_eq!(branch.description(), cloned.description());
    assert_eq!(branch.messages().len(), cloned.messages().len());
    assert_eq!(branch.messages()[0].id(), cloned.messages()[0].id());
}

#[test]
fn branch_messages_are_accessible() {
    let msg1 = Message::new("First", Role::User);
    let msg2 = Message::new("Second", Role::Assistant);
    let messages = vec![msg1, msg2];

    let branch = Branch::from_messages("test", messages);

    assert_eq!(branch.messages().len(), 2);
    assert_eq!(branch.messages()[0].content(), "First");
    assert_eq!(branch.messages()[1].content(), "Second");
    assert_eq!(branch.messages()[0].role(), &Role::User);
    assert_eq!(branch.messages()[1].role(), &Role::Assistant);
}

#[test]
fn branch_supports_different_name_types() {
    let branch1 = Branch::new("string_literal");
    let branch2 = Branch::new(String::from("owned_string"));

    assert_eq!(branch1.name(), "string_literal");
    assert_eq!(branch2.name(), "owned_string");
}

#[test]
fn branch_debug_format() {
    let branch = Branch::new("debug_test");
    let debug_str = format!("{:?}", branch);

    assert!(debug_str.contains("debug_test"));
}
