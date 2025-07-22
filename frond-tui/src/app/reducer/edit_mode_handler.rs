use crate::actions::EditModeAction;
use crate::app::Mode;
use crate::app::state::AppState;

pub fn handle_edit_mode_action(state: &mut AppState, action: EditModeAction) {
    match action {
        EditModeAction::ExitCurrentMode => handle_exit_current_mode(state),
    }
}

fn handle_exit_current_mode(state: &mut AppState) {
    state.mode = Mode::Normal;
    state.error_message = None;
}
