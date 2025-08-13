use crate::app::state::AppState;
use crate::services::DialogueService;

pub fn handle_delete_message(state: &mut AppState) {
    let Some(message_id) = state.focused_message_id else {
        state.error_message = Some("No message selected for deletion".to_string());
        return;
    };

    if let Some(branch_id) = state.current_branch_id {
        let message_index = state.get_message_index(message_id);

        match DialogueService::delete_message(&mut state.dialogue, message_id, branch_id) {
            Ok(_) => {
                // Invalidate cache for deleted message
                state.invalidate_message_highlight(message_id);
                state.invalidate_message_height(message_id);
                if let Some(index) = message_index {
                    state.update_focused_message_after_deletion(index);
                }
            }
            Err(e) => {
                state.error_message = Some(e);
            }
        }
    }
}

pub fn handle_fork_branch(state: &mut AppState) {
    let Some(message_id) = state.focused_message_id else {
        state.error_message = Some("No message selected for forking".to_string());
        return;
    };

    if let Some(tree_id) = state.current_tree_id {
        if let Some(branch_id) = state.current_branch_id {
            let new_branch_name = format!("fork-{}", chrono::Utc::now().timestamp());

            match DialogueService::fork_branch(
                &mut state.dialogue,
                tree_id,
                branch_id,
                message_id,
                new_branch_name,
            ) {
                Ok(_) => {
                    // Branch created successfully
                }
                Err(e) => {
                    state.error_message = Some(e);
                }
            }
        }
    }
}

pub fn handle_hide_message(state: &mut AppState) {
    let Some(message_id) = state.focused_message_id else {
        state.error_message = Some("No message selected for hiding".to_string());
        return;
    };

    if let Err(e) = DialogueService::hide_message(&mut state.dialogue, message_id) {
        state.error_message = Some(e);
    }
}

pub fn handle_show_message(state: &mut AppState) {
    let Some(message_id) = state.focused_message_id else {
        state.error_message = Some("No message selected for showing".to_string());
        return;
    };

    if let Err(e) = DialogueService::show_message(&mut state.dialogue, message_id) {
        state.error_message = Some(e);
    }
}