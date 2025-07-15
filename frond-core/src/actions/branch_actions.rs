use crate::core::dialogue::Dialogue;
use crate::core::error::DialogueError;
use crate::core::message::{Message, Role};
use uuid::Uuid;

impl Dialogue {
    pub(crate) fn append_message(
        &mut self,
        branch_id: Uuid,
        message_content: String,
    ) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        let message = Message::new(message_content, Role::User);
        branch.add_message(message);
        Ok(())
    }

    pub(crate) fn undo_append_message(&mut self, branch_id: Uuid) -> Result<(), DialogueError> {
        let branch = self
            .get_branch_by_id_mut(branch_id)
            .ok_or(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        // Remove the last message (which should be the one we just added)
        if !branch.messages().is_empty() {
            // Remove the last message by getting its ID and using the generated method
            if let Some(last_message) = branch.messages().get(branch.messages().len() - 1) {
                let last_id = last_message.id();
                branch.remove_message_by_id(last_id);
            }
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
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))?;

        branch.rename(new_name);
        Ok(())
    }
}
