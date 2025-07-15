use crate::core::tree::Tree;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum DialogueAction {
    // Dialogue metadata actions
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

    // Dialogue state actions
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

    // Tag management actions
    AddTag {
        tag: String,
    },
    RemoveTag {
        tag: String,
    },
    ClearTags {
        removed_tags: Vec<String>,
    },

    // Tree management actions (dialogue manages its trees)
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
}

use crate::core::dialogue::Dialogue;
use crate::core::error::DialogueError;

impl Dialogue {
    pub(crate) fn apply_dialogue_action(
        &mut self,
        action: &DialogueAction,
    ) -> Result<(), DialogueError> {
        match action {
            DialogueAction::RenameDialogue { new_name, .. } => {
                self.rename_dialogue(new_name.clone())
            }
            DialogueAction::SetDialogueDescription {
                new_description, ..
            } => self.set_dialogue_description(new_description.clone()),
            DialogueAction::ClearDialogueDescription { .. } => self.clear_dialogue_description(),
            DialogueAction::ArchiveDialogue { .. } => self.archive_dialogue(),
            DialogueAction::UnarchiveDialogue { .. } => self.unarchive_dialogue(),
            DialogueAction::TrashDialogue { .. } => self.trash_dialogue(),
            DialogueAction::RestoreDialogue { .. } => self.restore_dialogue(),
            DialogueAction::AddTag { tag } => self.add_dialogue_tag(tag.clone()),
            DialogueAction::RemoveTag { tag } => self.remove_dialogue_tag(tag),
            DialogueAction::ClearTags { .. } => self.clear_dialogue_tags(),
            DialogueAction::AddTree { tree_name, .. } => {
                self.add_tree_to_dialogue(tree_name.clone())
            }
            DialogueAction::RemoveTree { tree_id, .. } => self.remove_tree_from_dialogue(*tree_id),
            DialogueAction::ClearTrees { .. } => self.clear_trees_from_dialogue(),
        }
    }

    pub(crate) fn undo_dialogue_action(
        &mut self,
        action: &DialogueAction,
    ) -> Result<(), DialogueError> {
        match action {
            DialogueAction::RenameDialogue { old_name, .. } => {
                self.rename_dialogue(old_name.clone())
            }
            DialogueAction::SetDialogueDescription {
                old_description, ..
            } => self.undo_set_dialogue_description(old_description.clone()),
            DialogueAction::ClearDialogueDescription { old_description } => {
                self.undo_clear_dialogue_description(old_description.clone())
            }
            DialogueAction::ArchiveDialogue { was_archived } => {
                self.undo_archive_dialogue(*was_archived)
            }
            DialogueAction::UnarchiveDialogue { was_archived } => {
                self.undo_unarchive_dialogue(*was_archived)
            }
            DialogueAction::TrashDialogue { was_trashed } => self.undo_trash_dialogue(*was_trashed),
            DialogueAction::RestoreDialogue { was_trashed } => {
                self.undo_restore_dialogue(*was_trashed)
            }
            DialogueAction::AddTag { tag } => self.undo_add_dialogue_tag(tag),
            DialogueAction::RemoveTag { tag } => self.undo_remove_dialogue_tag(tag.clone()),
            DialogueAction::ClearTags { removed_tags } => {
                self.undo_clear_dialogue_tags(removed_tags.clone())
            }
            DialogueAction::AddTree { tree_id, .. } => self.undo_add_tree_to_dialogue(*tree_id),
            DialogueAction::RemoveTree { removed_tree, .. } => {
                self.undo_remove_tree_from_dialogue(removed_tree.clone())
            }
            DialogueAction::ClearTrees { removed_trees } => {
                self.undo_clear_trees_from_dialogue(removed_trees.clone())
            }
        }
    }

    fn rename_dialogue(&mut self, new_name: String) -> Result<(), DialogueError> {
        self.rename(new_name);
        Ok(())
    }

    fn set_dialogue_description(&mut self, new_description: String) -> Result<(), DialogueError> {
        self.set_description(new_description);
        Ok(())
    }

    fn clear_dialogue_description(&mut self) -> Result<(), DialogueError> {
        self.clear_description();
        Ok(())
    }

    fn archive_dialogue(&mut self) -> Result<(), DialogueError> {
        self.archive();
        Ok(())
    }

    fn unarchive_dialogue(&mut self) -> Result<(), DialogueError> {
        self.unarchive();
        Ok(())
    }

    fn trash_dialogue(&mut self) -> Result<(), DialogueError> {
        self.trash();
        Ok(())
    }

    fn restore_dialogue(&mut self) -> Result<(), DialogueError> {
        self.restore();
        Ok(())
    }

    fn add_dialogue_tag(&mut self, tag: String) -> Result<(), DialogueError> {
        self.add_tag(tag);
        Ok(())
    }

    fn remove_dialogue_tag(&mut self, tag: &str) -> Result<(), DialogueError> {
        self.remove_tag(tag);
        Ok(())
    }

    fn clear_dialogue_tags(&mut self) -> Result<(), DialogueError> {
        self.clear_tags();
        Ok(())
    }

    // Undo methods
    fn undo_set_dialogue_description(
        &mut self,
        old_description: Option<String>,
    ) -> Result<(), DialogueError> {
        match old_description {
            Some(desc) => self.set_description(desc),
            None => self.clear_description(),
        }
        Ok(())
    }

    fn undo_clear_dialogue_description(
        &mut self,
        old_description: String,
    ) -> Result<(), DialogueError> {
        self.set_description(old_description);
        Ok(())
    }

    fn undo_archive_dialogue(&mut self, was_archived: bool) -> Result<(), DialogueError> {
        if was_archived {
            self.archive();
        } else {
            self.unarchive();
        }
        Ok(())
    }

    fn undo_unarchive_dialogue(&mut self, was_archived: bool) -> Result<(), DialogueError> {
        if was_archived {
            self.archive();
        } else {
            self.unarchive();
        }
        Ok(())
    }

    fn undo_trash_dialogue(&mut self, was_trashed: bool) -> Result<(), DialogueError> {
        if was_trashed {
            self.trash();
        } else {
            self.restore();
        }
        Ok(())
    }

    fn undo_restore_dialogue(&mut self, was_trashed: bool) -> Result<(), DialogueError> {
        if was_trashed {
            self.trash();
        } else {
            self.restore();
        }
        Ok(())
    }

    fn undo_add_dialogue_tag(&mut self, tag: &str) -> Result<(), DialogueError> {
        self.remove_tag(tag);
        Ok(())
    }

    fn undo_remove_dialogue_tag(&mut self, tag: String) -> Result<(), DialogueError> {
        self.add_tag(tag);
        Ok(())
    }

    fn undo_clear_dialogue_tags(&mut self, removed_tags: Vec<String>) -> Result<(), DialogueError> {
        for tag in removed_tags {
            self.add_tag(tag);
        }
        Ok(())
    }

    // Tree management actions
    fn add_tree_to_dialogue(&mut self, tree_name: String) -> Result<(), DialogueError> {
        let tree = crate::core::tree::Tree::new(tree_name);
        self.add_tree(tree);
        Ok(())
    }

    fn remove_tree_from_dialogue(&mut self, tree_id: Uuid) -> Result<(), DialogueError> {
        self.remove_tree_by_id(tree_id);
        Ok(())
    }

    fn clear_trees_from_dialogue(&mut self) -> Result<(), DialogueError> {
        self.clear();
        Ok(())
    }

    fn undo_add_tree_to_dialogue(&mut self, tree_id: Uuid) -> Result<(), DialogueError> {
        self.remove_tree_by_id(tree_id);
        Ok(())
    }

    fn undo_remove_tree_from_dialogue(
        &mut self,
        removed_tree: crate::core::tree::Tree,
    ) -> Result<(), DialogueError> {
        self.add_tree(removed_tree);
        Ok(())
    }

    fn undo_clear_trees_from_dialogue(
        &mut self,
        removed_trees: Vec<crate::core::tree::Tree>,
    ) -> Result<(), DialogueError> {
        self.add_trees(removed_trees);
        Ok(())
    }
}
