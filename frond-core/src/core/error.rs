use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum BranchError {
    #[error("Message with id {0} not found in branch")]
    MessageNotFound(Uuid),
}

#[derive(Debug, Error)]
pub enum TreeError {
    #[error("Branch with id {0} not found in tree")]
    BranchNotFound(Uuid),
    #[error(transparent)]
    Branch(#[from] BranchError),
}

#[derive(Debug, Error)]
pub enum DialogueError {
    #[error("Tree with id {0} not found in dialogue")]
    TreeNotFound(Uuid),
    #[error(transparent)]
    Tree(#[from] TreeError),
}
