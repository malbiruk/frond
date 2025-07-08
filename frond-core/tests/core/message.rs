use frond_core::{Message, Role};

#[test]
fn creates_message_with_content_and_role() {
    let msg = Message::new("Hello", Role::User);
    assert_eq!(msg.content(), "Hello");
    assert_eq!(*msg.role(), Role::User);
    assert!(!msg.is_hidden());
}

#[test]
fn hides_and_shows_message() {
    let mut msg = Message::new("Test", Role::Assistant);
    assert!(!msg.is_hidden());
    msg.hide();
    assert!(msg.is_hidden());
    msg.show();
    assert!(!msg.is_hidden());
}

#[test]
fn edits_message_content() {
    let mut msg = Message::new("Old", Role::User);
    msg.edit_content("New");
    assert_eq!(msg.content(), "New");
}

#[test]
fn switches_role() {
    let mut msg = Message::new("Hi", Role::User);
    msg.switch_role(Role::Assistant);
    assert_eq!(*msg.role(), Role::Assistant);
}

#[test]
fn message_id_is_unique() {
    let msg1 = Message::new("A", Role::User);
    let msg2 = Message::new("B", Role::User);
    assert_ne!(msg1.id(), msg2.id());
}
