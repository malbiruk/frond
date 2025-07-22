# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Frond is a TUI (Terminal UI) LLM chat client built in Rust that provides full context control, branching conversations, and structured thinking spaces. The project is organized as a workspace with two main crates:

- **frond-core**: Domain logic and data models (pure business logic)
- **frond-tui**: Terminal UI implementation using ratatui

## Common Development Commands

```bash
# Build the entire workspace
cargo build

# Build and run the TUI application
cargo run -p frond

# Run all tests
cargo test

# Run tests for specific crate
cargo test -p frond-core
cargo test -p frond-tui

# Run a specific test
cargo test test_name

# Check code without building
cargo check

# Format code
cargo fmt

# Lint with clippy
cargo clippy
```

## Architecture

The codebase follows a clean architecture pattern with clear separation between domain logic and UI concerns:

### Core Domain (frond-core)
- **Domain Models**: `Dialogue` → `Tree` → `Branch` → `Message` hierarchy
- **Actions System**: All mutations go through action/reducer pattern for undo/redo
- **Action Types**: `DialogueAction`, `TreeAction`, `BranchAction`, `MessageAction`

### TUI Layer (frond-tui) 
- **App State**: `AppState` holds dialogue data + UI state, implements `ActionDispatcher`
- **Input Handling**: KeyChord → ActionRegistry → UIAction pipeline
- **Mode System**: `Mode::Normal` and `Mode::Edit(EditMode)` with mode-specific actions
- **Action Flow**: User input → UIAction → Service layer → Core Action → Domain mutation

### Key Components
- **ActionRegistry**: Schema-based registry mapping action IDs to implementations
- **InputHandler**: Resolves key chords to UI actions with parameters
- **Reducer Pattern**: Both UI actions (frond-tui) and domain actions (frond-core) use reducers
- **DialogueService**: Translates UI operations to core domain actions

## File Structure

```
frond-core/src/
├── core/           # Domain models (Dialogue, Tree, Branch, Message)
├── actions/        # Domain actions and reducer logic

frond-tui/src/
├── actions/        # UI action schemas and registry
├── app/            # Application state and main loop
├── config/         # Configuration and keybindings
├── input/          # Input handling and key mapping
├── services/       # Business logic services
└── ui/             # Rendering and view calculations
```

## Development Notes

- The project uses edition 2024 Rust
- All domain mutations go through the action system for proper undo/redo support
- UI actions are separate from domain actions - services translate between them
- The ActionRegistry provides discoverability for all available user actions
- Mode-specific actions are handled by separate reducers (normal_mode_handler, edit_mode_handler)
- Tests are organized to mirror the source structure in both crates