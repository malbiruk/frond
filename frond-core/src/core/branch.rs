use super::error::BranchError;
use super::message::{Message, Role};
use uuid::Uuid;

use crate::Dialogue;
use crate::core::error::DialogueError;

define_core_entity! {
    pub struct Branch<Message> {
        message, messages
    }
}

impl Branch {
    pub fn fork_from(
        &self,
        message_id: Uuid,
        new_name: impl Into<String>,
    ) -> Result<Branch, BranchError> {
        let idx = self
            .get_message_index_by_id(message_id)
            .ok_or(BranchError::MessageNotFound(message_id))?;
        let messages = self.messages[..=idx].to_vec();
        Ok(Branch::from_messages(new_name, messages))
    }

    pub fn llm_context(&self) -> Vec<&Message> {
        self.messages().iter().filter(|m| !m.is_hidden()).collect()
    }

    pub fn insert_message_at_index(&mut self, index: usize, message: Message) {
        if index <= self.messages.len() {
            self.messages.insert(index, message);
        } else {
            self.messages.push(message);
        }
    }
}

impl Dialogue {
    pub(crate) fn append_message(
        &mut self,
        branch_id: Uuid,
        message_content: String,
    ) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                super::error::TreeError::BranchNotFound(branch_id),
            ))?;

        let message = Message::new(message_content, Role::User);
        branch.add_message(message);
        Ok(())
    }

    pub(crate) fn undo_append_message(&mut self, branch_id: Uuid) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                super::error::TreeError::BranchNotFound(branch_id),
            ))?;

        // Remove the last message (which should be the one we just added)
        if !branch.messages.is_empty() {
            branch.messages.pop();
        }
        Ok(())
    }

    pub(crate) fn rename_branch(
        &mut self,
        branch_id: Uuid,
        new_name: String,
    ) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                super::error::TreeError::BranchNotFound(branch_id),
            ))?;

        branch.rename(new_name);
        Ok(())
    }
}
