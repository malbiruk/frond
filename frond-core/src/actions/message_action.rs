use crate::core::dialogue::Dialogue;
use crate::core::error::DialogueError;
use crate::core::message::Message;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum MessageAction {
    // Message content actions
    EditMessage {
        message_id: Uuid,
        old_content: String,
        new_content: String,
    },
    DeleteMessage {
        message_id: Uuid,
        branch_id: Uuid,
        message_index: usize,
        deleted_message: Message,
    },

    // Message state actions
    HideMessage {
        message_id: Uuid,
    },
    ShowMessage {
        message_id: Uuid,
    },
    ToggleMessageRole {
        message_id: Uuid,
    },
}

impl Dialogue {
    // Apply methods for MessageAction
    pub(crate) fn apply_message_action(
        &mut self,
        action: &MessageAction,
    ) -> Result<(), DialogueError> {
        match action {
            MessageAction::EditMessage {
                message_id,
                new_content,
                ..
            } => self.edit_message(*message_id, new_content.clone()),
            MessageAction::DeleteMessage {
                message_id,
                branch_id,
                message_index,
                ..
            } => self.delete_message(*message_id, *branch_id, *message_index),
            MessageAction::HideMessage { message_id } => self.hide_message(*message_id),
            MessageAction::ShowMessage { message_id } => self.show_message(*message_id),
            MessageAction::ToggleMessageRole { message_id } => {
                self.toggle_message_role(*message_id)
            }
        }
    }

    // Undo methods for MessageAction
    pub(crate) fn undo_message_action(
        &mut self,
        action: &MessageAction,
    ) -> Result<(), DialogueError> {
        match action {
            MessageAction::EditMessage {
                message_id,
                old_content,
                ..
            } => self.edit_message(*message_id, old_content.clone()),
            MessageAction::DeleteMessage {
                branch_id,
                message_index,
                deleted_message,
                ..
            } => self.undo_delete_message(*branch_id, *message_index, deleted_message.clone()),
            MessageAction::HideMessage { message_id } => self.undo_hide_message(*message_id),
            MessageAction::ShowMessage { message_id } => self.undo_show_message(*message_id),
            MessageAction::ToggleMessageRole { message_id } => {
                self.toggle_message_role(*message_id)
            }
        }
    }

    // Concrete implementations
    fn edit_message(&mut self, message_id: Uuid, new_content: String) -> Result<(), DialogueError> {
        let message = self
            .get_message_by_id_mut(message_id)
            .ok_or(DialogueError::Tree(crate::core::error::TreeError::Branch(
                crate::core::error::BranchError::MessageNotFound(message_id),
            )))?;

        message.edit_content(new_content);
        Ok(())
    }

    fn delete_message(
        &mut self,
        message_id: Uuid,
        branch_id: Uuid,
        message_index: usize,
    ) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        // Validate that the message exists at the expected index
        if let Some(message) = branch.messages().get(message_index) {
            if message.id() == message_id {
                branch.remove_message_by_id(message_id);
                Ok(())
            } else {
                Err(DialogueError::Tree(crate::core::error::TreeError::Branch(
                    crate::core::error::BranchError::MessageNotFound(message_id),
                )))
            }
        } else {
            Err(DialogueError::Tree(crate::core::error::TreeError::Branch(
                crate::core::error::BranchError::MessageNotFound(message_id),
            )))
        }
    }

    fn hide_message(&mut self, message_id: Uuid) -> Result<(), DialogueError> {
        let message = self
            .get_message_by_id_mut(message_id)
            .ok_or(DialogueError::Tree(crate::core::error::TreeError::Branch(
                crate::core::error::BranchError::MessageNotFound(message_id),
            )))?;

        message.hide();
        Ok(())
    }

    fn show_message(&mut self, message_id: Uuid) -> Result<(), DialogueError> {
        let message = self
            .get_message_by_id_mut(message_id)
            .ok_or(DialogueError::Tree(crate::core::error::TreeError::Branch(
                crate::core::error::BranchError::MessageNotFound(message_id),
            )))?;

        message.show();
        Ok(())
    }

    fn toggle_message_role(&mut self, message_id: Uuid) -> Result<(), DialogueError> {
        let message = self
            .get_message_by_id_mut(message_id)
            .ok_or(DialogueError::Tree(crate::core::error::TreeError::Branch(
                crate::core::error::BranchError::MessageNotFound(message_id),
            )))?;

        message.toggle_role();
        Ok(())
    }

    fn undo_delete_message(
        &mut self,
        branch_id: Uuid,
        message_index: usize,
        deleted_message: Message,
    ) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        // Re-insert the message at its original position
        branch.insert_message_at_index(message_index, deleted_message);
        Ok(())
    }

    fn undo_hide_message(&mut self, message_id: Uuid) -> Result<(), DialogueError> {
        let message = self
            .get_message_by_id_mut(message_id)
            .ok_or(DialogueError::Tree(crate::core::error::TreeError::Branch(
                crate::core::error::BranchError::MessageNotFound(message_id),
            )))?;

        message.show();
        Ok(())
    }

    fn undo_show_message(&mut self, message_id: Uuid) -> Result<(), DialogueError> {
        let message = self
            .get_message_by_id_mut(message_id)
            .ok_or(DialogueError::Tree(crate::core::error::TreeError::Branch(
                crate::core::error::BranchError::MessageNotFound(message_id),
            )))?;

        message.hide();
        Ok(())
    }
}
