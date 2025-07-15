use super::action::Action;
use crate::core::dialogue::Dialogue;
use crate::core::error::DialogueError;

impl Dialogue {
    pub(crate) fn dispatch_action_apply(&mut self, action: &Action) -> Result<(), DialogueError> {
        match action {
            // Dialogue actions
            Action::RenameDialogue { new_name, .. } => self.rename_dialogue(new_name.to_string()),

            Action::SetDialogueDescription {
                old_description: _,
                new_description,
            } => self.set_dialogue_description(new_description.clone()),

            Action::ClearDialogueDescription { old_description: _ } => {
                self.clear_dialogue_description()
            }

            Action::ArchiveDialogue { was_archived: _ } => self.archive_dialogue(),
            Action::UnarchiveDialogue { was_archived: _ } => self.unarchive_dialogue(),
            Action::TrashDialogue { was_trashed: _ } => self.trash_dialogue(),
            Action::RestoreDialogue { was_trashed: _ } => self.restore_dialogue(),
            Action::AddTag { tag } => self.add_dialogue_tag(tag.clone()),
            Action::RemoveTag { tag } => self.remove_dialogue_tag(tag),
            Action::ClearTags { removed_tags: _ } => self.clear_dialogue_tags(),

            // Tree actions
            Action::RenameTree {
                tree_id,
                old_name: _,
                new_name,
            } => self.rename_tree(*tree_id, new_name.to_string()),

            Action::ForkBranch {
                tree_id,
                branch_id,
                from_message_id,
                new_branch_name,
            } => self.fork_branch(
                *tree_id,
                *branch_id,
                *from_message_id,
                new_branch_name.to_string(),
            ),

            // Branch actions
            Action::AppendMessage {
                branch_id,
                message_content,
            } => self.append_message(*branch_id, message_content.clone()),

            Action::RenameBranch {
                branch_id,
                old_name: _,
                new_name,
            } => self.rename_branch(*branch_id, new_name.clone()),

            // Message actions
            Action::EditMessage {
                message_id,
                old_content: _,
                new_content,
            } => self.edit_message(*message_id, new_content.clone()),

            Action::DeleteMessage {
                message_id,
                branch_id,
                message_index,
                deleted_message: _,
            } => self.delete_message(*message_id, *branch_id, *message_index),

            Action::ToggleMessageRole { message_id } => self.toggle_message_role(*message_id),

            // Tree management actions
            Action::AddTree {
                tree_id: _,
                tree_name,
            } => self.add_tree_to_dialogue(tree_name.clone()),

            Action::RemoveTree {
                tree_id,
                removed_tree: _,
            } => self.remove_tree_from_dialogue(*tree_id),

            Action::ClearTrees { removed_trees: _ } => self.clear_trees_from_dialogue(),

            Action::SetTreeDescription {
                tree_id,
                old_description: _,
                new_description,
            } => self.set_tree_description(*tree_id, new_description.clone()),

            Action::ClearTreeDescription {
                tree_id,
                old_description: _,
            } => self.clear_tree_description(*tree_id),

            // Branch management actions
            Action::AddBranch {
                tree_id,
                branch_id: _,
                branch_name,
            } => self.add_branch_to_tree(*tree_id, branch_name.clone()),

            Action::RemoveBranch {
                tree_id,
                branch_id,
                removed_branch: _,
            } => self.remove_branch_from_tree(*tree_id, *branch_id),

            Action::ClearBranches {
                tree_id,
                removed_branches: _,
            } => self.clear_branches_from_tree(*tree_id),

            Action::SetBranchDescription {
                branch_id,
                old_description: _,
                new_description,
            } => self.set_branch_description(*branch_id, new_description.clone()),

            Action::ClearBranchDescription {
                branch_id,
                old_description: _,
            } => self.clear_branch_description(*branch_id),

            // Message management actions
            Action::InsertMessageAtIndex {
                branch_id,
                message_index,
                message,
            } => self.insert_message_at_index(*branch_id, *message_index, message.clone()),

            Action::RemoveMessageById {
                branch_id,
                message_id,
                message_index: _,
                removed_message: _,
            } => self.remove_message_by_id(*branch_id, *message_id),

            Action::ClearMessages {
                branch_id,
                removed_messages: _,
            } => self.clear_messages_from_branch(*branch_id),

            Action::HideMessage { message_id } => self.hide_message(*message_id),
            Action::ShowMessage { message_id } => self.show_message(*message_id),
        }
    }

    pub(crate) fn dispatch_action_undo(&mut self, action: &Action) -> Result<(), DialogueError> {
        match action {
            // Dialogue actions
            Action::RenameDialogue { old_name, .. } => self.rename_dialogue(old_name.to_string()),

            Action::SetDialogueDescription {
                old_description,
                new_description: _,
            } => self.undo_set_dialogue_description(old_description.clone()),

            Action::ClearDialogueDescription { old_description } => {
                self.undo_clear_dialogue_description(old_description.clone())
            }

            Action::ArchiveDialogue { was_archived } => self.undo_archive_dialogue(*was_archived),

            Action::UnarchiveDialogue { was_archived } => {
                self.undo_unarchive_dialogue(*was_archived)
            }

            Action::TrashDialogue { was_trashed } => self.undo_trash_dialogue(*was_trashed),
            Action::RestoreDialogue { was_trashed } => self.undo_restore_dialogue(*was_trashed),
            Action::AddTag { tag } => self.undo_add_dialogue_tag(tag),
            Action::RemoveTag { tag } => self.undo_remove_dialogue_tag(tag.clone()),

            Action::ClearTags { removed_tags } => {
                self.undo_clear_dialogue_tags(removed_tags.clone())
            }

            // Tree actions
            Action::RenameTree {
                tree_id,
                old_name,
                new_name: _,
            } => self.rename_tree(*tree_id, old_name.to_string()),

            Action::ForkBranch {
                tree_id,
                branch_id: _,
                from_message_id: _,
                new_branch_name,
            } => self.undo_fork_branch(*tree_id, new_branch_name.to_string()),

            // Branch actions
            Action::AppendMessage {
                branch_id,
                message_content: _,
            } => self.undo_append_message(*branch_id),

            Action::RenameBranch {
                branch_id,
                old_name,
                new_name: _,
            } => self.rename_branch(*branch_id, old_name.clone()),

            // Message actions
            Action::EditMessage {
                message_id,
                old_content,
                new_content: _,
            } => self.edit_message(*message_id, old_content.clone()),

            Action::DeleteMessage {
                message_id: _,
                branch_id,
                message_index,
                deleted_message,
            } => self.undo_delete_message(*branch_id, *message_index, deleted_message.clone()),

            Action::ToggleMessageRole { message_id } => self.toggle_message_role(*message_id),

            // Tree management actions
            Action::AddTree {
                tree_id,
                tree_name: _,
            } => self.undo_add_tree_to_dialogue(*tree_id),

            Action::RemoveTree {
                tree_id: _,
                removed_tree,
            } => self.undo_remove_tree_from_dialogue(removed_tree.clone()),

            Action::ClearTrees { removed_trees } => {
                self.undo_clear_trees_from_dialogue(removed_trees.clone())
            }
            Action::SetTreeDescription {
                tree_id,
                old_description,
                new_description: _,
            } => self.undo_set_tree_description(*tree_id, old_description.clone()),

            Action::ClearTreeDescription {
                tree_id,
                old_description,
            } => self.undo_clear_tree_description(*tree_id, old_description.clone()),

            // Branch management actions
            Action::AddBranch {
                tree_id,
                branch_id,
                branch_name: _,
            } => self.undo_add_branch_to_tree(*tree_id, *branch_id),

            Action::RemoveBranch {
                tree_id,
                branch_id: _,
                removed_branch,
            } => self.undo_remove_branch_from_tree(*tree_id, removed_branch.clone()),

            Action::ClearBranches {
                tree_id,
                removed_branches,
            } => self.undo_clear_branches_from_tree(*tree_id, removed_branches.clone()),

            Action::SetBranchDescription {
                branch_id,
                old_description,
                new_description: _,
            } => self.undo_set_branch_description(*branch_id, old_description.clone()),

            Action::ClearBranchDescription {
                branch_id,
                old_description,
            } => self.undo_clear_branch_description(*branch_id, old_description.clone()),

            // Message management actions
            Action::InsertMessageAtIndex {
                branch_id,
                message_index: _,
                message,
            } => self.undo_insert_message_at_index(*branch_id, message.clone()),

            Action::RemoveMessageById {
                branch_id,
                message_id: _,
                message_index,
                removed_message,
            } => {
                self.undo_remove_message_by_id(*branch_id, *message_index, removed_message.clone())
            }

            Action::ClearMessages {
                branch_id,
                removed_messages,
            } => self.undo_clear_messages_from_branch(*branch_id, removed_messages.clone()),

            Action::HideMessage { message_id } => self.undo_hide_message(*message_id),
            Action::ShowMessage { message_id } => self.undo_show_message(*message_id),
        }
    }
}
