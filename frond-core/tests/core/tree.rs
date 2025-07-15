use frond_core::{Branch, Message, Role, Tree};

// Basic Tree Creation Tests
#[test]
fn new_tree_has_name_and_empty_branches() {
    let tree = Tree::new("main");

    assert_eq!(tree.name(), "main");
    assert!(tree.branches().is_empty());
    assert_eq!(tree.description(), None);
}

#[test]
fn new_tree_has_unique_id() {
    let tree1 = Tree::new("main");
    let tree2 = Tree::new("main");

    assert_ne!(tree1.id(), tree2.id());
}

#[test]
fn new_tree_accepts_different_name_types() {
    let tree1 = Tree::new("string_literal");
    let tree2 = Tree::new(String::from("owned_string"));

    assert_eq!(tree1.name(), "string_literal");
    assert_eq!(tree2.name(), "owned_string");
}

// Tree from Branch Tests
#[test]
fn from_branch_creates_tree_with_single_branch() {
    let branch = Branch::new("feature");
    let branch_id = branch.id();
    let tree = Tree::from_branch(branch);

    assert_eq!(tree.name(), "feature");
    assert_eq!(tree.branches().len(), 1);
    assert_eq!(tree.branches()[0].id(), branch_id);
}

#[test]
fn from_branch_preserves_branch_messages() {
    let msg1 = Message::new("Hello", Role::User);
    let msg2 = Message::new("Hi", Role::Assistant);
    let messages = vec![msg1, msg2];
    let branch = Branch::from_messages("feature", messages);
    let tree = Tree::from_branch(branch);

    assert_eq!(tree.branches().len(), 1);
    assert_eq!(tree.branches()[0].messages().len(), 2);
    assert_eq!(tree.branches()[0].messages()[0].content(), "Hello");
    assert_eq!(tree.branches()[0].messages()[1].content(), "Hi");
}

// Tree from Branches Tests
#[test]
fn from_branches_creates_tree_with_multiple_branches() {
    let branch1 = Branch::new("main");
    let branch2 = Branch::new("feature");
    let branch1_id = branch1.id();
    let branch2_id = branch2.id();
    let branches = vec![branch1, branch2];

    let tree = Tree::from_branches("project", branches);

    assert_eq!(tree.name(), "project");
    assert_eq!(tree.branches().len(), 2);
    assert!(tree.get_branch_by_id(branch1_id).is_some());
    assert!(tree.get_branch_by_id(branch2_id).is_some());
}

#[test]
fn from_branches_handles_empty_vector() {
    let branches = vec![];
    let tree = Tree::from_branches("empty", branches);

    assert_eq!(tree.name(), "empty");
    assert!(tree.branches().is_empty());
}

// Branch Lookup Tests
#[test]
fn get_branch_by_id_finds_existing_branch() {
    let branch = Branch::new("findable");
    let branch_id = branch.id();
    let tree = Tree::from_branch(branch);

    let found = tree.get_branch_by_id(branch_id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "findable");
}

#[test]
fn get_branch_by_id_returns_none_for_nonexistent_id() {
    let tree = Tree::new("main");
    let fake_id = uuid::Uuid::new_v4();

    let found = tree.get_branch_by_id(fake_id);
    assert!(found.is_none());
}

#[test]
fn get_branch_index_by_id_returns_correct_position() {
    let branch1 = Branch::new("first");
    let branch2 = Branch::new("second");
    let branch1_id = branch1.id();
    let branch2_id = branch2.id();
    let branches = vec![branch1, branch2];

    let tree = Tree::from_branches("test", branches);

    assert_eq!(tree.get_branch_index_by_id(branch1_id), Some(0));
    assert_eq!(tree.get_branch_index_by_id(branch2_id), Some(1));
}

#[test]
fn get_branch_index_by_id_returns_none_for_nonexistent_id() {
    let tree = Tree::new("main");
    let fake_id = uuid::Uuid::new_v4();

    let index = tree.get_branch_index_by_id(fake_id);
    assert!(index.is_none());
}

// Branch Access Tests
#[test]
fn branches_provides_read_access_to_all_branches() {
    let branch1 = Branch::new("main");
    let branch2 = Branch::new("feature");
    let branches = vec![branch1, branch2];

    let tree = Tree::from_branches("project", branches);

    assert_eq!(tree.branches().len(), 2);
    assert_eq!(tree.branches()[0].name(), "main");
    assert_eq!(tree.branches()[1].name(), "feature");
}

#[test]
fn branches_is_empty_for_new_tree() {
    let tree = Tree::new("empty");

    assert!(tree.branches().is_empty());
    assert_eq!(tree.branches().len(), 0);
}

// Clone and Debug Tests
#[test]
fn clone_preserves_all_tree_data() {
    let msg = Message::new("Test", Role::User);
    let messages = vec![msg];
    let branch = Branch::from_messages("feature", messages);
    let tree = Tree::from_branch(branch);

    let cloned = tree.clone();

    assert_eq!(tree.id(), cloned.id());
    assert_eq!(tree.name(), cloned.name());
    assert_eq!(tree.description(), cloned.description());
    assert_eq!(tree.branches().len(), cloned.branches().len());
    assert_eq!(tree.branches()[0].id(), cloned.branches()[0].id());
}

#[test]
fn debug_format_contains_tree_name() {
    let tree = Tree::new("debug_test");
    let debug_str = format!("{:?}", tree);

    assert!(debug_str.contains("debug_test"));
}

// Complex Structure Tests
#[test]
fn complex_tree_maintains_branch_structure() {
    let msg1 = Message::new("Hello", Role::User);
    let msg2 = Message::new("Hi", Role::Assistant);
    let main_messages = vec![msg1, msg2];
    let main_branch = Branch::from_messages("main", main_messages);

    let feature_msg = Message::new("Feature work", Role::User);
    let feature_messages = vec![feature_msg];
    let feature_branch = Branch::from_messages("feature", feature_messages);

    let main_id = main_branch.id();
    let feature_id = feature_branch.id();
    let branches = vec![main_branch, feature_branch];

    let tree = Tree::from_branches("complex", branches);

    assert_eq!(tree.branches().len(), 2);

    let main_branch = tree.get_branch_by_id(main_id).unwrap();
    assert_eq!(main_branch.messages().len(), 2);
    assert_eq!(main_branch.messages()[0].content(), "Hello");
    assert_eq!(main_branch.messages()[1].content(), "Hi");

    let feature_branch = tree.get_branch_by_id(feature_id).unwrap();
    assert_eq!(feature_branch.messages().len(), 1);
    assert_eq!(feature_branch.messages()[0].content(), "Feature work");
}
