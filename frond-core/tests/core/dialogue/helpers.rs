use frond_core::{Action, Dialogue, Tree};
use uuid::Uuid;

#[test]
fn gets_branch_by_id() {
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

    let found = dialogue.get_branch_by_id(branch_id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "test_branch");
}

#[test]
fn get_branch_by_id_returns_none_for_nonexistent() {
    let tree = Tree::new("main");
    let dialogue = Dialogue::from_tree(tree);

    let not_found = dialogue.get_branch_by_id(Uuid::new_v4());
    assert!(not_found.is_none());
}

#[test]
fn gets_message_by_id() {
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
    let message_id = branch.messages()[0].id();

    let found = dialogue.get_message_by_id(message_id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().content(), "Hello world");
}

#[test]
fn get_message_by_id_returns_none_for_nonexistent() {
    let tree = Tree::new("main");
    let dialogue = Dialogue::from_tree(tree);

    let not_found = dialogue.get_message_by_id(Uuid::new_v4());
    assert!(not_found.is_none());
}

#[test]
fn get_message_by_id_works_across_trees_and_branches() {
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
            message_content: "First message".to_string(),
        })
        .unwrap();
    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "Second message".to_string(),
        })
        .unwrap();
    dialogue
        .apply_action(Action::AppendMessage {
            branch_id,
            message_content: "Third message".to_string(),
        })
        .unwrap();

    let branch = dialogue.get_branch_by_id(branch_id).unwrap();
    let msg1_id = branch.messages()[0].id();
    let msg2_id = branch.messages()[1].id();
    let msg3_id = branch.messages()[2].id();

    let found_msg1 = dialogue.get_message_by_id(msg1_id);
    assert!(found_msg1.is_some());
    assert_eq!(found_msg1.unwrap().content(), "First message");

    let found_msg2 = dialogue.get_message_by_id(msg2_id);
    assert!(found_msg2.is_some());
    assert_eq!(found_msg2.unwrap().content(), "Second message");

    let found_msg3 = dialogue.get_message_by_id(msg3_id);
    assert!(found_msg3.is_some());
    assert_eq!(found_msg3.unwrap().content(), "Third message");

    let nonexistent_id = Uuid::new_v4();
    let result = dialogue.get_message_by_id(nonexistent_id);
    assert!(result.is_none());
}

#[test]
fn helper_methods_work_across_multiple_trees() {
    let tree1 = Tree::new("tree1");
    let tree2 = Tree::new("tree2");
    let tree1_id = tree1.id();
    let tree2_id = tree2.id();
    let mut dialogue = Dialogue::from_trees("dialogue", vec![tree1, tree2]);

    dialogue
        .apply_action(Action::AddBranch {
            tree_id: tree1_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "branch1".to_string(),
        })
        .unwrap();

    dialogue
        .apply_action(Action::AddBranch {
            tree_id: tree2_id,
            branch_id: uuid::Uuid::new_v4(),
            branch_name: "branch2".to_string(),
        })
        .unwrap();

    let branch1_id = dialogue.get_tree_by_id(tree1_id).unwrap().branches()[0].id();
    let branch2_id = dialogue.get_tree_by_id(tree2_id).unwrap().branches()[0].id();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id: branch1_id,
            message_content: "Message in tree1".to_string(),
        })
        .unwrap();

    dialogue
        .apply_action(Action::AppendMessage {
            branch_id: branch2_id,
            message_content: "Message in tree2".to_string(),
        })
        .unwrap();

    let branch1 = dialogue.get_branch_by_id(branch1_id).unwrap();
    let branch2 = dialogue.get_branch_by_id(branch2_id).unwrap();
    let message1_id = branch1.messages()[0].id();
    let message2_id = branch2.messages()[0].id();

    assert_eq!(
        dialogue.get_message_by_id(message1_id).unwrap().content(),
        "Message in tree1"
    );
    assert_eq!(
        dialogue.get_message_by_id(message2_id).unwrap().content(),
        "Message in tree2"
    );

    assert_eq!(
        dialogue.get_branch_by_id(branch1_id).unwrap().name(),
        "branch1"
    );
    assert_eq!(
        dialogue.get_branch_by_id(branch2_id).unwrap().name(),
        "branch2"
    );
}
