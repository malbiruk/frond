use crate::core::branch::Branch;
use crate::core::dialogue::Dialogue;
use crate::core::error::DialogueError;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum TreeAction {
    // Tree metadata actions
    RenameTree {
        tree_id: Uuid,
        old_name: String,
        new_name: String,
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

    // Branch operations (tree manages its branches)
    ForkBranch {
        tree_id: Uuid,
        branch_id: Uuid,
        from_message_id: Uuid,
        new_branch_name: String,
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
}

impl Dialogue {
    // Apply methods for TreeAction
    pub(crate) fn apply_tree_action(&mut self, action: &TreeAction) -> Result<(), DialogueError> {
        match action {
            TreeAction::RenameTree {
                tree_id, new_name, ..
            } => self.rename_tree(*tree_id, new_name.clone()),
            TreeAction::SetTreeDescription {
                tree_id,
                new_description,
                ..
            } => self.set_tree_description(*tree_id, new_description.clone()),
            TreeAction::ClearTreeDescription { tree_id, .. } => {
                self.clear_tree_description(*tree_id)
            }
            TreeAction::ForkBranch {
                tree_id,
                branch_id,
                from_message_id,
                new_branch_name,
            } => self.fork_branch(
                *tree_id,
                *branch_id,
                *from_message_id,
                new_branch_name.clone(),
            ),
            TreeAction::AddBranch {
                tree_id,
                branch_name,
                ..
            } => self.add_branch_to_tree(*tree_id, branch_name.clone()),
            TreeAction::RemoveBranch {
                tree_id, branch_id, ..
            } => self.remove_branch_from_tree(*tree_id, *branch_id),
            TreeAction::ClearBranches { tree_id, .. } => self.clear_branches_from_tree(*tree_id),
        }
    }

    // Undo methods for TreeAction
    pub(crate) fn undo_tree_action(&mut self, action: &TreeAction) -> Result<(), DialogueError> {
        match action {
            TreeAction::RenameTree {
                tree_id, old_name, ..
            } => self.rename_tree(*tree_id, old_name.clone()),
            TreeAction::SetTreeDescription {
                tree_id,
                old_description,
                ..
            } => self.undo_set_tree_description(*tree_id, old_description.clone()),
            TreeAction::ClearTreeDescription {
                tree_id,
                old_description,
            } => self.undo_clear_tree_description(*tree_id, old_description.clone()),
            TreeAction::ForkBranch {
                tree_id,
                new_branch_name,
                ..
            } => self.undo_fork_branch(*tree_id, new_branch_name.clone()),
            TreeAction::AddBranch {
                tree_id, branch_id, ..
            } => self.undo_add_branch_to_tree(*tree_id, *branch_id),
            TreeAction::RemoveBranch {
                tree_id,
                removed_branch,
                ..
            } => self.undo_remove_branch_from_tree(*tree_id, removed_branch.clone()),
            TreeAction::ClearBranches {
                tree_id,
                removed_branches,
            } => self.undo_clear_branches_from_tree(*tree_id, removed_branches.clone()),
        }
    }

    // Concrete implementations
    fn rename_tree(&mut self, tree_id: Uuid, new_name: String) -> Result<(), DialogueError> {
        self.get_tree_by_id_mut(tree_id)
            .ok_or(DialogueError::TreeNotFound(tree_id))?
            .rename(new_name);
        Ok(())
    }

    fn set_tree_description(
        &mut self,
        tree_id: Uuid,
        new_description: String,
    ) -> Result<(), DialogueError> {
        if let Some(tree) = self.get_tree_by_id_mut(tree_id) {
            tree.set_description(new_description);
            Ok(())
        } else {
            Err(DialogueError::TreeNotFound(tree_id))
        }
    }

    fn clear_tree_description(&mut self, tree_id: Uuid) -> Result<(), DialogueError> {
        if let Some(tree) = self.get_tree_by_id_mut(tree_id) {
            tree.clear_description();
            Ok(())
        } else {
            Err(DialogueError::TreeNotFound(tree_id))
        }
    }

    fn fork_branch(
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

    fn undo_fork_branch(
        &mut self,
        tree_id: Uuid,
        branch_name: String,
    ) -> Result<(), DialogueError> {
        let tree = self
            .get_tree_by_id_mut(tree_id)
            .ok_or(DialogueError::TreeNotFound(tree_id))?;

        // Find the branch with the given name and remove it
        let branch_id = tree
            .branches()
            .iter()
            .find(|b| b.name() == branch_name)
            .map(|b| b.id());
        if let Some(branch_id) = branch_id {
            tree.remove_branch_by_id(branch_id);
        }

        Ok(())
    }

    fn undo_set_tree_description(
        &mut self,
        tree_id: Uuid,
        old_description: Option<String>,
    ) -> Result<(), DialogueError> {
        if let Some(tree) = self.get_tree_by_id_mut(tree_id) {
            if let Some(old_desc) = old_description {
                tree.set_description(old_desc);
            } else {
                tree.clear_description();
            }
            Ok(())
        } else {
            Err(DialogueError::TreeNotFound(tree_id))
        }
    }

    fn undo_clear_tree_description(
        &mut self,
        tree_id: Uuid,
        old_description: String,
    ) -> Result<(), DialogueError> {
        if let Some(tree) = self.get_tree_by_id_mut(tree_id) {
            tree.set_description(old_description);
            Ok(())
        } else {
            Err(DialogueError::TreeNotFound(tree_id))
        }
    }

    // Branch management actions
    fn add_branch_to_tree(
        &mut self,
        tree_id: Uuid,
        branch_name: String,
    ) -> Result<(), DialogueError> {
        if let Some(tree) = self.get_tree_by_id_mut(tree_id) {
            let branch = Branch::new(branch_name);
            tree.add_branch(branch);
            Ok(())
        } else {
            Err(DialogueError::TreeNotFound(tree_id))
        }
    }

    fn remove_branch_from_tree(
        &mut self,
        tree_id: Uuid,
        branch_id: Uuid,
    ) -> Result<(), DialogueError> {
        if let Some(tree) = self.get_tree_by_id_mut(tree_id) {
            tree.remove_branch_by_id(branch_id);
            Ok(())
        } else {
            Err(DialogueError::TreeNotFound(tree_id))
        }
    }

    fn clear_branches_from_tree(&mut self, tree_id: Uuid) -> Result<(), DialogueError> {
        if let Some(tree) = self.get_tree_by_id_mut(tree_id) {
            tree.clear();
            Ok(())
        } else {
            Err(DialogueError::TreeNotFound(tree_id))
        }
    }

    fn undo_add_branch_to_tree(
        &mut self,
        tree_id: Uuid,
        branch_id: Uuid,
    ) -> Result<(), DialogueError> {
        if let Some(tree) = self.get_tree_by_id_mut(tree_id) {
            tree.remove_branch_by_id(branch_id);
            Ok(())
        } else {
            Err(DialogueError::TreeNotFound(tree_id))
        }
    }

    fn undo_remove_branch_from_tree(
        &mut self,
        tree_id: Uuid,
        removed_branch: Branch,
    ) -> Result<(), DialogueError> {
        if let Some(tree) = self.get_tree_by_id_mut(tree_id) {
            tree.add_branch(removed_branch);
            Ok(())
        } else {
            Err(DialogueError::TreeNotFound(tree_id))
        }
    }

    fn undo_clear_branches_from_tree(
        &mut self,
        tree_id: Uuid,
        removed_branches: Vec<Branch>,
    ) -> Result<(), DialogueError> {
        if let Some(tree) = self.get_tree_by_id_mut(tree_id) {
            tree.add_branches(removed_branches);
            Ok(())
        } else {
            Err(DialogueError::TreeNotFound(tree_id))
        }
    }
}
