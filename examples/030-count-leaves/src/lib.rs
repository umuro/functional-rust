#![allow(clippy::all)]
// Count leaf nodes — Node(_, Leaf, Leaf) — of a binary tree (OCaml 99 Problems #30).
#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf,
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}

impl<T> Tree<T> {
    pub fn leaf() -> Self {
        Tree::Leaf
    }

    pub fn node(val: T, left: Tree<T>, right: Tree<T>) -> Self {
        Tree::Node(val, Box::new(left), Box::new(right))
    }
}

pub fn count_leaves<T>(tree: &Tree<T>) -> usize {
    match tree {
        Tree::Leaf => 0,
        Tree::Node(_, l, r) if matches!(**l, Tree::Leaf) && matches!(**r, Tree::Leaf) => 1,
        Tree::Node(_, l, r) => count_leaves(l) + count_leaves(r),
    }
}

pub fn count_nodes<T>(tree: &Tree<T>) -> usize {
    match tree {
        Tree::Leaf => 0,
        Tree::Node(_, l, r) => 1 + count_nodes(l) + count_nodes(r),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Tree<i32> {
        Tree::node(
            1,
            Tree::node(2, Tree::leaf(), Tree::leaf()),
            Tree::node(3, Tree::node(4, Tree::leaf(), Tree::leaf()), Tree::leaf()),
        )
    }

    #[test]
    fn test_count_leaves() {
        assert_eq!(count_leaves(&sample()), 2);
    }

    #[test]
    fn test_single_node_is_one_leaf() {
        let single = Tree::node(1, Tree::leaf(), Tree::leaf());
        assert_eq!(count_leaves(&single), 1);
    }

    #[test]
    fn test_empty_tree_has_no_leaves() {
        assert_eq!(count_leaves::<i32>(&Tree::leaf()), 0);
    }

    #[test]
    fn test_leaves_plus_internal_equals_total() {
        let internal = 2; // nodes {1, 3}
        assert_eq!(count_leaves(&sample()) + internal, count_nodes(&sample()));
    }
}
