#[macro_use]
mod macros;

use frond_core::{Branch, Message, Role};
use uuid::Uuid;

test_core_entity!(
    Branch,
    message,
    messages,
    Message,
    Message::new("test", Role::User)
);

#[test]
fn fork_from_valid_message() {
    let mut branch = Branch::new("main");
    let msg1 = Message::new("First", Role::User);
    let msg2 = Message::new("Second", Role::Assistant);
    branch.add_message(msg1.clone());
    branch.add_message(msg2.clone());

    let fork = branch
        .fork_from(msg2.id(), "alt")
        .expect("Fork should succeed");
    assert_eq!(fork.messages().len(), 2);
    assert_eq!(fork.messages()[0].content(), "First");
    assert_eq!(fork.messages()[1].content(), "Second");
    assert_eq!(fork.name(), "alt");
}

#[test]
fn fork_from_invalid_message_returns_error() {
    let branch = Branch::new("main");
    let invalid_id = Uuid::new_v4();
    let result = branch.fork_from(invalid_id, "alt");
    assert!(result.is_err());
}
