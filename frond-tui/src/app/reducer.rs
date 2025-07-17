use super::state::AppState;
use crate::actions::{CommonAction, EditModeAction, NormalModeAction, UIAction};
use frond_core::{Action, BranchAction, MessageAction, TreeAction};
use uuid::Uuid;

pub fn reduce(state: &mut AppState, action: UIAction) {
    match action {
        UIAction::Common(common_action) => handle_common_action(state, common_action),
        UIAction::NormalMode(normal_action) => handle_normal_mode_action(state, normal_action),
        UIAction::EditMode(edit_action) => handle_edit_mode_action(state, edit_action),
    }
}

fn handle_common_action(state: &mut AppState, action: CommonAction) {
    match action {
        CommonAction::Quit => {
            // Handle quit - this should be handled at the app level
        }
        CommonAction::ShowError(message) => handle_show_error(state, message),
        CommonAction::ClearError => handle_clear_error(state),
        CommonAction::UpdateConfig(config) => handle_update_config(state, config),
    }
}

fn handle_normal_mode_action(state: &mut AppState, action: NormalModeAction) {
    match action {
        // Navigation actions
        NormalModeAction::ScrollUp => handle_scroll_up(state),
        NormalModeAction::ScrollDown => handle_scroll_down(state),
        NormalModeAction::ScrollToMessage(message_id) => {
            handle_scroll_to_message(state, message_id)
        }
        NormalModeAction::FocusMessage(message_id) => handle_focus_message(state, message_id),

        // Mode transitions
        NormalModeAction::EnterEditMode(message_id) => handle_enter_edit_mode(state, message_id),
        NormalModeAction::EnterAppendMode => handle_enter_append_mode(state),
        NormalModeAction::EnterCommandPalette => handle_enter_command_palette(state),

        // Content actions that map to frond-core
        NormalModeAction::EditMessage {
            message_id,
            content,
        } => handle_edit_message(state, message_id, content),
        NormalModeAction::AppendMessage(content) => handle_append_message(state, content),
        NormalModeAction::DeleteMessage(message_id) => handle_delete_message(state, message_id),
        NormalModeAction::ForkBranch(message_id) => handle_fork_branch(state, message_id),
        NormalModeAction::HideMessage(message_id) => handle_hide_message(state, message_id),
        NormalModeAction::ShowMessage(message_id) => handle_show_message(state, message_id),

        // Branch/Tree navigation
        NormalModeAction::NextBranch => handle_next_branch(state),
        NormalModeAction::PrevBranch => handle_prev_branch(state),
        NormalModeAction::NextTree => handle_next_tree(state),
        NormalModeAction::PrevTree => handle_prev_tree(state),

        // UI-specific actions
        NormalModeAction::ShowHelp => handle_show_help(state),
    }
}

fn handle_edit_mode_action(state: &mut AppState, action: EditModeAction) {
    match action {
        EditModeAction::ExitCurrentMode => handle_exit_current_mode(state),
    }
}

// Navigation handlers
fn handle_scroll_up(state: &mut AppState) {
    state.scroll_offset = state.scroll_offset.saturating_sub(1);
    // We'll update focus when rendering, as we need viewport height
}

fn handle_scroll_down(state: &mut AppState) {
    let total_height = state.get_total_content_height();
    // Allow scrolling until only one line of content is visible at the top
    let max_scroll = total_height.saturating_sub(1);
    if state.scroll_offset < max_scroll {
        state.scroll_offset = state.scroll_offset.saturating_add(1);
    }
    // We'll update focus when rendering, as we need viewport height
}

// Mode transition handlers
fn handle_enter_edit_mode(state: &mut AppState, message_id: Uuid) {
    if message_id == Uuid::nil() {
        state.error_message = Some("No message selected for editing".to_string());
        return;
    }

    let has_messages_below = state.has_messages_after(message_id);
    state.mode = crate::app::Mode::Edit(crate::app::EditMode::EditInPlace {
        message_id,
        has_messages_below,
    });
    state.scroll_for_edit_mode();
}

fn handle_enter_append_mode(state: &mut AppState) {
    state.mode = crate::app::Mode::Edit(crate::app::EditMode::Append);
    state.scroll_for_edit_mode();
}

fn handle_enter_command_palette(state: &mut AppState) {
    // TODO: Implement command palette mode
    state.error_message = Some("Command palette not implemented yet".to_string());
}

fn handle_exit_current_mode(state: &mut AppState) {
    state.mode = crate::app::Mode::Normal;
    state.clear_error();
}

// Content action handlers that bridge to frond-core
fn handle_edit_message(state: &mut AppState, message_id: Uuid, content: String) {
    let old_content = state
        .dialogue
        .get_message_by_id(message_id)
        .map(|m| m.content().to_string())
        .unwrap_or_default();

    let core_action = Action::Message(MessageAction::EditMessage {
        message_id,
        old_content,
        new_content: content,
    });

    if let Err(e) = state.dialogue.apply_action(core_action) {
        state.error_message = Some(format!("Failed to edit message: {}", e));
    }
}

fn handle_append_message(state: &mut AppState, content: String) {
    if let Some(branch_id) = state.current_branch_id {
        let core_action = Action::Branch(BranchAction::AppendMessage {
            branch_id,
            message_content: content,
        });

        if let Err(e) = state.dialogue.apply_action(core_action) {
            state.error_message = Some(format!("Failed to append message: {}", e));
        } else {
            // Update focused message to the new one
            if let Some(branch) = state.current_branch() {
                if let Some(last_message) = branch.messages().iter().last() {
                    state.focused_message_id = Some(last_message.id());
                }
            }
        }
    }
}

fn handle_delete_message(state: &mut AppState, message_id: Uuid) {
    if let Some(branch_id) = state.current_branch_id {
        if let Some(branch) = state.dialogue.get_branch_by_id(branch_id) {
            if let Some(message_index) = branch.get_message_index_by_id(message_id) {
                if let Some(message) = branch.get_message_by_id(message_id) {
                    let core_action = Action::Message(MessageAction::DeleteMessage {
                        message_id,
                        branch_id,
                        message_index,
                        deleted_message: message.clone(),
                    });

                    if let Err(e) = state.dialogue.apply_action(core_action) {
                        state.error_message = Some(format!("Failed to delete message: {}", e));
                    } else {
                        // Update focused message
                        state.update_focused_message_after_deletion(message_index);
                    }
                }
            }
        }
    }
}

fn handle_fork_branch(state: &mut AppState, message_id: Uuid) {
    if let Some(tree_id) = state.current_tree_id {
        if let Some(branch_id) = state.current_branch_id {
            let new_branch_name = format!("fork-{}", chrono::Utc::now().timestamp());

            let core_action = Action::Tree(TreeAction::ForkBranch {
                tree_id,
                branch_id,
                from_message_id: message_id,
                new_branch_name,
            });

            if let Err(e) = state.dialogue.apply_action(core_action) {
                state.error_message = Some(format!("Failed to fork branch: {}", e));
            }
        }
    }
}

fn handle_hide_message(state: &mut AppState, message_id: Uuid) {
    let core_action = Action::Message(MessageAction::HideMessage { message_id });

    if let Err(e) = state.dialogue.apply_action(core_action) {
        state.error_message = Some(format!("Failed to hide message: {}", e));
    }
}

fn handle_show_message(state: &mut AppState, message_id: Uuid) {
    let core_action = Action::Message(MessageAction::ShowMessage { message_id });

    if let Err(e) = state.dialogue.apply_action(core_action) {
        state.error_message = Some(format!("Failed to show message: {}", e));
    }
}

// Branch/Tree navigation handlers
fn handle_next_branch(state: &mut AppState) {
    if let Some(tree) = state.current_tree() {
        if let Some(current_branch_id) = state.current_branch_id {
            let branches = tree.branches();
            if let Some(current_index) = branches.get_index_by_id(current_branch_id) {
                let next_index = (current_index + 1) % branches.len();
                if let Some(next_branch) = branches.get(next_index) {
                    state.current_branch_id = Some(next_branch.id());
                    state.reset_focus_for_new_branch();
                }
            }
        }
    }
}

fn handle_prev_branch(state: &mut AppState) {
    if let Some(tree) = state.current_tree() {
        if let Some(current_branch_id) = state.current_branch_id {
            let branches = tree.branches();
            if let Some(current_index) = branches.get_index_by_id(current_branch_id) {
                let prev_index = if current_index == 0 {
                    branches.len() - 1
                } else {
                    current_index - 1
                };
                if let Some(prev_branch) = branches.get(prev_index) {
                    state.current_branch_id = Some(prev_branch.id());
                    state.reset_focus_for_new_branch();
                }
            }
        }
    }
}

fn handle_next_tree(state: &mut AppState) {
    let trees = state.dialogue.trees();
    if let Some(current_tree_id) = state.current_tree_id {
        if let Some(current_index) = trees.get_index_by_id(current_tree_id) {
            let next_index = (current_index + 1) % trees.len();
            if let Some(next_tree) = trees.get(next_index) {
                state.current_tree_id = Some(next_tree.id());
                state.reset_focus_for_new_tree();
            }
        }
    }
}

fn handle_prev_tree(state: &mut AppState) {
    let trees = state.dialogue.trees();
    if let Some(current_tree_id) = state.current_tree_id {
        if let Some(current_index) = trees.get_index_by_id(current_tree_id) {
            let prev_index = if current_index == 0 {
                trees.len() - 1
            } else {
                current_index - 1
            };
            if let Some(prev_tree) = trees.get(prev_index) {
                state.current_tree_id = Some(prev_tree.id());
                state.reset_focus_for_new_tree();
            }
        }
    }
}

// UI-specific handlers
fn handle_update_config(state: &mut AppState, config: crate::config::Config) {
    state.config = config;
}

fn handle_show_help(state: &mut AppState) {
    // TODO: Implement help system
    state.error_message = Some("Help system not implemented yet".to_string());
}

// Error handling
fn handle_show_error(state: &mut AppState, message: String) {
    state.error_message = Some(message);
}

fn handle_clear_error(state: &mut AppState) {
    state.error_message = None;
}

fn handle_scroll_to_message(state: &mut AppState, message_id: Uuid) {
    if let Some(index) = state.get_message_index(message_id) {
        state.scroll_offset = index;
        state.focused_message_id = Some(message_id);
    }
}

fn handle_focus_message(state: &mut AppState, message_id: Uuid) {
    state.focused_message_id = Some(message_id);
    // Center the focused message in the viewport
    if let Some(message_index) = state.get_message_index(message_id) {
        let messages = state.current_messages();
        let mut line_offset: usize = 0;
        for (i, message) in messages.iter().enumerate() {
            if i == message_index {
                // Center this message by setting scroll to position it in the middle
                let _message_height = state.calculate_message_display_height(message);
                let viewport_height = 20; // Default estimate - will be updated during render
                let center_offset = viewport_height / 2;
                state.scroll_offset = line_offset.saturating_sub(center_offset);
                break;
            }
            line_offset += state.calculate_message_display_height(message);
        }
    }
}
