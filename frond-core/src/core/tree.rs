use crate::Dialogue;
use crate::core::error::DialogueError;

use super::branch::Branch;
use super::error::TreeError;
use uuid::Uuid;

define_core_entity! {
    pub struct Tree<Branch> {
        branch, branches
    }
}

impl Tree {
    pub fn from_branch(child: Branch) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: child.name().to_string(),
            description: None,
            branches: vec![child],
        }
    }

    pub fn fork_branch_from_message(
        &mut self,
        source_branch_id: Uuid,
        message_id: Uuid,
        new_branch_name: impl Into<String>,
    ) -> Result<&Branch, TreeError> {
        let source_branch = self
            .branches
            .iter()
            .find(|b| b.id() == source_branch_id)
            .ok_or(TreeError::BranchNotFound(source_branch_id))?;

        let fork = source_branch
            .fork_from(message_id, new_branch_name)
            .map_err(TreeError::Branch)?;

        self.branches.push(fork);
        Ok(self.branches.last().expect("Just pushed, so must exist"))
    }
}

impl Dialogue {
    pub(crate) fn rename_tree(
        &mut self,
        tree_id: Uuid,
        new_name: String,
    ) -> Result<(), DialogueError> {
        self.get_tree_by_id_mut(tree_id)
            .ok_or(DialogueError::TreeNotFound(tree_id))?
            .rename(new_name);
        Ok(())
    }

    pub(crate) fn fork_branch(
        &mut self,
        tree_id: Uuid,
        branch_id: Uuid,
        from_message_id: Uuid,
        new_branch_name: String,
    ) -> Result<(), DialogueError> {
        self.get_tree_by_id_mut(tree_id)
            .ok_or(DialogueError::TreeNotFound(tree_id))?
            .fork_branch_from_message(branch_id, from_message_id, new_branch_name)?;
        Ok(())
    }

    pub(crate) fn undo_fork_branch(
        &mut self,
        tree_id: Uuid,
        branch_name: String,
    ) -> Result<(), DialogueError> {
        let tree = self
            .get_tree_by_id_mut(tree_id)
            .ok_or(DialogueError::TreeNotFound(tree_id))?;

        // Find the branch with the given name and remove it
        if let Some(branch_index) = tree.branches().iter().position(|b| b.name() == branch_name) {
            tree.branches.remove(branch_index);
        }

        Ok(())
    }
}
