# Changelog

All notable changes to Frond will be documented in this file.

## [Unreleased]

### frond-tui

#### 2025-08-09
- Added hierarchical caching system for improved rendering performance
- Added edit and append modes for message modification
- Added system clipboard integration with fallback to internal clipboard
- Added text selection operations
- Refactored edit mode handler into modular submodules

#### 2025-08-06
- Added syntax highlighting for markdown message content using bat crate
- Added smart focus behavior for large messages (show beginning/end instead of centering)

#### 2025-07-22
- Added error popup with Esc to dismiss
- Added scroll to next/previous message functionality

#### 2025-07-21
- Added advanced scrolling actions
- Added customizable theme options

#### 2025-07-18
- Added DialogueService to translate UI operations to core domain actions
- Refactored action system to use schema enums
- Added focus and scrolling improvements with center-on-last-message

#### 2025-07-17
- Added scrollbar support
- Added action system, keybinding, and state management modules
- Reorganized UI actions into mode-specific modules

#### 2025-07-15
- Added static UI rendering
- Established action/reducer architecture for UI layer

### frond-core

#### 2025-07-15
- Added collection wrappers for core entities
- Added action system and action stack for undo/redo support
- Established core action logic with dedicated actions module

#### 2025-07-10
- Added archive, trash, and tag methods to Dialogue

#### 2025-07-08
- Added core entity API with Dialogue, Tree, Branch, Message hierarchy
- Initialize workspace structure with frond-core crate

### Project Setup

#### 2025-07-09
- Added MIT license

#### 2025-06-21
- Initial repository with project vision

## Architecture

The project follows clean architecture with two main crates:

- **frond-core**: Domain logic with Dialogue → Tree → Branch → Message hierarchy
- **frond-tui**: Terminal UI implementation using ratatui
