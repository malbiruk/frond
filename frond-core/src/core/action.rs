use super::message::Message;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    // Message actions
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
    ToggleMessageRole {
        message_id: Uuid,
    },

    // Branch actions
    AppendMessage {
        branch_id: Uuid,
        message_content: String,
    },
    RenameBranch {
        branch_id: Uuid,
        old_name: String,
        new_name: String,
    },

    // Tree actions
    RenameTree {
        tree_id: Uuid,
        old_name: String,
        new_name: String,
    },
    ForkBranch {
        tree_id: Uuid,
        branch_id: Uuid,
        from_message_id: Uuid,
        new_branch_name: String,
    },

    // Dialogue actions
    RenameDialogue {
        old_name: String,
        new_name: String,
    },
}
