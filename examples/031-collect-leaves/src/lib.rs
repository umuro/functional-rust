#![allow(clippy::all)]
// Collect the values of leaf nodes — Node(x, Leaf, Leaf) — left to right (OCaml 99 Problems #31).
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

pub fn collect_leaves<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    match tree {
        Tree::Leaf => vec![],
        Tree::Node(v, l, r) if matches!(**l, Tree::Leaf) && matches!(**r, Tree::Leaf) => {
            vec![v.clone()]
        }
        Tree::Node(_, l, r) => {
            let mut left_leaves = collect_leaves(l);
            left_leaves.extend(collect_leaves(r));
            left_leaves
        }
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
    fn test_collect_leaves_left_to_right() {
        assert_eq!(collect_leaves(&sample()), vec![2, 4]);
    }

    #[test]
    fn test_single_node_is_its_own_leaf() {
        assert_eq!(collect_leaves(&Tree::node(9, Tree::leaf(), Tree::leaf())), vec![9]);
    }

    #[test]
    fn test_empty_tree_has_no_leaves() {
        let empty: Vec<i32> = vec![];
        assert_eq!(collect_leaves::<i32>(&Tree::leaf()), empty);
    }
}
