use frond_core::{Branch, Dialogue, Message, Role, Tree};

pub fn create_test_dialogue() -> Dialogue {
    // Create some test messages
    let messages = vec![
        Message::new("Hello, how can I help you today?", Role::Assistant),
        Message::new("I need help with Rust programming", Role::User),
        Message::new(
            "Of course! What specific aspect of Rust would you like to learn about?",
            Role::Assistant,
        ),
        Message::new("I'm struggling with ownership and borrowing", Role::User),
        Message::new(
            "Ownership is one of Rust's most important concepts. Let me explain...",
            Role::Assistant,
        ),
    ];

    let mut main_branch = Branch::new("main");
    for msg in messages {
        main_branch.add_message(msg);
    }

    // Create alternative branch
    let alt_messages = vec![
        Message::new("Hello, how can I help you today?", Role::Assistant),
        Message::new("I need help with Rust programming", Role::User),
        Message::new(
            "Great! Rust is a fantastic language. What would you like to build?",
            Role::Assistant,
        ),
        Message::new("I want to build a CLI application", Role::User),
    ];

    let mut alt_branch = Branch::new("cli-focused");
    for msg in alt_messages {
        alt_branch.add_message(msg);
    }

    let mut tree = Tree::new("Rust Help Session");
    tree.add_branch(main_branch);
    tree.add_branch(alt_branch);

    // Create second tree
    let mut second_tree = Tree::new("Python Questions");
    let mut python_branch = Branch::new("main");
    python_branch.add_message(Message::new("How do I learn Python?", Role::User));
    python_branch.add_message(Message::new(
        "Start with the basics: variables, functions, and control flow.",
        Role::Assistant,
    ));
    second_tree.add_branch(python_branch);

    let mut dialogue = Dialogue::new("My Conversations");
    dialogue.add_tree(tree);
    dialogue.add_tree(second_tree);

    dialogue
}
