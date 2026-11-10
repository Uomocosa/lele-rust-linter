use crate::tree;

pub fn total(tree: &tree::Tree) -> u32 {
    fn recursion(tree: &tree::Tree, out: &mut u32) {
        *out = out.saturating_add(tree.value);
        for child in &tree.children {
            recursion(child, out);
        }
    }

    let mut out = 0;
    recursion(tree, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::total;
    use crate::tree::Tree;

    #[test]
    fn test_usage() {
        let leaf = Tree {
            value: 2,
            children: Vec::new(),
        };
        let root = Tree {
            value: 1,
            children: vec![leaf],
        };
        assert_eq!(total(&root), 3);
    }
}
