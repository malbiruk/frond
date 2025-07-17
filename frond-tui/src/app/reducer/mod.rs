use super::state::AppState;
use crate::{
    actions::{CommonAction, UIAction},
    app::Mode,
};
pub mod edit_mode_handler;
pub mod normal_mode_handler;

pub fn reduce(state: &mut AppState, action: UIAction) {
    match action {
        UIAction::Common(common_action) => handle_common_action(state, common_action),
        UIAction::NormalMode(normal_action) => {
            normal_mode_handler::handle_normal_mode_action(state, normal_action)
        }
        UIAction::EditMode(edit_action) => {
            edit_mode_handler::handle_edit_mode_action(state, edit_action)
        }
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
        CommonAction::ShowHelp(mode) => handle_show_help(state, mode),
    }
}

fn handle_update_config(state: &mut AppState, config: crate::config::Config) {
    state.config = config;
}

fn handle_show_error(state: &mut AppState, message: String) {
    state.error_message = Some(message);
}

fn handle_clear_error(state: &mut AppState) {
    state.error_message = None;
}

fn handle_show_help(state: &mut AppState, _mode: Mode) {
    // TODO: Implement help system
    state.error_message = Some("Help system not implemented yet".to_string());
}
