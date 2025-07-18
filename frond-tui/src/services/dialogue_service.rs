use frond_core::{Action, BranchAction, Dialogue, MessageAction, TreeAction};
use uuid::Uuid;

/// Service layer for all business logic interactions with frond-core
pub struct DialogueService;

impl DialogueService {
    /// Edit a message's content
    pub fn edit_message(
        dialogue: &mut Dialogue,
        message_id: Uuid,
        content: String,
    ) -> Result<(), String> {
        let old_content = dialogue
            .get_message_by_id(message_id)
            .map(|m| m.content().to_string())
            .unwrap_or_default();

        let action = Action::Message(MessageAction::EditMessage {
            message_id,
            old_content,
            new_content: content,
        });

        dialogue
            .apply_action(action)
            .map_err(|e| format!("Failed to edit message: {}", e))
    }

    /// Append a new message to a branch
    pub fn append_message(
        dialogue: &mut Dialogue,
        branch_id: Uuid,
        content: String,
    ) -> Result<(), String> {
        let action = Action::Branch(BranchAction::AppendMessage {
            branch_id,
            message_content: content,
        });

        dialogue
            .apply_action(action)
            .map_err(|e| format!("Failed to append message: {}", e))
    }

    /// Delete a message from a branch
    pub fn delete_message(
        dialogue: &mut Dialogue,
        message_id: Uuid,
        branch_id: Uuid,
    ) -> Result<(), String> {
        // Get message details before deletion - if anything fails, return a descriptive error
        let branch = dialogue
            .get_branch_by_id(branch_id)
            .ok_or_else(|| format!("Branch with id {} not found", branch_id))?;

        let message_index = branch
            .get_message_index_by_id(message_id)
            .ok_or_else(|| format!("Message with id {} not found in branch", message_id))?;

        let deleted_message = branch
            .get_message_by_id(message_id)
            .ok_or_else(|| format!("Message with id {} not found in branch", message_id))?
            .clone();

        let action = Action::Message(MessageAction::DeleteMessage {
            message_id,
            branch_id,
            message_index,
            deleted_message,
        });

        dialogue
            .apply_action(action)
            .map_err(|e| format!("Failed to delete message: {}", e))
    }

    /// Create a new branch by forking from a message
    pub fn fork_branch(
        dialogue: &mut Dialogue,
        tree_id: Uuid,
        source_branch_id: Uuid,
        from_message_id: Uuid,
        new_branch_name: String,
    ) -> Result<(), String> {
        let action = Action::Tree(TreeAction::ForkBranch {
            tree_id,
            branch_id: source_branch_id,
            from_message_id,
            new_branch_name,
        });

        dialogue
            .apply_action(action)
            .map_err(|e| format!("Failed to fork branch: {}", e))
    }

    /// Hide a message from LLM context
    pub fn hide_message(dialogue: &mut Dialogue, message_id: Uuid) -> Result<(), String> {
        let action = Action::Message(MessageAction::HideMessage { message_id });
        dialogue
            .apply_action(action)
            .map_err(|e| format!("Failed to hide message: {}", e))
    }

    /// Show a message in LLM context
    pub fn show_message(dialogue: &mut Dialogue, message_id: Uuid) -> Result<(), String> {
        let action = Action::Message(MessageAction::ShowMessage { message_id });
        dialogue
            .apply_action(action)
            .map_err(|e| format!("Failed to show message: {}", e))
    }

    /// Undo the last action
    pub fn undo(dialogue: &mut Dialogue) -> Result<(), String> {
        dialogue
            .undo_action()
            .map_err(|e| format!("Failed to undo: {}", e))
    }

    /// Redo the last undone action
    pub fn redo(dialogue: &mut Dialogue) -> Result<(), String> {
        dialogue
            .redo_action()
            .map_err(|e| format!("Failed to redo: {}", e))
    }
}
