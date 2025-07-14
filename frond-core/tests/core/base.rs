use frond_core::{Branch, Dialogue, Message, Role, Tree};

#[test]
fn can_create_dialogue_tree_branch_and_messages() {
    let mut dialogue = Dialogue::new("Test Dialogue");
    let tree = Tree::new("Initial Plan");
    let tree_id = tree.id();
    dialogue.add_tree(tree);

    let mut branch = Branch::new("main");
    let branch_id = branch.id();

    branch.add_message(Message::new("Hello, LLM!", Role::User));
    branch.add_message(Message::new("Hi, user!", Role::Assistant));

    dialogue
        .get_tree_by_id_mut(tree_id)
        .unwrap()
        .add_branch(branch);

    // Verify the structure was created successfully
    let created_branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(created_branch.messages().len(), 2);
    assert_eq!(created_branch.messages()[0].content(), "Hello, LLM!");
    assert_eq!(created_branch.messages()[1].content(), "Hi, user!");
}

#[test]
fn can_fork_branch_from_message() {
    let mut branch = Branch::new("main");
    let first_msg = Message::new("First", Role::User);
    let second_msg = Message::new("Second", Role::Assistant);
    let fork_point_id = second_msg.id();

    branch.add_message(first_msg);
    branch.add_message(second_msg);

    let fork = branch
        .fork_from(fork_point_id, "alt")
        .expect("Fork should succeed");

    // Verify fork has correct content and name
    assert_eq!(fork.messages().len(), 2);
    assert_eq!(fork.messages()[0].content(), "First");
    assert_eq!(fork.messages()[1].content(), "Second");
    assert_eq!(fork.name(), "alt");
}

#[test]
fn hidden_messages_are_excluded_from_context() {
    let mut branch = Branch::new("main");
    let visible_msg = Message::new("Visible", Role::User);
    let mut hidden_msg = Message::new("Hidden", Role::User);

    // Hide the second message before adding
    hidden_msg.hide();

    branch.add_message(visible_msg);
    branch.add_message(hidden_msg);

    // Verify only visible messages appear in context
    let context = branch.llm_context();
    assert_eq!(context.len(), 1);
    assert_eq!(context[0].content(), "Visible");
}

// #[test]
// fn can_merge_and_split_messages() {
//     let mut branch = Branch::new("main");
//     branch.add_message(Role::User, "Hello");
//     branch.add_message(Role::User, "World");
//     branch.merge_messages(0, 1);
//     assert_eq!(branch.messages()[0].content, "Hello\nWorld");
//     branch.split_message(0, 5); // split after "Hello"
//     assert_eq!(branch.messages()[0].content, "Hello");
//     assert_eq!(branch.messages()[1].content, "\nWorld");
// }

// #[test]
// fn token_count_excludes_hidden() {
//     let mut branch = Branch::new("main");
//     branch.add_message(Role::User, "A");
//     branch.add_message(Role::User, "B");
//     branch.hide_message(1);
//     assert_eq!(branch.token_count(), count_tokens("A"));
// }
