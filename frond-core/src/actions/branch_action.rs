use crate::core::dialogue::Dialogue;
use crate::core::error::DialogueError;
use crate::core::message::Message;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum BranchAction {
    // Branch metadata actions
    RenameBranch {
        branch_id: Uuid,
        old_name: String,
        new_name: String,
    },
    SetBranchDescription {
        branch_id: Uuid,
        old_description: Option<String>,
        new_description: String,
    },
    ClearBranchDescription {
        branch_id: Uuid,
        old_description: String,
    },

    // Message management actions (branch manages its messages)
    AppendMessage {
        branch_id: Uuid,
        message_content: String,
    },
    InsertMessageAtIndex {
        branch_id: Uuid,
        message_index: usize,
        message: Message,
    },
    RemoveMessageById {
        branch_id: Uuid,
        message_id: Uuid,
        message_index: usize,
        removed_message: Message,
    },
    ClearMessages {
        branch_id: Uuid,
        removed_messages: Vec<Message>,
    },
}

impl Dialogue {
    // Apply methods for BranchAction
    pub(crate) fn apply_branch_action(
        &mut self,
        action: &BranchAction,
    ) -> Result<(), DialogueError> {
        match action {
            BranchAction::RenameBranch {
                branch_id,
                new_name,
                ..
            } => self.rename_branch(*branch_id, new_name.clone()),
            BranchAction::SetBranchDescription {
                branch_id,
                new_description,
                ..
            } => self.set_branch_description(*branch_id, new_description.clone()),
            BranchAction::ClearBranchDescription { branch_id, .. } => {
                self.clear_branch_description(*branch_id)
            }
            BranchAction::AppendMessage {
                branch_id,
                message_content,
            } => self.append_message(*branch_id, message_content.clone()),
            BranchAction::InsertMessageAtIndex {
                branch_id,
                message_index,
                message,
            } => self.insert_message_at_index(*branch_id, *message_index, message.clone()),
            BranchAction::RemoveMessageById {
                branch_id,
                message_id,
                ..
            } => self.remove_message_by_id(*branch_id, *message_id),
            BranchAction::ClearMessages { branch_id, .. } => {
                self.clear_messages_from_branch(*branch_id)
            }
        }
    }

    // Undo methods for BranchAction
    pub(crate) fn undo_branch_action(
        &mut self,
        action: &BranchAction,
    ) -> Result<(), DialogueError> {
        match action {
            BranchAction::RenameBranch {
                branch_id,
                old_name,
                ..
            } => self.rename_branch(*branch_id, old_name.clone()),
            BranchAction::SetBranchDescription {
                branch_id,
                old_description,
                ..
            } => self.undo_set_branch_description(*branch_id, old_description.clone()),
            BranchAction::ClearBranchDescription {
                branch_id,
                old_description,
            } => self.undo_clear_branch_description(*branch_id, old_description.clone()),
            BranchAction::AppendMessage { branch_id, .. } => self.undo_append_message(*branch_id),
            BranchAction::InsertMessageAtIndex {
                branch_id, message, ..
            } => self.undo_insert_message_at_index(*branch_id, message.clone()),
            BranchAction::RemoveMessageById {
                branch_id,
                message_index,
                removed_message,
                ..
            } => {
                self.undo_remove_message_by_id(*branch_id, *message_index, removed_message.clone())
            }
            BranchAction::ClearMessages {
                branch_id,
                removed_messages,
            } => self.undo_clear_messages_from_branch(*branch_id, removed_messages.clone()),
        }
    }

    // Concrete implementations
    fn rename_branch(&mut self, branch_id: Uuid, new_name: String) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        branch.rename(new_name);
        Ok(())
    }

    fn set_branch_description(
        &mut self,
        branch_id: Uuid,
        new_description: String,
    ) -> Result<(), DialogueError> {
        if let Some(branch) = self.get_branch_by_id_mut(branch_id) {
            branch.set_description(new_description);
            Ok(())
        } else {
            Err(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))
        }
    }

    fn clear_branch_description(&mut self, branch_id: Uuid) -> Result<(), DialogueError> {
        if let Some(branch) = self.get_branch_by_id_mut(branch_id) {
            branch.clear_description();
            Ok(())
        } else {
            Err(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))
        }
    }

    fn append_message(
        &mut self,
        branch_id: Uuid,
        message_content: String,
    ) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        let message = Message::new(message_content, crate::core::message::Role::User);
        branch.add_message(message);
        Ok(())
    }

    fn insert_message_at_index(
        &mut self,
        branch_id: Uuid,
        message_index: usize,
        message: Message,
    ) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        branch.insert_message_at_index(message_index, message);
        Ok(())
    }

    fn remove_message_by_id(
        &mut self,
        branch_id: Uuid,
        message_id: Uuid,
    ) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        branch.remove_message_by_id(message_id);
        Ok(())
    }

    fn clear_messages_from_branch(&mut self, branch_id: Uuid) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        branch.clear();
        Ok(())
    }

    fn undo_append_message(&mut self, branch_id: Uuid) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        // Remove the last message (which should be the one we just added)
        if !branch.messages().is_empty() {
            if let Some(last_message) = branch.messages().get(branch.messages().len() - 1) {
                let last_id = last_message.id();
                branch.remove_message_by_id(last_id);
            }
        }
        Ok(())
    }

    fn undo_insert_message_at_index(
        &mut self,
        branch_id: Uuid,
        message: Message,
    ) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        // Find and remove the message that was inserted
        branch.remove_message_by_id(message.id());
        Ok(())
    }

    fn undo_remove_message_by_id(
        &mut self,
        branch_id: Uuid,
        message_index: usize,
        removed_message: Message,
    ) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        // Re-insert the message at its original position
        branch.insert_message_at_index(message_index, removed_message);
        Ok(())
    }

    fn undo_clear_messages_from_branch(
        &mut self,
        branch_id: Uuid,
        removed_messages: Vec<Message>,
    ) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        // Re-add all the messages that were cleared
        for message in removed_messages {
            branch.add_message(message);
        }
        Ok(())
    }

    fn undo_set_branch_description(
        &mut self,
        branch_id: Uuid,
        old_description: Option<String>,
    ) -> Result<(), DialogueError> {
        if let Some(branch) = self.get_branch_by_id_mut(branch_id) {
            if let Some(old_desc) = old_description {
                branch.set_description(old_desc);
            } else {
                branch.clear_description();
            }
            Ok(())
        } else {
            Err(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))
        }
    }

    fn undo_clear_branch_description(
        &mut self,
        branch_id: Uuid,
        old_description: String,
    ) -> Result<(), DialogueError> {
        if let Some(branch) = self.get_branch_by_id_mut(branch_id) {
            branch.set_description(old_description);
            Ok(())
        } else {
            Err(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))
        }
    }
}
