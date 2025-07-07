# frond

**frond** 🌿 is a terminal-based (TUI) LLM chat client with branching, full context control, and structured conversation management.

Powered by [frond-core](../frond-core/README.md).

## Features

- **Full context control:**
  - Edit, hide, reorder, split/merge messages
  - Switch roles (user/assistant)
- **Branchable conversations:**
  - Fork from any message, explore side paths, merge or detach branches
- **Structured thinking space:**
  - Organize conversations as folders → dialogues → trees → branches → messages
- **Token usage tracking:**
  - See token count per branch/context
- **History and organization:**
  - Undo/redo, edit history, checkpoints, tags, folders, search
- **TUI-first:**
  - Fast, keyboard-driven, split-friendly, easy to pass file structures or context to the LLM

## Getting Started

1. **Install Rust:**
   https://rustup.rs

2. **Build and run:**
   ```sh
   cargo run --release -p frond
   ```

3. **Usage:**
   - Navigate conversations, edit messages, branch, and interact with your LLM (OpenAI, Ollama, etc.)

## Status

**Alpha.**
APIs and UI are under active development.

## License

MIT © Klim Kostiuk
