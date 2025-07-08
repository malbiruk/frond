#[macro_use]
mod macros;

use frond_core::{Dialogue, Tree};

test_core_entity!(Dialogue, tree, trees, Tree, Tree::new("test"));

#[test]
fn creates_dialogue_from_tree() {
    let tree = Tree::new("main");
    let dialogue = Dialogue::from_tree(tree.clone());
    assert_eq!(dialogue.name(), "main");
    assert_eq!(dialogue.trees().len(), 1);
    assert_eq!(dialogue.trees()[0].id(), tree.id());
}
