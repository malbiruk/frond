use frond_core::{Branch, Dialogue, Message, Role, Tree};
use uuid::Uuid;

#[test]
fn gets_branch_by_id() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let branch = Branch::new("test_branch");
    let branch_id = branch.id();
    dialogue.trees_mut()[0].add_branch(branch);

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
fn gets_branch_by_id_mut() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let branch = Branch::new("test_branch");
    let branch_id = branch.id();
    dialogue.trees_mut()[0].add_branch(branch);

    let found = dialogue.get_branch_by_id_mut(branch_id);
    assert!(found.is_some());
    found.unwrap().rename("renamed_branch");

    assert_eq!(
        dialogue.get_branch_by_id(branch_id).unwrap().name(),
        "renamed_branch"
    );
}

#[test]
fn get_branch_by_id_mut_returns_none_for_nonexistent() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let not_found = dialogue.get_branch_by_id_mut(Uuid::new_v4());
    assert!(not_found.is_none());
}

#[test]
fn gets_message_by_id() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let mut branch = Branch::new("test_branch");
    let message = Message::new("Hello world", Role::User);
    let message_id = message.id();
    branch.add_message(message);
    dialogue.trees_mut()[0].add_branch(branch);

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
fn gets_message_by_id_mut() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let mut branch = Branch::new("test_branch");
    let message = Message::new("Hello world", Role::User);
    let message_id = message.id();
    branch.add_message(message);
    dialogue.trees_mut()[0].add_branch(branch);

    let found = dialogue.get_message_by_id_mut(message_id);
    assert!(found.is_some());
    found.unwrap().edit_content("Updated content");

    assert_eq!(
        dialogue.get_message_by_id(message_id).unwrap().content(),
        "Updated content"
    );
}

#[test]
fn get_message_by_id_mut_returns_none_for_nonexistent() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let not_found = dialogue.get_message_by_id_mut(Uuid::new_v4());
    assert!(not_found.is_none());
}

#[test]
fn get_message_by_id_works_across_trees_and_branches() {
    let tree = Tree::new("main");
    let mut dialogue = Dialogue::from_tree(tree);

    let mut branch = Branch::new("test_branch");
    let msg1 = Message::new("First message", Role::User);
    let msg2 = Message::new("Second message", Role::Assistant);
    let msg3 = Message::new("Third message", Role::User);

    let msg1_id = msg1.id();
    let msg2_id = msg2.id();
    let msg3_id = msg3.id();

    branch.add_message(msg1.clone());
    branch.add_message(msg2.clone());
    branch.add_message(msg3.clone());

    dialogue.trees_mut()[0].add_branch(branch);

    // Test finding messages by ID
    let found_msg1 = dialogue.get_message_by_id(msg1_id);
    assert!(found_msg1.is_some());
    assert_eq!(found_msg1.unwrap().content(), msg1.content());

    let found_msg2 = dialogue.get_message_by_id(msg2_id);
    assert!(found_msg2.is_some());
    assert_eq!(found_msg2.unwrap().content(), msg2.content());

    let found_msg3 = dialogue.get_message_by_id(msg3_id);
    assert!(found_msg3.is_some());
    assert_eq!(found_msg3.unwrap().content(), msg3.content());

    // Test with nonexistent message
    let nonexistent_id = Uuid::new_v4();
    let result = dialogue.get_message_by_id(nonexistent_id);
    assert!(result.is_none());
}

#[test]
fn helper_methods_work_across_multiple_trees() {
    let tree1 = Tree::new("tree1");
    let tree2 = Tree::new("tree2");
    let mut dialogue = Dialogue::from_trees("dialogue", vec![tree1, tree2]);

    let mut branch1 = Branch::new("branch1");
    let mut branch2 = Branch::new("branch2");

    let message1 = Message::new("Message in tree1", Role::User);
    let message2 = Message::new("Message in tree2", Role::Assistant);
    let message1_id = message1.id();
    let message2_id = message2.id();

    let branch1_id = branch1.id();
    let branch2_id = branch2.id();

    branch1.add_message(message1);
    branch2.add_message(message2);

    dialogue.trees_mut()[0].add_branch(branch1);
    dialogue.trees_mut()[1].add_branch(branch2);

    // Verify messages can be found across different trees
    assert_eq!(
        dialogue.get_message_by_id(message1_id).unwrap().content(),
        "Message in tree1"
    );
    assert_eq!(
        dialogue.get_message_by_id(message2_id).unwrap().content(),
        "Message in tree2"
    );

    // Verify branches can be found across different trees
    assert_eq!(
        dialogue.get_branch_by_id(branch1_id).unwrap().name(),
        "branch1"
    );
    assert_eq!(
        dialogue.get_branch_by_id(branch2_id).unwrap().name(),
        "branch2"
    );
}
