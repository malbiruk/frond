use frond_core::{Branch, Message, Role, Tree, TreeError};
use uuid::Uuid;

crate::test_core_entity!(Tree, branch, branches, Branch, Branch::new("test"));

#[test]
fn creates_tree_from_branch() {
    let branch = Branch::new("main");
    let tree = Tree::from_branch(branch.clone());
    assert_eq!(tree.name(), "main");
    assert_eq!(tree.branches().len(), 1);
    assert_eq!(tree.branches()[0].id(), branch.id());
}

#[test]
fn fork_branch_from_message_success() {
    let mut tree = Tree::new("My Tree");
    let mut branch = Branch::new("main");
    let msg1 = Message::new("First", Role::User);
    let msg2 = Message::new("Second", Role::Assistant);
    branch.add_message(msg1.clone());
    branch.add_message(msg2.clone());
    let branch_id = branch.id();
    let fork_point_id = msg2.id();

    tree.add_branch(branch);

    let forked = tree
        .fork_branch_from_message(branch_id, fork_point_id, "forked")
        .expect("Should fork branch");

    assert_eq!(forked.messages().len(), 2);
    assert_eq!(forked.messages()[0].content(), "First");
    assert_eq!(forked.messages()[1].content(), "Second");
    assert_eq!(forked.name(), "forked");
}

#[test]
fn fork_branch_from_message_branch_not_found() {
    let mut tree = Tree::new("My Tree");
    let fake_branch_id = Uuid::new_v4();
    let fake_message_id = Uuid::new_v4();

    let result = tree.fork_branch_from_message(fake_branch_id, fake_message_id, "forked");
    assert!(matches!(result, Err(TreeError::BranchNotFound(_))));
}

#[test]
fn fork_branch_from_message_message_not_found() {
    let mut tree = Tree::new("My Tree");
    let branch = Branch::new("main");
    let branch_id = branch.id();
    tree.add_branch(branch);

    let fake_message_id = Uuid::new_v4();
    let result = tree.fork_branch_from_message(branch_id, fake_message_id, "forked");
    assert!(matches!(result, Err(TreeError::Branch(_))));
}
