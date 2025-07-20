# Architecture Overview

## 1. Project Structure

```
frond/
├── frond-core/                 # Domain logic & data models
│   ├── src/core/              # Core domain models
│   │   ├── dialogue.rs        # Dialogue aggregate root
│   │   ├── tree.rs           # Tree entity with branches
│   │   ├── branch.rs         # Branch entity with messages
│   │   └── message.rs        # Message value object
│   └── src/actions/          # Domain actions & reducer
│       ├── action.rs         # Action enum (Dialogue/Tree/Branch/Message)
│       ├── *_action.rs       # Specific action types
│       └── reducer.rs        # Action application/undo logic
└── frond-tui/                 # Terminal UI implementation
    ├── src/config/           # Configuration & keybindings
    ├── src/input/            # Input handling & key mapping
    ├── src/actions/          # UI action schemas & registry
    ├── src/app/              # Application state & runner
    │   ├── runner.rs         # Main app loop
    │   ├── state.rs          # AppState (data + UI state)
    │   └── reducer/          # UI action handlers
    ├── src/services/         # Business logic services
    └── src/ui/               # Rendering & view calculations
```

## 2. User Intent → Realization Flow

```
[User Intent: Press 'e' to edit message]
           ↓
[Config] → KeyChord('e') → ActionRegistry.get_schema("edit_message")
           ↓
[Input] → InputHandler.handle_input() → ActionSchema → UIAction::EditMessage(msg_id)
           ↓
[App] → App.run() event loop → state.dispatch(action)
           ↓
[State] → AppState implements ActionDispatcher → reducer::reduce()
           ↓
[Reducer] → match UIAction → normal_mode_handler::handle_normal_mode_action()
           ↓
[Service] → DialogueService::edit_message() → constructs frond_core::Action
           ↓
[Core] → dialogue.apply_action(action) → reducer.reduce_action_apply()
           ↓
[Domain] → Action saved to dialogue.action_stack (undo/redo)
           ↓
[UI] → App.run() → terminal.draw() → ui::render() → updated display
```

### Key Components:

- **Config**: Keybindings map KeyChord → ActionID
- **InputHandler**: Resolves KeyChord → ActionSchema → UIAction (with params)
- **ActionRegistry**: Schema-based registry for user-discoverable actions
- **AppState**: Holds dialogue + UI state, implements ActionDispatcher
- **Reducer**: Routes UIActions to appropriate handlers (normal/edit/common)
- **DialogueService**: Translates UI operations to frond-core Actions
- **frond-core**: Domain actions applied via reducer, saved to action_stack for undo/redo

### Action Flow Types:

1. **UI Actions** (frond-tui): User intent, mode-specific, UI state changes
2. **Domain Actions** (frond-core): Data mutations, applied/undone via reducer, persisted in action_stack