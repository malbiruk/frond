use crate::core::branch::Branch;
use crate::core::dialogue::Dialogue;
use crate::core::error::DialogueError;
use crate::core::tree::Tree;
use uuid::Uuid;

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

    // Tree management actions
    pub(crate) fn add_tree_to_dialogue(&mut self, tree_name: String) -> Result<(), DialogueError> {
        let tree = Tree::new(tree_name);
        self.add_tree(tree);
        Ok(())
    }

    pub(crate) fn remove_tree_from_dialogue(&mut self, tree_id: Uuid) -> Result<(), DialogueError> {
        self.remove_tree_by_id(tree_id);
        Ok(())
    }

    pub(crate) fn clear_trees_from_dialogue(&mut self) -> Result<(), DialogueError> {
        self.clear();
        Ok(())
    }

    pub(crate) fn set_tree_description(
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

    pub(crate) fn clear_tree_description(&mut self, tree_id: Uuid) -> Result<(), DialogueError> {
        if let Some(tree) = self.get_tree_by_id_mut(tree_id) {
            tree.clear_description();
            Ok(())
        } else {
            Err(DialogueError::TreeNotFound(tree_id))
        }
    }

    pub(crate) fn undo_add_tree_to_dialogue(&mut self, tree_id: Uuid) -> Result<(), DialogueError> {
        self.remove_tree_by_id(tree_id);
        Ok(())
    }

    pub(crate) fn undo_remove_tree_from_dialogue(
        &mut self,
        removed_tree: Tree,
    ) -> Result<(), DialogueError> {
        self.add_tree(removed_tree);
        Ok(())
    }

    pub(crate) fn undo_clear_trees_from_dialogue(
        &mut self,
        removed_trees: Vec<Tree>,
    ) -> Result<(), DialogueError> {
        self.add_trees(removed_trees);
        Ok(())
    }

    pub(crate) fn undo_set_tree_description(
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

    pub(crate) fn undo_clear_tree_description(
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
    pub(crate) fn add_branch_to_tree(
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

    pub(crate) fn remove_branch_from_tree(
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

    pub(crate) fn clear_branches_from_tree(&mut self, tree_id: Uuid) -> Result<(), DialogueError> {
        if let Some(tree) = self.get_tree_by_id_mut(tree_id) {
            tree.clear();
            Ok(())
        } else {
            Err(DialogueError::TreeNotFound(tree_id))
        }
    }

    pub(crate) fn set_branch_description(
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

    pub(crate) fn clear_branch_description(
        &mut self,
        branch_id: Uuid,
    ) -> Result<(), DialogueError> {
        if let Some(branch) = self.get_branch_by_id_mut(branch_id) {
            branch.clear_description();
            Ok(())
        } else {
            Err(DialogueError::Tree(
                crate::core::error::TreeError::BranchNotFound(branch_id),
            ))
        }
    }

    pub(crate) fn undo_add_branch_to_tree(
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

    pub(crate) fn undo_remove_branch_from_tree(
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

    pub(crate) fn undo_clear_branches_from_tree(
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

    pub(crate) fn undo_set_branch_description(
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

    pub(crate) fn undo_clear_branch_description(
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
