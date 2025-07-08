use frond_core::{Branch, Dialogue, Message, Role, Tree};

#[test]
fn can_create_dialogue_tree_branch_and_messages() {
    let mut dialogue = Dialogue::new("Test Dialogue");
    dialogue.add_tree(Tree::new("Initial Plan"));

    let tree = dialogue.trees_mut().last_mut().expect("Tree should exist");
    tree.add_branch(Branch::new("main"));

    let branch = tree.branches_mut().last_mut().expect("Branch should exist");
    branch.add_message(Message::new("Hello, LLM!", Role::User));
    branch.add_message(Message::new("Hi, user!", Role::Assistant));

    assert_eq!(branch.messages().len(), 2);
}

#[test]
fn can_fork_branch_from_message() {
    let mut branch = Branch::new("main");
    branch.add_message(Message::new("First", Role::User));
    branch.add_message(Message::new("Second", Role::Assistant));

    let fork_point_id = branch.messages()[1].id();

    let fork = branch
        .fork_from(fork_point_id, "alt")
        .expect("Fork should succeed");

    assert_eq!(fork.messages()[0].content(), "First");
    assert_eq!(fork.messages()[1].content(), "Second");
    assert_eq!(fork.name(), "alt");
}

#[test]
fn hidden_messages_are_excluded_from_context() {
    let mut branch = Branch::new("main");
    branch.add_message(Message::new("Visible", Role::User));
    branch.add_message(Message::new("Hidden", Role::User));

    branch.messages_mut()[1].hide();

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
