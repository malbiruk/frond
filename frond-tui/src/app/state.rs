use crate::actions::{ActionDispatcher, UIAction};
use crate::config::Config;
use crate::ui::normal_mode::scrolling;
use frond_core::Dialogue;
use ratatui::widgets::ScrollbarState;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FocusRequest {
    Message(Uuid),
    LastMessage,
    SameIndexOrLast { previous_index: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal,
    Edit(EditMode),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditMode {
    EditInPlace {
        message_id: Uuid,
        has_messages_below: bool,
    },
    Append,
}

#[derive(Debug)]
pub struct AppState {
    pub dialogue: Dialogue,
    pub mode: Mode,
    pub config: Config,

    // Navigation state
    pub current_tree_id: Option<Uuid>,
    pub current_branch_id: Option<Uuid>,
    pub focused_message_id: Option<Uuid>,

    // Scrolling state
    pub scroll_offset: isize,
    pub scrollbar_state: ScrollbarState,

    // Focus request (processed during render)
    pub pending_focus_request: Option<FocusRequest>,

    // Error state
    pub error_message: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        let dialogue = super::create_test_dialogue();

        // Set up initial navigation
        let tree_id = dialogue.trees().get(0).map(|t| t.id());
        let branch_id = tree_id.and_then(|tid| {
            dialogue
                .get_tree_by_id(tid)
                .and_then(|t| t.branches().get(0))
                .map(|b| b.id())
        });

        let mut state = Self {
            dialogue,
            mode: Mode::Normal,
            config: Config::default(),
            current_tree_id: tree_id,
            current_branch_id: branch_id,
            focused_message_id: None,
            scroll_offset: 0,
            scrollbar_state: ScrollbarState::default(),
            pending_focus_request: None,
            error_message: None,
        };

        // Request focus on last message if available
        if state
            .current_branch()
            .is_some_and(|b| !b.messages().is_empty())
        {
            state.pending_focus_request = Some(FocusRequest::LastMessage);
        }

        state
    }
}

impl ActionDispatcher for AppState {
    fn dispatch(&mut self, action: UIAction) {
        super::reducer::reduce(self, action);
    }
}

impl AppState {
    pub fn current_tree(&self) -> Option<&frond_core::Tree> {
        self.current_tree_id
            .and_then(|tree_id| self.dialogue.get_tree_by_id(tree_id))
    }

    pub fn current_branch(&self) -> Option<&frond_core::Branch> {
        self.current_branch_id
            .and_then(|branch_id| self.dialogue.get_branch_by_id(branch_id))
    }

    pub fn current_messages(&self) -> Vec<&frond_core::Message> {
        self.current_branch()
            .map(|b| b.messages().iter().collect())
            .unwrap_or_default()
    }

    pub fn update_focused_message_from_scroll(
        &mut self,
        viewport_height: isize,
        viewport_width: u16,
    ) {
        let messages = self.current_messages();
        self.focused_message_id = scrolling::update_focused_message_from_scroll(
            &messages,
            self.scroll_offset,
            viewport_height,
            viewport_width,
        );
    }

    pub fn calculate_message_display_height(
        &self,
        message: &frond_core::Message,
        viewport_width: u16,
    ) -> usize {
        scrolling::calculate_message_display_height(message, viewport_width)
    }

    pub fn get_total_content_height(&self, viewport_width: u16) -> usize {
        let messages = self.current_messages();
        scrolling::calculate_total_content_height(&messages, viewport_width)
    }

    pub fn update_scrollbar_state(&mut self, viewport_height: usize, viewport_width: u16) {
        let total_height = self.get_total_content_height(viewport_width);
        self.scrollbar_state = scrolling::update_scrollbar_state(
            self.scrollbar_state,
            self.scroll_offset,
            viewport_height,
            total_height,
        );
    }

    pub fn get_message_index(&self, message_id: Uuid) -> Option<usize> {
        self.current_branch()?.get_message_index_by_id(message_id)
    }

    pub fn has_messages_after(&self, message_id: Uuid) -> bool {
        if let Some(branch) = self.current_branch() {
            if let Some(index) = branch.get_message_index_by_id(message_id) {
                return index < branch.messages().len() - 1;
            }
        }
        false
    }

    pub fn scroll_for_edit_mode(&mut self) {
        match self.mode {
            Mode::Edit(EditMode::EditInPlace {
                has_messages_below: true,
                ..
            }) => {
                // TODO: Implement center edit area scrolling
            }
            Mode::Edit(EditMode::EditInPlace {
                has_messages_below: false,
                ..
            })
            | Mode::Edit(EditMode::Append) => {
                // TODO: Implement bottom edit area scrolling
            }
            _ => {}
        }
    }

    pub fn update_focused_message_after_deletion(&mut self, deleted_index: usize) {
        let messages = self.current_messages();
        self.focused_message_id =
            scrolling::update_focused_message_after_deletion(&messages, deleted_index);
    }

    pub fn reset_focus_for_new_branch(&mut self) {
        if let Some(branch) = self.current_branch() {
            let messages: Vec<&frond_core::Message> = branch.messages().iter().collect();
            self.focused_message_id = scrolling::get_first_message_id(&messages);
            self.scroll_offset = 0;
            self.scrollbar_state = ScrollbarState::default();
        }
    }

    pub fn reset_focus_for_new_tree(&mut self) {
        if let Some(tree) = self.current_tree() {
            if let Some(first_branch) = tree.branches().get(0) {
                self.current_branch_id = Some(first_branch.id());
                self.reset_focus_for_new_branch();
            }
        }
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
}
