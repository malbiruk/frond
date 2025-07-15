use crate::core::branch::Branch;
use crate::core::message::Message;
use crate::core::tree::Tree;
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

    // Tree management actions
    AddTree {
        tree_id: Uuid,
        tree_name: String,
    },
    RemoveTree {
        tree_id: Uuid,
        removed_tree: Tree,
    },
    ClearTrees {
        removed_trees: Vec<Tree>,
    },
    SetTreeDescription {
        tree_id: Uuid,
        old_description: Option<String>,
        new_description: String,
    },
    ClearTreeDescription {
        tree_id: Uuid,
        old_description: String,
    },

    // Branch management actions
    AddBranch {
        tree_id: Uuid,
        branch_id: Uuid,
        branch_name: String,
    },
    RemoveBranch {
        tree_id: Uuid,
        branch_id: Uuid,
        removed_branch: Branch,
    },
    ClearBranches {
        tree_id: Uuid,
        removed_branches: Vec<Branch>,
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

    // Message management actions
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
    HideMessage {
        message_id: Uuid,
    },
    ShowMessage {
        message_id: Uuid,
    },

    // Dialogue actions
    RenameDialogue {
        old_name: String,
        new_name: String,
    },
    SetDialogueDescription {
        old_description: Option<String>,
        new_description: String,
    },
    ClearDialogueDescription {
        old_description: String,
    },
    ArchiveDialogue {
        was_archived: bool,
    },
    UnarchiveDialogue {
        was_archived: bool,
    },
    TrashDialogue {
        was_trashed: bool,
    },
    RestoreDialogue {
        was_trashed: bool,
    },
    AddTag {
        tag: String,
    },
    RemoveTag {
        tag: String,
    },
    ClearTags {
        removed_tags: Vec<String>,
    },
}
