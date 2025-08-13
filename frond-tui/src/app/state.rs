use crate::actions::{ActionDispatcher, UIAction};
use crate::config::Config;
use crate::ui::normal_mode::scrolling;
use frond_core::Dialogue;
use ratatui::widgets::ScrollbarState;
use std::collections::HashMap;
use tui_textarea::TextArea;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrollingRequest {
    ScrollToMessage(Uuid),
    ScrollToLastMessage,
    ScrollToMessageWithSameIndexOrLast { previous_index: usize },
    ScrollToTop,
    ScrollToBottom,
    ScrollPageUp,
    ScrollPageDown,
    ScrollHalfPageUp,
    ScrollHalfPageDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal,
    Edit,
}

#[derive(Debug, Default)]
pub struct ModelInfo {
    pub tokens_used: u32,
    pub tokens_available: u32,
}

#[derive(Debug)]
pub struct AppState {
    pub dialogue: Dialogue,
    pub mode: Mode,
    pub config: Config,
    pub model_info: ModelInfo,

    // Navigation state
    pub current_tree_id: Option<Uuid>,
    pub current_branch_id: Option<Uuid>,
    pub focused_message_id: Option<Uuid>,

    // Scrolling
    pub scroll_offset: isize,
    pub scrollbar_state: ScrollbarState,
    pub pending_scrolling_request: Option<ScrollingRequest>,

    // Error state
    pub error_message: Option<String>,

    // Edit mode state
    pub edit_textarea: Option<TextArea<'static>>,

    // Syntax highlighting cache (tree_id, branch_id, message_id) -> highlighted_text
    pub highlight_cache: HashMap<(Uuid, Uuid, Uuid), ratatui::text::Text<'static>>,
    
    // Height cache (tree_id, branch_id, message_id, viewport_width) -> height
    pub height_cache: HashMap<(Uuid, Uuid, Uuid, u16), usize>,
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
            model_info: ModelInfo::default(),
            current_tree_id: tree_id,
            current_branch_id: branch_id,
            focused_message_id: None,
            scroll_offset: 0,
            scrollbar_state: ScrollbarState::default(),
            pending_scrolling_request: None,
            error_message: None,
            edit_textarea: None,
            highlight_cache: HashMap::new(),
            height_cache: HashMap::new(),
        };

        // Request focus on last message if available
        if state
            .current_branch()
            .is_some_and(|b| !b.messages().is_empty())
        {
            state.pending_scrolling_request = Some(ScrollingRequest::ScrollToLastMessage);
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

    // Internal helper methods - not user actions
    pub fn focus_message(&mut self, message_id: Uuid) {
        // Find which tree and branch contain this message
        let mut found_tree_id = None;
        let mut found_branch_id = None;
        
        'outer: for tree in self.dialogue.trees().iter() {
            for branch in tree.branches().iter() {
                if branch.get_message_by_id(message_id).is_some() {
                    found_tree_id = Some(tree.id());
                    found_branch_id = Some(branch.id());
                    break 'outer;
                }
            }
        }
        
        // Switch to the tree and branch containing the message
        if let (Some(tree_id), Some(branch_id)) = (found_tree_id, found_branch_id) {
            self.current_tree_id = Some(tree_id);
            self.current_branch_id = Some(branch_id);
        }
        
        // Request focus - will be resolved during render with real viewport
        self.pending_scrolling_request = Some(ScrollingRequest::ScrollToMessage(message_id));
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

    fn current_cache_context(&self) -> Option<(Uuid, Uuid)> {
        Some((self.current_tree_id?, self.current_branch_id?))
    }

    pub fn get_highlighted_text(&mut self, message: &frond_core::Message) -> ratatui::text::Text<'static> {
        if let Some((tree_id, branch_id)) = self.current_cache_context() {
            let cache_key = (tree_id, branch_id, message.id());
            
            if let Some(cached_text) = self.highlight_cache.get(&cache_key) {
                return cached_text.clone();
            }
            
            let highlighted_text = crate::ui::normal_mode::highlighting::highlight_markdown(message.content());
            self.highlight_cache.insert(cache_key, highlighted_text.clone());
            highlighted_text
        } else {
            crate::ui::normal_mode::highlighting::highlight_markdown(message.content())
        }
    }

    pub fn get_cached_highlighted_text(&self, message_id: Uuid) -> Option<ratatui::text::Text<'static>> {
        let (tree_id, branch_id) = self.current_cache_context()?;
        self.highlight_cache.get(&(tree_id, branch_id, message_id)).cloned()
    }

    pub fn has_cached_highlight(&self, message_id: Uuid) -> bool {
        if let Some((tree_id, branch_id)) = self.current_cache_context() {
            self.highlight_cache.contains_key(&(tree_id, branch_id, message_id))
        } else {
            false
        }
    }

    pub fn cache_highlighted_text(&mut self, message_id: Uuid, text: ratatui::text::Text<'static>) {
        if let Some((tree_id, branch_id)) = self.current_cache_context() {
            self.highlight_cache.insert((tree_id, branch_id, message_id), text);
        }
    }

    pub fn invalidate_message_highlight(&mut self, message_id: Uuid) {
        if let Some((tree_id, branch_id)) = self.current_cache_context() {
            self.highlight_cache.remove(&(tree_id, branch_id, message_id));
        }
    }

    pub fn clear_highlight_cache(&mut self) {
        self.highlight_cache.clear();
    }

    pub fn get_cached_height(&self, message_id: Uuid, viewport_width: u16) -> Option<usize> {
        let (tree_id, branch_id) = self.current_cache_context()?;
        self.height_cache.get(&(tree_id, branch_id, message_id, viewport_width)).copied()
    }

    pub fn cache_height(&mut self, message_id: Uuid, viewport_width: u16, height: usize) {
        if let Some((tree_id, branch_id)) = self.current_cache_context() {
            self.height_cache.insert((tree_id, branch_id, message_id, viewport_width), height);
        }
    }

    pub fn invalidate_message_height(&mut self, message_id: Uuid) {
        if let Some((tree_id, branch_id)) = self.current_cache_context() {
            self.height_cache.retain(|(t_id, b_id, m_id, _), _| 
                !(*t_id == tree_id && *b_id == branch_id && *m_id == message_id));
        }
    }

    pub fn clear_height_cache(&mut self) {
        self.height_cache.clear();
    }

    pub fn populate_caches_for_current_branch(&mut self, viewport_width: u16) {
        let message_data: Vec<(uuid::Uuid, String)> = self.current_messages()
            .iter()
            .map(|msg| (msg.id(), msg.content().to_string()))
            .collect();

        for (message_id, content) in message_data {
            if !self.has_cached_highlight(message_id) {
                let highlighted_text = crate::ui::normal_mode::highlighting::highlight_markdown(&content);
                self.cache_highlighted_text(message_id, highlighted_text);
            }

            if self.get_cached_height(message_id, viewport_width).is_none() {
                let text = self.get_cached_highlighted_text(message_id)
                    .unwrap_or_else(|| ratatui::text::Text::raw(content));
                
                let block = ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL);
                let paragraph = ratatui::widgets::Paragraph::new(text)
                    .block(block)
                    .wrap(ratatui::widgets::Wrap { trim: false });
                
                let height = paragraph.line_count(viewport_width);
                self.cache_height(message_id, viewport_width, height);
            }
        }
    }

    pub fn is_append_mode(&self) -> bool {
        let Some(focused_message_id) = self.focused_message_id else {
            return false;
        };

        let Some(branch) = self.current_branch() else {
            return false;
        };

        let Some(message) = branch.get_message_by_id(focused_message_id) else {
            return false;
        };

        branch.messages().iter().last().map(|last| last.id()) == Some(message.id())
            && *message.role() == frond_core::Role::User
    }
}
