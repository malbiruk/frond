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

    pub(crate) fn fork_branch_from_message(
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
