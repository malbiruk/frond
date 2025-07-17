use crate::actions::{ActionDispatcher, UIAction};
use crate::config::Config;
use frond_core::Dialogue;
use ratatui::widgets::ScrollbarState;
use uuid::Uuid;

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
    pub scroll_offset: usize,
    pub scrollbar_state: ScrollbarState,

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
        let message_id = branch_id.and_then(|bid| {
            dialogue
                .get_branch_by_id(bid)
                .and_then(|b| b.messages().get(0))
                .map(|m| m.id())
        });

        Self {
            dialogue,
            mode: Mode::Normal,
            config: Config::default(),
            current_tree_id: tree_id,
            current_branch_id: branch_id,
            focused_message_id: message_id,
            scroll_offset: 0,
            scrollbar_state: ScrollbarState::default(),
            error_message: None,
        }
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

    pub fn update_focused_message_from_scroll(&mut self, viewport_height: usize) {
        let messages = self.current_messages();
        if messages.is_empty() {
            return;
        }

        // Calculate which message should be focused based on the center of the viewport
        let center_line = self.scroll_offset + viewport_height / 2;

        // Find the message that contains the center line
        let mut current_line = 0;
        for message in &messages {
            let message_height = self.calculate_message_display_height(message);
            if current_line + message_height > center_line {
                self.focused_message_id = Some(message.id());
                return;
            }
            current_line += message_height;
        }

        // If we didn't find a message, focus the last one
        if let Some(last_message) = messages.last() {
            self.focused_message_id = Some(last_message.id());
        }
    }

    pub fn calculate_message_display_height(&self, message: &frond_core::Message) -> usize {
        // More accurate message height calculation
        let content_width = 76; // Account for borders and padding (80 - 4)
        let content = message.content();

        let mut line_count = 0;
        for line in content.lines() {
            if line.is_empty() {
                line_count += 1;
            } else {
                line_count += (line.len() + content_width - 1) / content_width; // Ceiling division
            }
        }

        if line_count == 0 {
            line_count = 1; // At least one line for empty content
        }

        line_count + 2 // +2 for top and bottom borders
    }

    pub fn get_total_content_height(&self) -> usize {
        let messages = self.current_messages();
        let total: usize = messages
            .iter()
            .map(|msg| self.calculate_message_display_height(msg))
            .sum();
        total.max(1) // Ensure at least 1 for empty conversations
    }

    pub fn update_scrollbar_state(&mut self, viewport_height: usize) {
        let total_height = self.get_total_content_height();
        let max_scroll = total_height.saturating_sub(1);
        self.scrollbar_state = self
            .scrollbar_state
            .content_length(total_height)
            .viewport_content_length(viewport_height)
            .position(self.scroll_offset.min(max_scroll));
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
        if messages.is_empty() {
            self.focused_message_id = None;
        } else if deleted_index < messages.len() {
            self.focused_message_id = Some(messages[deleted_index].id());
        } else if deleted_index > 0 {
            self.focused_message_id = Some(messages[deleted_index - 1].id());
        }
    }

    pub fn reset_focus_for_new_branch(&mut self) {
        if let Some(branch) = self.current_branch() {
            if let Some(first_message) = branch.messages().get(0) {
                self.focused_message_id = Some(first_message.id());
                self.scroll_offset = 0;
                self.scrollbar_state = ScrollbarState::default();
            }
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
