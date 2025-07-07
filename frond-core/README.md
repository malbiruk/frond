# frond-core

**frond-core** is the core library powering [frond](../README.md), a TUI LLM chat client with branching, context control, and structured conversation management.

## What is frond-core?

This crate provides the data structures, algorithms, and storage logic for managing complex, branchable LLM conversations. It is designed to be UI-agnostic and reusable across different frontends (TUI, GUI, web, etc.).

## Features

- **Conversation primitives:**
  - Messages (user/assistant, editable, hideable, reorderable)
  - Branches (fork from any message, merge, detach, attach)
  - Trees and Dialogues (organize conversations hierarchically)
- **Full context control:**
  - Edit, split, merge, or hide any message
  - Switch roles (user/assistant)
  - Track token usage per branch
- **History and organization:**
  - Per-message and per-tree edit history
  - Snapshots and checkpoints
  - Tags, folders, and search support
- **Persistence:**
  - JSON and/or SQLite storage abstraction
- **Extensible:**
  - Designed for use in TUI, GUI, or web frontends

## Usage

Add to your `Cargo.toml`:

```toml
frond-core = "0.1"
```

Or, if using in a workspace:

```toml
frond-core = { path = "../frond-core" }
```

Then in your code:

```rust
use frond_core::{Message, Branch, Tree, Dialogue, Role};
```

## Status

**Work in progress.**
APIs are unstable and may change rapidly.

## License

MIT © Klim Kostiuk
