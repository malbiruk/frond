mod scrolling;
mod mode_transitions;
mod message_actions;
mod navigation;

use crate::actions::NormalModeAction;
use crate::app::state::AppState;

pub fn handle_normal_mode_action(state: &mut AppState, action: NormalModeAction) {
    match action {
        // Navigation actions
        NormalModeAction::ScrollUp => scrolling::handle_scroll_up(state),
        NormalModeAction::ScrollDown => scrolling::handle_scroll_down(state),
        NormalModeAction::ScrollHalfPageUp => scrolling::handle_scroll_half_page_up(state),
        NormalModeAction::ScrollHalfPageDown => scrolling::handle_scroll_half_page_down(state),
        NormalModeAction::ScrollPageUp => scrolling::handle_scroll_page_up(state),
        NormalModeAction::ScrollPageDown => scrolling::handle_scroll_page_down(state),
        NormalModeAction::ScrollToTop => scrolling::handle_scroll_to_top(state),
        NormalModeAction::ScrollToBottom => scrolling::handle_scroll_to_bottom(state),
        NormalModeAction::ScrollToNextMessage => scrolling::handle_scroll_to_next_message(state),
        NormalModeAction::ScrollToPreviousMessage => scrolling::handle_scroll_to_previous_message(state),

        // Mode transitions
        NormalModeAction::EnterEditMode => mode_transitions::handle_enter_edit_mode(state),
        NormalModeAction::EnterAppendMode => mode_transitions::handle_enter_append_mode(state),
        NormalModeAction::EnterCommandPalette => mode_transitions::handle_enter_command_palette(state),

        // Message actions that use frond-core
        NormalModeAction::DeleteMessage => message_actions::handle_delete_message(state),
        NormalModeAction::ForkBranch => message_actions::handle_fork_branch(state),
        NormalModeAction::HideMessage => message_actions::handle_hide_message(state),
        NormalModeAction::ShowMessage => message_actions::handle_show_message(state),

        // Branch/Tree navigation
        NormalModeAction::NextBranch => navigation::handle_next_branch(state),
        NormalModeAction::PrevBranch => navigation::handle_prev_branch(state),
        NormalModeAction::NextTree => navigation::handle_next_tree(state),
        NormalModeAction::PrevTree => navigation::handle_prev_tree(state),
    }
}