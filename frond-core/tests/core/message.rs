use frond_core::{Message, Role};

#[test]
fn creates_message_with_content_and_role() {
    let message = Message::new("Hello world", Role::User);

    assert_eq!(message.content(), "Hello world");
    assert_eq!(message.role(), &Role::User);
    assert!(!message.is_hidden());
}

#[test]
fn creates_assistant_message() {
    let message = Message::new("Hi there!", Role::Assistant);

    assert_eq!(message.content(), "Hi there!");
    assert_eq!(message.role(), &Role::Assistant);
    assert!(!message.is_hidden());
}

#[test]
fn message_has_unique_id() {
    let msg1 = Message::new("First", Role::User);
    let msg2 = Message::new("Second", Role::User);

    assert_ne!(msg1.id(), msg2.id());
}

#[test]
fn message_starts_visible() {
    let message = Message::new("Test message", Role::User);
    assert!(!message.is_hidden());
}

#[test]
fn empty_message_content() {
    let message = Message::new("", Role::User);
    assert_eq!(message.content(), "");
}

#[test]
fn message_with_multiline_content() {
    let content = "Line 1\nLine 2\nLine 3";
    let message = Message::new(content, Role::User);
    assert_eq!(message.content(), content);
}

#[test]
fn message_with_unicode_content() {
    let content = "Hello 世界 🌍";
    let message = Message::new(content, Role::User);
    assert_eq!(message.content(), content);
}

#[test]
fn message_accepts_string_and_str() {
    let owned_string = String::from("Owned string");
    let message1 = Message::new(owned_string, Role::User);
    let message2 = Message::new("String literal", Role::Assistant);

    assert_eq!(message1.content(), "Owned string");
    assert_eq!(message2.content(), "String literal");
}

#[test]
fn role_equality() {
    assert_eq!(Role::User, Role::User);
    assert_eq!(Role::Assistant, Role::Assistant);
    assert_ne!(Role::User, Role::Assistant);
}

#[test]
fn message_clone_preserves_all_fields() {
    let original = Message::new("Test content", Role::Assistant);
    let cloned = original.clone();

    assert_eq!(original.id(), cloned.id());
    assert_eq!(original.content(), cloned.content());
    assert_eq!(original.role(), cloned.role());
    assert_eq!(original.is_hidden(), cloned.is_hidden());
}

#[test]
fn message_debug_format() {
    let message = Message::new("Debug test", Role::User);
    let debug_str = format!("{:?}", message);

    assert!(debug_str.contains("Debug test"));
    assert!(debug_str.contains("User"));
}
