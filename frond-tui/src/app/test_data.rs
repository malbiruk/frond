use frond_core::{Branch, Dialogue, Message, Role, Tree};

pub fn create_test_dialogue() -> Dialogue {
    // Create main branch with detailed Rust ownership discussion
    let messages = vec![
        Message::new("Hello! How can I help you today?", Role::Assistant),
        Message::new("I need help understanding Rust's ownership system. I keep getting borrow checker errors and I don't understand why.", Role::User),
        Message::new(
            "I'd be happy to help you understand Rust's ownership system! It's one of Rust's most unique features and can be tricky at first.\n\n## The Three Rules of Ownership\n\n1. **Each value has a single owner**\n2. **There can only be one owner at a time**\n3. **When the owner goes out of scope, the value is dropped**\n\nLet me show you a simple example:\n\n```rust\nfn main() {\n    let s1 = String::from(\"hello\");\n    let s2 = s1;  // s1 is moved to s2\n    \n    // println!(\"{}\", s1);  // This would error!\n    println!(\"{}\", s2);     // This works\n}\n```\n\nThe key insight is that when we assign `s1` to `s2`, we're **moving** ownership, not copying. After the move, `s1` is no longer valid.",
            Role::Assistant,
        ),
        Message::new("Okay, that makes sense. But what if I want to use the same String in multiple places without moving it?", Role::User),
        Message::new(
            "Great question! You have several options:\n\n## 1. Borrowing with References\n\nThe most common approach is to use **references** (`&`):\n\n```rust\nfn main() {\n    let s1 = String::from(\"hello\");\n    let len = calculate_length(&s1);  // Borrow s1\n    println!(\"Length of '{}' is {}\", s1, len);  // s1 still valid!\n}\n\nfn calculate_length(s: &String) -> usize {\n    s.len()  // s is a reference, doesn't own the data\n}\n```\n\n## 2. Clone for Independent Copies\n\nIf you need truly independent copies:\n\n```rust\nlet s1 = String::from(\"hello\");\nlet s2 = s1.clone();  // Deep copy\nprintln!(\"s1: {}, s2: {}\", s1, s2);  // Both valid!\n```\n\n## 3. Multiple Immutable References\n\nYou can have many immutable references at once:\n\n```rust\nlet s = String::from(\"hello\");\nlet r1 = &s;\nlet r2 = &s;\nlet r3 = &s;\nprintln!(\"{}, {}, {}\", r1, r2, r3);  // All work!\n```\n\nWhich approach would work best for your use case?",
            Role::Assistant,
        ),
        Message::new("I think borrowing makes sense. But I'm getting an error about mutable and immutable references. Here's my code:\n\n```rust\nlet mut s = String::from(\"hello\");\nlet r1 = &s;\nlet r2 = &mut s;\nprintln!(\"{} {}\", r1, r2);\n```", Role::User),
        Message::new(
            "Ah, you've hit the **borrowing rules**! This is a critical safety feature in Rust.\n\n## The Borrowing Rules\n\n1. You can have **either**:\n   - Any number of immutable references (`&T`)\n   - **OR** exactly one mutable reference (`&mut T`)\n   - But **never both at the same time**!\n\n2. References must always be valid\n\nYour code violates rule #1. Here's why it fails:\n\n```rust\nlet mut s = String::from(\"hello\");\nlet r1 = &s;        // Immutable borrow starts\nlet r2 = &mut s;    // ERROR: Can't mutably borrow while immutably borrowed\nprintln!(\"{} {}\", r1, r2);  // r1's borrow would end here\n```\n\n## The Fix\n\nYou have a few options:\n\n### Option 1: Use references sequentially\n```rust\nlet mut s = String::from(\"hello\");\nlet r1 = &s;\nprintln!(\"Immutable: {}\", r1);  // r1's borrow ends here\n\nlet r2 = &mut s;  // Now we can mutably borrow\nr2.push_str(\", world\");\nprintln!(\"After mutation: {}\", r2);\n```\n\n### Option 2: Use scopes to limit borrow lifetime\n```rust\nlet mut s = String::from(\"hello\");\n{\n    let r1 = &s;\n    println!(\"Inside scope: {}\", r1);\n}  // r1 goes out of scope, borrow ends\n\nlet r2 = &mut s;  // Now safe to mutably borrow\nr2.push_str(\", world\");\n```\n\nThis prevents data races at compile time - pretty cool, right?",
            Role::Assistant,
        ),
    ];

    let mut main_branch = Branch::new("main");
    for msg in messages {
        main_branch.add_message(msg);
    }

    // Create alternative branch focused on lifetimes
    let alt_messages = vec![
        Message::new("Hello! How can I help you today?", Role::Assistant),
        Message::new("I need help understanding Rust's ownership system. I keep getting borrow checker errors and I don't understand why.", Role::User),
        Message::new(
            "I'll help you understand Rust's ownership system! Let's start with **lifetimes** since that's often where the confusion comes from.\n\n## What are Lifetimes?\n\nLifetimes are Rust's way of tracking how long references are valid. Every reference has a lifetime, even if you don't see it written explicitly.\n\n```rust\n// These are equivalent:\nfn foo(x: &str) { }\nfn foo<'a>(x: &'a str) { }  // 'a is the lifetime\n```\n\nWould you like me to explain when you need to explicitly annotate lifetimes?",
            Role::Assistant,
        ),
        Message::new("Yes, please! And can you show an example where lifetimes are actually needed?", Role::User),
        Message::new(
            "## When You Need Lifetime Annotations\n\nYou need explicit lifetimes when:\n1. Functions return references\n2. Structs hold references\n3. The compiler can't figure out the relationships\n\n## Example: Function Returning a Reference\n\n```rust\n// This won't compile - compiler doesn't know which lifetime to use\nfn longest(x: &str, y: &str) -> &str {\n    if x.len() > y.len() { x } else { y }\n}\n\n// Fixed with lifetime annotations\nfn longest<'a>(x: &'a str, y: &'a str) -> &'a str {\n    if x.len() > y.len() { x } else { y }\n}\n```\n\nThis tells Rust: \"The returned reference will live as long as the shorter of the two input lifetimes.\"\n\n## Example: Struct with References\n\n```rust\n// Struct holding a reference needs a lifetime\nstruct BookReview<'a> {\n    text: &'a str,  // This reference must outlive the struct\n}\n\nimpl<'a> BookReview<'a> {\n    fn level(&self) -> i32 {\n        3  // some logic here\n    }\n    \n    fn summary(&self) -> &str {\n        &self.text[..50]  // Lifetime automatically inferred!\n    }\n}\n```\n\n## Real World Example\n\n```rust\nfn main() {\n    let novel = String::from(\"Call me Ishmael. Some years ago...\");\n    let first_sentence: &str;\n    \n    {\n        let temp = String::from(\"Much shorter.\");\n        first_sentence = longest(novel.as_str(), temp.as_str());\n        // This could be either novel or temp!\n    }  // temp is dropped here\n    \n    // println!(\"{}\", first_sentence);  // Might be dangling!\n}\n```\n\nThe lifetime system prevents use-after-free bugs at compile time!",
            Role::Assistant,
        ),
    ];

    let mut alt_branch = Branch::new("lifetimes-focus");
    for msg in alt_messages {
        alt_branch.add_message(msg);
    }

    // Create a third branch about async Rust
    let async_messages = vec![
        Message::new("Hello! How can I help you today?", Role::Assistant),
        Message::new("I need help understanding Rust's ownership system. I keep getting borrow checker errors and I don't understand why.", Role::User),
        Message::new(
            "Let's explore ownership through a practical example with **async Rust**, which adds interesting ownership challenges!\n\n## Ownership in Async Context\n\n```rust\nuse tokio;\n\n#[tokio::main]\nasync fn main() {\n    let data = String::from(\"important\");\n    \n    // This won't work - why?\n    tokio::spawn(async {\n        println!(\"Data: {}\", data);  // Error: data might outlive the task\n    });\n}\n```\n\n## The Problem\n\nAsync tasks might outlive the scope where `data` was created. Rust can't guarantee `data` will still exist when the task runs.\n\n## Solutions\n\n### 1. Move ownership\n```rust\ntokio::spawn(async move {\n    println!(\"Data: {}\", data);  // Takes ownership\n});\n// data no longer accessible here\n```\n\n### 2. Clone for shared access\n```rust\nlet data_clone = data.clone();\ntokio::spawn(async move {\n    println!(\"Data: {}\", data_clone);\n});\nprintln!(\"Still have: {}\", data);\n```\n\n### 3. Arc for shared ownership\n```rust\nuse std::sync::Arc;\n\nlet data = Arc::new(String::from(\"shared\"));\nlet data_clone = Arc::clone(&data);\n\ntokio::spawn(async move {\n    println!(\"Task: {}\", data_clone);\n});\n\nprintln!(\"Main: {}\", data);\n```\n\nThis shows how ownership rules guide you toward safe concurrent code!",
            Role::Assistant,
        ),
    ];

    let mut async_branch = Branch::new("async-ownership");
    for msg in async_messages {
        async_branch.add_message(msg);
    }

    let mut tree = Tree::new("Rust Ownership Discussion");
    tree.add_branch(main_branch);
    tree.add_branch(alt_branch);
    tree.add_branch(async_branch);

    // Create second tree with error handling discussion
    let mut second_tree = Tree::new("Error Handling");
    let mut error_branch = Branch::new("main");
    
    let error_messages = vec![
        Message::new("How should I handle errors in Rust? Coming from Python where we use try/except everywhere.", Role::User),
        Message::new(
            "Rust's error handling is quite different from Python! Instead of exceptions, Rust uses the `Result<T, E>` type.\n\n## The Result Type\n\n```rust\nenum Result<T, E> {\n    Ok(T),   // Success case with value of type T\n    Err(E),  // Error case with error of type E\n}\n```\n\n## Basic Error Handling\n\n```rust\nuse std::fs::File;\nuse std::io::Read;\n\nfn read_username() -> Result<String, std::io::Error> {\n    let mut file = match File::open(\"username.txt\") {\n        Ok(file) => file,\n        Err(e) => return Err(e),  // Early return on error\n    };\n    \n    let mut username = String::new();\n    match file.read_to_string(&mut username) {\n        Ok(_) => Ok(username),\n        Err(e) => Err(e),\n    }\n}\n```\n\n## The ? Operator (Much Cleaner!)\n\n```rust\nfn read_username() -> Result<String, std::io::Error> {\n    let mut file = File::open(\"username.txt\")?;  // ? returns early on error\n    let mut username = String::new();\n    file.read_to_string(&mut username)?;\n    Ok(username)\n}\n```\n\n## Even More Concise\n\n```rust\nfn read_username() -> Result<String, std::io::Error> {\n    std::fs::read_to_string(\"username.txt\")\n}\n```\n\nNo hidden exceptions - errors are part of the type system!",
            Role::Assistant,
        ),
        Message::new("That's interesting! But what about custom error types? In Python I'd create custom exception classes.", Role::User),
        Message::new(
            "Great question! Custom error types in Rust are very powerful. Here are the common patterns:\n\n## 1. Simple Enum Errors\n\n```rust\n#[derive(Debug)]\nenum AppError {\n    NotFound,\n    PermissionDenied,\n    InvalidInput(String),\n}\n\nfn process_user(id: u32) -> Result<User, AppError> {\n    if id == 0 {\n        return Err(AppError::InvalidInput(\n            \"User ID cannot be zero\".to_string()\n        ));\n    }\n    // ... rest of logic\n}\n```\n\n## 2. Using thiserror Crate (Recommended)\n\n```rust\nuse thiserror::Error;\n\n#[derive(Error, Debug)]\nenum AppError {\n    #[error(\"User not found: {id}\")]\n    NotFound { id: u32 },\n    \n    #[error(\"Permission denied for user {0}\")]\n    PermissionDenied(String),\n    \n    #[error(\"Database error\")]\n    Database(#[from] sqlx::Error),  // Auto-conversion from sqlx errors\n    \n    #[error(\"IO error: {0}\")]\n    Io(#[from] std::io::Error),     // Auto-conversion from IO errors\n}\n```\n\n## 3. Using anyhow for Applications\n\n```rust\nuse anyhow::{Result, Context, bail};\n\nfn process_config(path: &str) -> Result<Config> {\n    let content = std::fs::read_to_string(path)\n        .context(\"Failed to read config file\")?;  // Adds context\n    \n    if content.is_empty() {\n        bail!(\"Config file cannot be empty\");  // Quick error return\n    }\n    \n    let config: Config = toml::from_str(&content)\n        .context(\"Failed to parse TOML\")?;\n        \n    Ok(config)\n}\n```\n\n## Error Propagation Pattern\n\n```rust\nfn main() -> Result<()> {\n    let user = get_user(42)?;\n    let profile = load_profile(&user)?;\n    render_page(&profile)?;\n    Ok(())\n}\n```\n\nMuch more explicit than Python's hidden exceptions, and the compiler ensures you handle errors!",
            Role::Assistant,
        ),
    ];
    
    for msg in error_messages {
        error_branch.add_message(msg);
    }
    second_tree.add_branch(error_branch);

    let mut dialogue = Dialogue::new("Programming Help Sessions");
    dialogue.add_tree(tree);
    dialogue.add_tree(second_tree);

    dialogue
}
