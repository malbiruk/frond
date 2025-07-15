use frond_core::{Branch, Dialogue, Message, Role, Tree};
use uuid::Uuid;

// Basic Dialogue Creation Tests
#[test]
fn new_dialogue_has_name_and_empty_trees() {
    let dialogue = Dialogue::new("test_dialogue");

    assert_eq!(dialogue.name(), "test_dialogue");
    assert!(dialogue.trees().is_empty());
    assert_eq!(dialogue.description(), None);
}

#[test]
fn new_dialogue_has_unique_id() {
    let dialogue1 = Dialogue::new("test");
    let dialogue2 = Dialogue::new("test");

    assert_ne!(dialogue1.id(), dialogue2.id());
}

#[test]
fn new_dialogue_accepts_different_name_types() {
    let dialogue1 = Dialogue::new("string_literal");
    let dialogue2 = Dialogue::new(String::from("owned_string"));

    assert_eq!(dialogue1.name(), "string_literal");
    assert_eq!(dialogue2.name(), "owned_string");
}

// Dialogue from Tree Tests
#[test]
fn from_tree_creates_dialogue_with_single_tree() {
    let tree = Tree::new("main_tree");
    let tree_id = tree.id();
    let dialogue = Dialogue::from_tree(tree);

    assert_eq!(dialogue.name(), "main_tree");
    assert_eq!(dialogue.trees().len(), 1);
    assert_eq!(dialogue.trees()[0].id(), tree_id);
}

#[test]
fn from_tree_inherits_tree_name() {
    let tree = Tree::new("inherited_name");
    let dialogue = Dialogue::from_tree(tree);

    assert_eq!(dialogue.name(), "inherited_name");
}

#[test]
fn from_tree_preserves_tree_structure() {
    let msg1 = Message::new("Hello", Role::User);
    let msg2 = Message::new("Hi", Role::Assistant);
    let messages = vec![msg1, msg2];
    let branch = Branch::from_messages("main_branch", messages);
    let tree = Tree::from_branch(branch);
    let dialogue = Dialogue::from_tree(tree);

    assert_eq!(dialogue.trees().len(), 1);
    assert_eq!(dialogue.trees()[0].branches().len(), 1);
    assert_eq!(dialogue.trees()[0].branches()[0].messages().len(), 2);
}

// Dialogue from Trees Tests
#[test]
fn from_trees_creates_dialogue_with_multiple_trees() {
    let tree1 = Tree::new("tree1");
    let tree2 = Tree::new("tree2");
    let tree1_id = tree1.id();
    let tree2_id = tree2.id();
    let trees = vec![tree1, tree2];

    let dialogue = Dialogue::from_trees("multi_tree_dialogue", trees);

    assert_eq!(dialogue.name(), "multi_tree_dialogue");
    assert_eq!(dialogue.trees().len(), 2);
    assert!(dialogue.get_tree_by_id(tree1_id).is_some());
    assert!(dialogue.get_tree_by_id(tree2_id).is_some());
}

#[test]
fn from_trees_handles_empty_vector() {
    let trees = vec![];
    let dialogue = Dialogue::from_trees("empty_dialogue", trees);

    assert_eq!(dialogue.name(), "empty_dialogue");
    assert!(dialogue.trees().is_empty());
}

// Tree Lookup Tests
#[test]
fn get_tree_by_id_finds_existing_tree() {
    let tree = Tree::new("findable_tree");
    let tree_id = tree.id();
    let dialogue = Dialogue::from_tree(tree);

    let found = dialogue.get_tree_by_id(tree_id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "findable_tree");
}

#[test]
fn get_tree_by_id_returns_none_for_nonexistent_id() {
    let dialogue = Dialogue::new("test");
    let fake_id = Uuid::new_v4();

    let found = dialogue.get_tree_by_id(fake_id);
    assert!(found.is_none());
}

#[test]
fn get_tree_index_by_id_returns_correct_position() {
    let tree1 = Tree::new("first");
    let tree2 = Tree::new("second");
    let tree1_id = tree1.id();
    let tree2_id = tree2.id();
    let trees = vec![tree1, tree2];

    let dialogue = Dialogue::from_trees("test", trees);

    assert_eq!(dialogue.get_tree_index_by_id(tree1_id), Some(0));
    assert_eq!(dialogue.get_tree_index_by_id(tree2_id), Some(1));
}

#[test]
fn get_tree_index_by_id_returns_none_for_nonexistent_id() {
    let dialogue = Dialogue::new("test");
    let fake_id = Uuid::new_v4();

    let index = dialogue.get_tree_index_by_id(fake_id);
    assert!(index.is_none());
}

// Tree Access Tests
#[test]
fn trees_provides_read_access_to_all_trees() {
    let tree1 = Tree::new("tree1");
    let tree2 = Tree::new("tree2");
    let trees = vec![tree1, tree2];

    let dialogue = Dialogue::from_trees("multi_tree", trees);

    assert_eq!(dialogue.trees().len(), 2);
    assert_eq!(dialogue.trees()[0].name(), "tree1");
    assert_eq!(dialogue.trees()[1].name(), "tree2");
}

#[test]
fn trees_is_empty_for_new_dialogue() {
    let dialogue = Dialogue::new("empty");

    assert!(dialogue.trees().is_empty());
    assert_eq!(dialogue.trees().len(), 0);
}

// Branch Helper Tests
#[test]
fn get_branch_by_id_finds_branch_across_trees() {
    let msg = Message::new("Test message", Role::User);
    let branch = Branch::from_messages("test_branch", vec![msg]);
    let branch_id = branch.id();
    let tree = Tree::from_branch(branch);
    let dialogue = Dialogue::from_tree(tree);

    let found = dialogue.get_branch_by_id(branch_id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "test_branch");
    assert_eq!(found.unwrap().messages().len(), 1);
}

#[test]
fn get_branch_by_id_returns_none_for_nonexistent_id() {
    let dialogue = Dialogue::new("test");
    let fake_id = Uuid::new_v4();

    let found = dialogue.get_branch_by_id(fake_id);
    assert!(found.is_none());
}

#[test]
fn get_branch_by_id_searches_across_multiple_trees() {
    let branch1 = Branch::new("branch1");
    let branch2 = Branch::new("branch2");
    let branch1_id = branch1.id();
    let branch2_id = branch2.id();

    let tree1 = Tree::from_branch(branch1);
    let tree2 = Tree::from_branch(branch2);
    let trees = vec![tree1, tree2];

    let dialogue = Dialogue::from_trees("multi_tree", trees);

    let found1 = dialogue.get_branch_by_id(branch1_id);
    let found2 = dialogue.get_branch_by_id(branch2_id);

    assert!(found1.is_some());
    assert!(found2.is_some());
    assert_eq!(found1.unwrap().name(), "branch1");
    assert_eq!(found2.unwrap().name(), "branch2");
}

// Message Helper Tests
#[test]
fn get_message_by_id_finds_message_across_trees_and_branches() {
    let msg1 = Message::new("First message", Role::User);
    let msg2 = Message::new("Second message", Role::Assistant);
    let msg1_id = msg1.id();
    let msg2_id = msg2.id();

    let branch = Branch::from_messages("test_branch", vec![msg1, msg2]);
    let tree = Tree::from_branch(branch);
    let dialogue = Dialogue::from_tree(tree);

    let found1 = dialogue.get_message_by_id(msg1_id);
    let found2 = dialogue.get_message_by_id(msg2_id);

    assert!(found1.is_some());
    assert!(found2.is_some());
    assert_eq!(found1.unwrap().content(), "First message");
    assert_eq!(found2.unwrap().content(), "Second message");
}

#[test]
fn get_message_by_id_returns_none_for_nonexistent_id() {
    let dialogue = Dialogue::new("test");
    let fake_id = Uuid::new_v4();

    let found = dialogue.get_message_by_id(fake_id);
    assert!(found.is_none());
}

#[test]
fn get_message_by_id_searches_across_multiple_trees_and_branches() {
    // Tree 1 with 2 branches
    let msg1 = Message::new("Tree1 Branch1 Message", Role::User);
    let msg2 = Message::new("Tree1 Branch2 Message", Role::Assistant);
    let msg1_id = msg1.id();
    let msg2_id = msg2.id();

    let branch1 = Branch::from_messages("tree1_branch1", vec![msg1]);
    let branch2 = Branch::from_messages("tree1_branch2", vec![msg2]);
    let tree1 = Tree::from_branches("tree1", vec![branch1, branch2]);

    // Tree 2 with 1 branch
    let msg3 = Message::new("Tree2 Branch1 Message", Role::User);
    let msg3_id = msg3.id();
    let branch3 = Branch::from_messages("tree2_branch1", vec![msg3]);
    let tree2 = Tree::from_branch(branch3);

    let dialogue = Dialogue::from_trees("complex_dialogue", vec![tree1, tree2]);

    let found1 = dialogue.get_message_by_id(msg1_id);
    let found2 = dialogue.get_message_by_id(msg2_id);
    let found3 = dialogue.get_message_by_id(msg3_id);

    assert!(found1.is_some());
    assert!(found2.is_some());
    assert!(found3.is_some());
    assert_eq!(found1.unwrap().content(), "Tree1 Branch1 Message");
    assert_eq!(found2.unwrap().content(), "Tree1 Branch2 Message");
    assert_eq!(found3.unwrap().content(), "Tree2 Branch1 Message");
}

// Archive Status Tests
#[test]
fn new_dialogue_is_not_archived() {
    let dialogue = Dialogue::new("test");
    assert!(!dialogue.is_archived());
}

#[test]
fn dialogue_from_tree_is_not_archived() {
    let tree = Tree::new("test");
    let dialogue = Dialogue::from_tree(tree);
    assert!(!dialogue.is_archived());
}

// Trash Status Tests
#[test]
fn new_dialogue_is_not_trashed() {
    let dialogue = Dialogue::new("test");
    assert!(!dialogue.is_trashed());
}

#[test]
fn dialogue_from_tree_is_not_trashed() {
    let tree = Tree::new("test");
    let dialogue = Dialogue::from_tree(tree);
    assert!(!dialogue.is_trashed());
}

// Tag Tests
#[test]
fn new_dialogue_has_empty_tags() {
    let dialogue = Dialogue::new("test");
    assert!(dialogue.tags().is_empty());
}

#[test]
fn dialogue_from_tree_has_empty_tags() {
    let tree = Tree::new("test");
    let dialogue = Dialogue::from_tree(tree);
    assert!(dialogue.tags().is_empty());
}

// Clone Tests
#[test]
fn clone_preserves_all_dialogue_data() {
    let msg = Message::new("Test message", Role::User);
    let branch = Branch::from_messages("test_branch", vec![msg]);
    let tree = Tree::from_branch(branch);
    let dialogue = Dialogue::from_tree(tree);

    let cloned = dialogue.clone();

    assert_eq!(dialogue.id(), cloned.id());
    assert_eq!(dialogue.name(), cloned.name());
    assert_eq!(dialogue.description(), cloned.description());
    assert_eq!(dialogue.trees().len(), cloned.trees().len());
    assert_eq!(dialogue.trees()[0].id(), cloned.trees()[0].id());
    assert_eq!(dialogue.is_archived(), cloned.is_archived());
    assert_eq!(dialogue.is_trashed(), cloned.is_trashed());
    assert_eq!(dialogue.tags(), cloned.tags());
}

// Debug Tests
#[test]
fn debug_format_contains_dialogue_name() {
    let dialogue = Dialogue::new("debug_test");
    let debug_str = format!("{:?}", dialogue);

    assert!(debug_str.contains("debug_test"));
}

// Complex Structure Tests
#[test]
fn complex_dialogue_maintains_hierarchical_structure() {
    // Create a complex structure: 2 trees, each with 2 branches, each with 2 messages
    let msg1 = Message::new("T1B1M1", Role::User);
    let msg2 = Message::new("T1B1M2", Role::Assistant);
    let msg3 = Message::new("T1B2M1", Role::User);
    let msg4 = Message::new("T1B2M2", Role::Assistant);
    let msg1_id = msg1.id();
    let msg4_id = msg4.id();

    let branch1 = Branch::from_messages("tree1_branch1", vec![msg1, msg2]);
    let branch2 = Branch::from_messages("tree1_branch2", vec![msg3, msg4]);
    let tree1 = Tree::from_branches("tree1", vec![branch1, branch2]);
    let tree1_id = tree1.id();

    let msg5 = Message::new("T2B1M1", Role::User);
    let msg6 = Message::new("T2B1M2", Role::Assistant);
    let msg7 = Message::new("T2B2M1", Role::User);
    let msg8 = Message::new("T2B2M2", Role::Assistant);
    let msg7_id = msg7.id();

    let branch3 = Branch::from_messages("tree2_branch1", vec![msg5, msg6]);
    let branch4 = Branch::from_messages("tree2_branch2", vec![msg7, msg8]);
    let tree2 = Tree::from_branches("tree2", vec![branch3, branch4]);
    let tree2_id = tree2.id();

    let dialogue = Dialogue::from_trees("complex_dialogue", vec![tree1, tree2]);

    // Verify structure
    assert_eq!(dialogue.trees().len(), 2);
    assert_eq!(dialogue.name(), "complex_dialogue");

    // Verify tree access
    let found_tree1 = dialogue.get_tree_by_id(tree1_id).unwrap();
    let found_tree2 = dialogue.get_tree_by_id(tree2_id).unwrap();
    assert_eq!(found_tree1.name(), "tree1");
    assert_eq!(found_tree2.name(), "tree2");
    assert_eq!(found_tree1.branches().len(), 2);
    assert_eq!(found_tree2.branches().len(), 2);

    // Verify cross-tree message lookup
    let found_msg1 = dialogue.get_message_by_id(msg1_id).unwrap();
    let found_msg4 = dialogue.get_message_by_id(msg4_id).unwrap();
    let found_msg7 = dialogue.get_message_by_id(msg7_id).unwrap();
    assert_eq!(found_msg1.content(), "T1B1M1");
    assert_eq!(found_msg4.content(), "T1B2M2");
    assert_eq!(found_msg7.content(), "T2B2M1");

    // Verify branch lookup works across trees
    let branch1_id = found_tree1.branches()[0].id();
    let branch4_id = found_tree2.branches()[1].id();
    let found_branch1 = dialogue.get_branch_by_id(branch1_id).unwrap();
    let found_branch4 = dialogue.get_branch_by_id(branch4_id).unwrap();
    assert_eq!(found_branch1.name(), "tree1_branch1");
    assert_eq!(found_branch4.name(), "tree2_branch2");
}
