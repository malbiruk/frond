use crate::core::dialogue::Dialogue;
use crate::core::error::{BranchError, DialogueError, TreeError};
use crate::core::message::{Message, Role};
use uuid::Uuid;

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

    // Message management actions
    pub(crate) fn insert_message_at_index(
        &mut self,
        branch_id: Uuid,
        message_index: usize,
        message: Message,
    ) -> Result<(), DialogueError> {
        if let Some(branch) = self.get_branch_by_id_mut(branch_id) {
            branch.insert_message_at_index(message_index, message);
            Ok(())
        } else {
            Err(DialogueError::Tree(TreeError::BranchNotFound(branch_id)))
        }
    }

    pub(crate) fn remove_message_by_id(
        &mut self,
        branch_id: Uuid,
        message_id: Uuid,
    ) -> Result<(), DialogueError> {
        if let Some(branch) = self.get_branch_by_id_mut(branch_id) {
            branch.remove_message_by_id(message_id);
            Ok(())
        } else {
            Err(DialogueError::Tree(TreeError::BranchNotFound(branch_id)))
        }
    }

    pub(crate) fn clear_messages_from_branch(
        &mut self,
        branch_id: Uuid,
    ) -> Result<(), DialogueError> {
        if let Some(branch) = self.get_branch_by_id_mut(branch_id) {
            branch.clear();
            Ok(())
        } else {
            Err(DialogueError::Tree(TreeError::BranchNotFound(branch_id)))
        }
    }

    pub(crate) fn hide_message(&mut self, message_id: Uuid) -> Result<(), DialogueError> {
        if let Some(message) = self.get_message_by_id_mut(message_id) {
            message.hide();
            Ok(())
        } else {
            Err(DialogueError::Tree(TreeError::Branch(
                BranchError::MessageNotFound(message_id),
            )))
        }
    }

    pub(crate) fn show_message(&mut self, message_id: Uuid) -> Result<(), DialogueError> {
        if let Some(message) = self.get_message_by_id_mut(message_id) {
            message.show();
            Ok(())
        } else {
            Err(DialogueError::Tree(TreeError::Branch(
                BranchError::MessageNotFound(message_id),
            )))
        }
    }

    pub(crate) fn undo_insert_message_at_index(
        &mut self,
        branch_id: Uuid,
        message: Message,
    ) -> Result<(), DialogueError> {
        if let Some(branch) = self.get_branch_by_id_mut(branch_id) {
            branch.remove_message_by_id(message.id());
            Ok(())
        } else {
            Err(DialogueError::Tree(TreeError::BranchNotFound(branch_id)))
        }
    }

    pub(crate) fn undo_remove_message_by_id(
        &mut self,
        branch_id: Uuid,
        message_index: usize,
        removed_message: Message,
    ) -> Result<(), DialogueError> {
        if let Some(branch) = self.get_branch_by_id_mut(branch_id) {
            branch.insert_message_at_index(message_index, removed_message);
            Ok(())
        } else {
            Err(DialogueError::Tree(TreeError::BranchNotFound(branch_id)))
        }
    }

    pub(crate) fn undo_clear_messages_from_branch(
        &mut self,
        branch_id: Uuid,
        removed_messages: Vec<Message>,
    ) -> Result<(), DialogueError> {
        if let Some(branch) = self.get_branch_by_id_mut(branch_id) {
            branch.add_messages(removed_messages);
            Ok(())
        } else {
            Err(DialogueError::Tree(TreeError::BranchNotFound(branch_id)))
        }
    }

    pub(crate) fn undo_hide_message(&mut self, message_id: Uuid) -> Result<(), DialogueError> {
        if let Some(message) = self.get_message_by_id_mut(message_id) {
            message.show();
            Ok(())
        } else {
            Err(DialogueError::Tree(TreeError::Branch(
                BranchError::MessageNotFound(message_id),
            )))
        }
    }

    pub(crate) fn undo_show_message(&mut self, message_id: Uuid) -> Result<(), DialogueError> {
        if let Some(message) = self.get_message_by_id_mut(message_id) {
            message.hide();
            Ok(())
        } else {
            Err(DialogueError::Tree(TreeError::Branch(
                BranchError::MessageNotFound(message_id),
            )))
        }
    }
}
