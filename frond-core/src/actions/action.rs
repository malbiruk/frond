use super::branch_action::BranchAction;
use super::dialogue_action::DialogueAction;
use super::message_action::MessageAction;
use super::tree_action::TreeAction;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Dialogue(DialogueAction),
    Tree(TreeAction),
    Branch(BranchAction),
    Message(MessageAction),
}
