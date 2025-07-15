use frond_core::{Action, Dialogue, Message, Role};

#[test]
fn creates_message_with_content_and_role() {
    let msg = Message::new("Hello", Role::User);
    assert_eq!(msg.content(), "Hello");
    assert_eq!(*msg.role(), Role::User);
    assert!(!msg.is_hidden());
}

#[test]
fn hides_and_shows_message() {
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
            message_content: "Test".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    let branch = tree
        .branches()
        .iter()
        .find(|b| b.id() == branch_id)
        .unwrap();
    let message_id = branch.messages()[0].id();

    assert!(!branch.messages()[0].is_hidden());

    dialogue
        .apply_action(Action::HideMessage { message_id })
        .unwrap();
    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    let branch = tree
        .branches()
        .iter()
        .find(|b| b.id() == branch_id)
        .unwrap();
    assert!(branch.messages()[0].is_hidden());

    dialogue
        .apply_action(Action::ShowMessage { message_id })
        .unwrap();
    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    let branch = tree
        .branches()
        .iter()
        .find(|b| b.id() == branch_id)
        .unwrap();
    assert!(!branch.messages()[0].is_hidden());
}

#[test]
fn edits_message_content() {
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
            message_content: "Old".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    let branch = tree
        .branches()
        .iter()
        .find(|b| b.id() == branch_id)
        .unwrap();
    let message_id = branch.messages()[0].id();

    dialogue
        .apply_action(Action::EditMessage {
            message_id,
            old_content: "Old".to_string(),
            new_content: "New".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    let branch = tree
        .branches()
        .iter()
        .find(|b| b.id() == branch_id)
        .unwrap();
    assert_eq!(branch.messages()[0].content(), "New");
}

#[test]
fn switches_role() {
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
            message_content: "Hi".to_string(),
        })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    let branch = tree
        .branches()
        .iter()
        .find(|b| b.id() == branch_id)
        .unwrap();
    let message_id = branch.messages()[0].id();

    dialogue
        .apply_action(Action::ToggleMessageRole { message_id })
        .unwrap();

    let tree = dialogue.trees().iter().find(|t| t.id() == tree_id).unwrap();
    let branch = tree
        .branches()
        .iter()
        .find(|b| b.id() == branch_id)
        .unwrap();
    assert_eq!(*branch.messages()[0].role(), Role::Assistant);
}

#[test]
fn message_id_is_unique() {
    let msg1 = Message::new("A", Role::User);
    let msg2 = Message::new("B", Role::User);
    assert_ne!(msg1.id(), msg2.id());
}
