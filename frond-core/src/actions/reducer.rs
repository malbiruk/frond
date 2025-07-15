use super::action::Action;
use crate::core::dialogue::Dialogue;
use crate::core::error::DialogueError;

impl Dialogue {
    pub(crate) fn reduce_action_apply(&mut self, action: &Action) -> Result<(), DialogueError> {
        match action {
            Action::Dialogue(dialogue_action) => self.apply_dialogue_action(dialogue_action),
            Action::Tree(tree_action) => self.apply_tree_action(tree_action),
            Action::Branch(branch_action) => self.apply_branch_action(branch_action),
            Action::Message(message_action) => self.apply_message_action(message_action),
        }
    }

    pub(crate) fn reduce_action_undo(&mut self, action: &Action) -> Result<(), DialogueError> {
        match action {
            Action::Dialogue(dialogue_action) => self.undo_dialogue_action(dialogue_action),
            Action::Tree(tree_action) => self.undo_tree_action(tree_action),
            Action::Branch(branch_action) => self.undo_branch_action(branch_action),
            Action::Message(message_action) => self.undo_message_action(message_action),
        }
    }
}
