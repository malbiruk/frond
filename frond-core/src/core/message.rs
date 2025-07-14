use uuid::Uuid;

use super::error::{BranchError, TreeError};
use crate::Dialogue;
use crate::core::error::DialogueError;

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    id: Uuid,
    content: String,
    role: Role,
    is_hidden: bool,
}

impl Message {
    pub fn new(content: impl Into<String>, role: Role) -> Self {
        Message {
            id: Uuid::new_v4(),
            content: content.into(),
            role,
            is_hidden: false,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    pub fn hide(&mut self) {
        self.is_hidden = true;
    }

    pub fn show(&mut self) {
        self.is_hidden = false;
    }

    pub fn edit_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }

    pub fn switch_role(&mut self, role: Role) {
        self.role = role;
    }
}

impl Dialogue {
    pub(crate) fn edit_message(
        &mut self,
        message_id: Uuid,
        new_content: String,
    ) -> Result<(), DialogueError> {
        let message = self
            .get_message_by_id_mut(message_id)
            .ok_or(DialogueError::Tree(TreeError::Branch(
                BranchError::MessageNotFound(message_id),
            )))?;

        message.edit_content(new_content);
        Ok(())
    }

    pub(crate) fn delete_message(
        &mut self,
        message_id: Uuid,
        branch_id: Uuid,
        message_index: usize,
    ) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(TreeError::BranchNotFound(branch_id)))?;

        // Verify the message exists at the expected index
        if message_index < branch.messages().len()
            && branch.messages()[message_index].id() == message_id
        {
            branch.remove_message_by_id(message_id);
            Ok(())
        } else {
            Err(DialogueError::Tree(TreeError::Branch(
                BranchError::MessageNotFound(message_id),
            )))
        }
    }

    pub(crate) fn undo_delete_message(
        &mut self,
        branch_id: Uuid,
        message_index: usize,
        deleted_message: Message,
    ) -> Result<(), DialogueError> {
        // Restore the message to its original location
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(TreeError::BranchNotFound(branch_id)))?;

        branch.insert_message_at_index(message_index, deleted_message);
        Ok(())
    }

    pub(crate) fn toggle_message_role(&mut self, message_id: Uuid) -> Result<(), DialogueError> {
        let message = self
            .get_message_by_id_mut(message_id)
            .ok_or(DialogueError::Tree(TreeError::Branch(
                BranchError::MessageNotFound(message_id),
            )))?;

        // Toggle between User and Assistant roles
        let new_role = match message.role() {
            Role::User => Role::Assistant,
            Role::Assistant => Role::User,
        };

        message.switch_role(new_role);
        Ok(())
    }
}
