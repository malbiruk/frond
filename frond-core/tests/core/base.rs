use frond_core::{Action, BranchAction, Dialogue, DialogueAction, MessageAction, TreeAction};

#[test]
fn can_create_dialogue_tree_branch_and_messages() {
    let mut dialogue = Dialogue::new("Test Dialogue");

    dialogue
        .apply_action(Action::Dialogue(DialogueAction::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "Initial Plan".to_string(),
        }))
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    dialogue
        .apply_action(Action::Tree(TreeAction::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "main".to_string(),
        }))
        .unwrap();

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::Branch(BranchAction::AppendMessage {
            branch_id,
            message_content: "Hello, LLM!".to_string(),
        }))
        .unwrap();

    dialogue
        .apply_action(Action::Branch(BranchAction::AppendMessage {
            branch_id,
            message_content: "Hi, user!".to_string(),
        }))
        .unwrap();

    let created_branch = dialogue.get_branch_by_id(branch_id).unwrap();
    assert_eq!(created_branch.messages().len(), 2);
    assert_eq!(created_branch.messages()[0].content(), "Hello, LLM!");
    assert_eq!(created_branch.messages()[1].content(), "Hi, user!");
}

#[test]
fn can_fork_branch_from_message() {
    let mut dialogue = Dialogue::new("Test Dialogue");

    dialogue
        .apply_action(Action::Dialogue(DialogueAction::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "Initial Plan".to_string(),
        }))
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    dialogue
        .apply_action(Action::Tree(TreeAction::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "main".to_string(),
        }))
        .unwrap();

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::Branch(BranchAction::AppendMessage {
            branch_id,
            message_content: "First".to_string(),
        }))
        .unwrap();

    dialogue
        .apply_action(Action::Branch(BranchAction::AppendMessage {
            branch_id,
            message_content: "Second".to_string(),
        }))
        .unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let fork_point_id = branch.messages()[1].id();

    dialogue
        .apply_action(Action::Tree(TreeAction::ForkBranch {
            tree_id,
            branch_id,
            from_message_id: fork_point_id,
            new_branch_name: "alt".to_string(),
        }))
        .unwrap();

    let tree = dialogue.get_tree_by_id(tree_id).unwrap();
    let fork = tree.branches().iter().find(|b| b.name() == "alt").unwrap();
    assert_eq!(fork.messages().len(), 2);
    assert_eq!(fork.messages()[0].content(), "First");
    assert_eq!(fork.messages()[1].content(), "Second");
    assert_eq!(fork.name(), "alt");
}

#[test]
fn hidden_messages_are_excluded_from_context() {
    let mut dialogue = Dialogue::new("Test Dialogue");

    dialogue
        .apply_action(Action::Dialogue(DialogueAction::AddTree {
            tree_id: uuid::Uuid::new_v4(),
            tree_name: "Initial Plan".to_string(),
        }))
        .unwrap();

    let tree_id = dialogue.trees()[0].id();

    dialogue
        .apply_action(Action::Tree(TreeAction::AddBranch {
            tree_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "main".to_string(),
        }))
        .unwrap();

    let branch_id = dialogue.trees()[0].branches()[0].id();

    dialogue
        .apply_action(Action::Branch(BranchAction::AppendMessage {
            branch_id,
            message_content: "Visible".to_string(),
        }))
        .unwrap();

    dialogue
        .apply_action(Action::Branch(BranchAction::AppendMessage {
            branch_id,
            message_content: "Hidden".to_string(),
        }))
        .unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let hidden_msg_id = branch.messages()[1].id();

    dialogue
        .apply_action(Action::Message(MessageAction::HideMessage {
            message_id: hidden_msg_id,
        }))
        .unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
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
//     assert_eq!(branch.token_count(), count_tokens("A"));
// }
