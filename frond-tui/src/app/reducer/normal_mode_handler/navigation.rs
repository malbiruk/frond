use crate::app::state::{AppState, ScrollingRequest};

pub fn handle_next_branch(state: &mut AppState) {
    if let Some(tree) = state.current_tree() {
        if let Some(current_branch_id) = state.current_branch_id {
            let branches = tree.branches();
            if let Some(current_index) = branches.get_index_by_id(current_branch_id) {
                // Get current focused message index before switching
                let current_focused_index = state
                    .focused_message_id
                    .and_then(|id| state.get_message_index(id))
                    .unwrap_or(0);

                let next_index = (current_index + 1) % branches.len();
                if let Some(next_branch) = branches.get(next_index) {
                    state.current_branch_id = Some(next_branch.id());
                    state.reset_focus_for_new_branch();

                    // Request focus on same index, or last message if new branch is shorter
                    state.pending_scrolling_request =
                        Some(ScrollingRequest::ScrollToMessageWithSameIndexOrLast {
                            previous_index: current_focused_index,
                        });
                }
            }
        }
    }
}

pub fn handle_prev_branch(state: &mut AppState) {
    if let Some(tree) = state.current_tree() {
        if let Some(current_branch_id) = state.current_branch_id {
            let branches = tree.branches();
            if let Some(current_index) = branches.get_index_by_id(current_branch_id) {
                // Get current focused message index before switching
                let current_focused_index = state
                    .focused_message_id
                    .and_then(|id| state.get_message_index(id))
                    .unwrap_or(0);

                let prev_index = if current_index == 0 {
                    branches.len() - 1
                } else {
                    current_index - 1
                };
                if let Some(prev_branch) = branches.get(prev_index) {
                    state.current_branch_id = Some(prev_branch.id());
                    state.reset_focus_for_new_branch();

                    // Request focus on same index, or last message if new branch is shorter
                    state.pending_scrolling_request =
                        Some(ScrollingRequest::ScrollToMessageWithSameIndexOrLast {
                            previous_index: current_focused_index,
                        });
                }
            }
        }
    }
}

pub fn handle_next_tree(state: &mut AppState) {
    let trees = state.dialogue.trees();
    if let Some(current_tree_id) = state.current_tree_id {
        if let Some(current_index) = trees.get_index_by_id(current_tree_id) {
            let next_index = (current_index + 1) % trees.len();
            if let Some(next_tree) = trees.get(next_index) {
                state.current_tree_id = Some(next_tree.id());
                state.reset_focus_for_new_tree();

                // Request focus on last message of new tree
                state.pending_scrolling_request = Some(ScrollingRequest::ScrollToLastMessage);
            }
        }
    }
}

pub fn handle_prev_tree(state: &mut AppState) {
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

                // Request focus on last message of new tree
                state.pending_scrolling_request = Some(ScrollingRequest::ScrollToLastMessage);
            }
        }
    }
}