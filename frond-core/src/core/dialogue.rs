use super::tree::Tree;

define_core_entity! {
    pub struct Dialogue<Tree> {
        tree, trees
    }
}

impl Dialogue {
    pub fn from_tree(child: Tree) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: child.name().to_string(),
            description: None,
            trees: vec![child],
        }
    }
}
