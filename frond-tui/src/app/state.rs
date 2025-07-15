use crate::config::Config;
use frond_core::Dialogue;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal,
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
        }
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
}
