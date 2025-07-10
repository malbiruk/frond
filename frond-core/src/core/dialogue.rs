use super::tree::Tree;

define_core_entity! {
    pub struct Dialogue<Tree> {
        tree, trees,
        extra_fields {
            (is_archived: bool, false),
            (is_trashed: bool, false),
            (tags: Vec<String>, Vec::new())
        }
    }
}

impl Dialogue {
    // creation methods
    pub fn from_tree(child: Tree) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: child.name().to_string(),
            description: None,
            trees: vec![child],
            is_archived: false,
            is_trashed: false,
            tags: Vec::new(),
        }
    }
}

impl Dialogue {
    // archive methods
    pub fn is_archived(&self) -> bool {
        self.is_archived
    }

    pub fn archive(&mut self) {
        self.is_archived = true;
    }

    pub fn unarchive(&mut self) {
        self.is_archived = false;
    }
}

impl Dialogue {
    // trash methods
    pub fn is_trashed(&self) -> bool {
        self.is_trashed
    }

    pub fn trash(&mut self) {
        self.is_trashed = true;
    }

    pub fn restore(&mut self) {
        self.is_trashed = false;
    }
}

impl Dialogue {
    // tag methods
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }
}
