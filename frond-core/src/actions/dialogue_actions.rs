use crate::core::dialogue::Dialogue;
use crate::core::error::DialogueError;

impl Dialogue {
    pub(crate) fn rename_dialogue(&mut self, new_name: String) -> Result<(), DialogueError> {
        self.rename(new_name);
        Ok(())
    }

    pub(crate) fn set_dialogue_description(
        &mut self,
        new_description: String,
    ) -> Result<(), DialogueError> {
        self.set_description(new_description);
        Ok(())
    }

    pub(crate) fn clear_dialogue_description(&mut self) -> Result<(), DialogueError> {
        self.clear_description();
        Ok(())
    }

    pub(crate) fn archive_dialogue(&mut self) -> Result<(), DialogueError> {
        self.archive();
        Ok(())
    }

    pub(crate) fn unarchive_dialogue(&mut self) -> Result<(), DialogueError> {
        self.unarchive();
        Ok(())
    }

    pub(crate) fn trash_dialogue(&mut self) -> Result<(), DialogueError> {
        self.trash();
        Ok(())
    }

    pub(crate) fn restore_dialogue(&mut self) -> Result<(), DialogueError> {
        self.restore();
        Ok(())
    }

    pub(crate) fn add_dialogue_tag(&mut self, tag: String) -> Result<(), DialogueError> {
        self.add_tag(tag);
        Ok(())
    }

    pub(crate) fn remove_dialogue_tag(&mut self, tag: &str) -> Result<(), DialogueError> {
        self.remove_tag(tag);
        Ok(())
    }

    pub(crate) fn clear_dialogue_tags(&mut self) -> Result<(), DialogueError> {
        self.clear_tags();
        Ok(())
    }

    // Undo methods
    pub(crate) fn undo_set_dialogue_description(
        &mut self,
        old_description: Option<String>,
    ) -> Result<(), DialogueError> {
        match old_description {
            Some(desc) => self.set_description(desc),
            None => self.clear_description(),
        }
        Ok(())
    }

    pub(crate) fn undo_clear_dialogue_description(
        &mut self,
        old_description: String,
    ) -> Result<(), DialogueError> {
        self.set_description(old_description);
        Ok(())
    }

    pub(crate) fn undo_archive_dialogue(
        &mut self,
        was_archived: bool,
    ) -> Result<(), DialogueError> {
        if was_archived {
            self.archive();
        } else {
            self.unarchive();
        }
        Ok(())
    }

    pub(crate) fn undo_unarchive_dialogue(
        &mut self,
        was_archived: bool,
    ) -> Result<(), DialogueError> {
        if was_archived {
            self.archive();
        } else {
            self.unarchive();
        }
        Ok(())
    }

    pub(crate) fn undo_trash_dialogue(&mut self, was_trashed: bool) -> Result<(), DialogueError> {
        if was_trashed {
            self.trash();
        } else {
            self.restore();
        }
        Ok(())
    }

    pub(crate) fn undo_restore_dialogue(&mut self, was_trashed: bool) -> Result<(), DialogueError> {
        if was_trashed {
            self.trash();
        } else {
            self.restore();
        }
        Ok(())
    }

    pub(crate) fn undo_add_dialogue_tag(&mut self, tag: &str) -> Result<(), DialogueError> {
        self.remove_tag(tag);
        Ok(())
    }

    pub(crate) fn undo_remove_dialogue_tag(&mut self, tag: String) -> Result<(), DialogueError> {
        self.add_tag(tag);
        Ok(())
    }

    pub(crate) fn undo_clear_dialogue_tags(
        &mut self,
        removed_tags: Vec<String>,
    ) -> Result<(), DialogueError> {
        for tag in removed_tags {
            self.add_tag(tag);
        }
        Ok(())
    }
}
