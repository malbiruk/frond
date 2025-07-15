use super::tree::Tree;
use crate::actions::Action;
use crate::core::error::DialogueError;

define_core_entity! {
    pub struct Dialogue<Tree> {
        tree, trees,
        extra_fields {
            (is_archived: bool, false),
            (is_trashed: bool, false),
            (tags: Vec<String>, Vec::new()),
            (action_stack: Vec<Action>, Vec::new()),
            (action_pos: usize, 0),
        }
    }
}

impl Dialogue {
    // creation methods
    pub fn from_tree(child: Tree) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: child.name().to_string(),
            description: None,
            trees: vec![child],
            is_archived: false,
            is_trashed: false,
            tags: Vec::new(),
            action_stack: Vec::new(),
            action_pos: 0,
        }
    }
}

impl Dialogue {
    // branch helper methods
    pub fn get_branch_by_id(&self, branch_id: uuid::Uuid) -> Option<&super::branch::Branch> {
        for tree in &self.trees {
            if let Some(branch) = tree.get_branch_by_id(branch_id) {
                return Some(branch);
            }
        }
        None
    }

    pub(crate) fn get_branch_by_id_mut(
        &mut self,
        branch_id: uuid::Uuid,
    ) -> Option<&mut super::branch::Branch> {
        for tree in &mut self.trees {
            if let Some(branch) = tree.get_branch_by_id_mut(branch_id) {
                return Some(branch);
            }
        }
        None
    }

    // message helper methods
    pub fn get_message_by_id(&self, message_id: uuid::Uuid) -> Option<&super::message::Message> {
        for tree in &self.trees {
            for branch in tree.branches() {
                if let Some(message) = branch.get_message_by_id(message_id) {
                    return Some(message);
                }
            }
        }
        None
    }

    pub(crate) fn get_message_by_id_mut(
        &mut self,
        message_id: uuid::Uuid,
    ) -> Option<&mut super::message::Message> {
        for tree in &mut self.trees {
            for branch in tree.branches_mut() {
                if let Some(message) = branch.get_message_by_id_mut(message_id) {
                    return Some(message);
                }
            }
        }
        None
    }
}

impl Dialogue {
    // archive methods
    pub fn is_archived(&self) -> bool {
        self.is_archived
    }

    pub(crate) fn archive(&mut self) {
        self.is_archived = true;
    }

    pub(crate) fn unarchive(&mut self) {
        self.is_archived = false;
    }
}

impl Dialogue {
    // trash methods
    pub fn is_trashed(&self) -> bool {
        self.is_trashed
    }

    pub(crate) fn trash(&mut self) {
        self.is_trashed = true;
    }

    pub(crate) fn restore(&mut self) {
        self.is_trashed = false;
    }
}

impl Dialogue {
    // tag methods
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub(crate) fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    pub(crate) fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    pub(crate) fn clear_tags(&mut self) {
        self.tags.clear();
    }
}

// actions interface
impl Dialogue {
    pub fn apply_action(&mut self, action: Action) -> Result<(), DialogueError> {
        self.action_stack.truncate(self.action_pos);
        self.dispatch_action_apply(&action)?;
        self.action_stack.push(action);
        self.action_pos += 1;
        Ok(())
    }

    pub fn undo_action(&mut self) -> Result<(), DialogueError> {
        if self.action_pos > 0 {
            self.action_pos -= 1;
            let action = self.action_stack[self.action_pos].clone();
            self.dispatch_action_undo(&action)?;
        }
        Ok(())
    }

    pub fn redo_action(&mut self) -> Result<(), DialogueError> {
        if self.action_pos < self.action_stack.len() {
            let action = self.action_stack[self.action_pos].clone();
            self.dispatch_action_apply(&action)?;
            self.action_pos += 1;
        }
        Ok(())
    }
}
