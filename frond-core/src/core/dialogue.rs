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
