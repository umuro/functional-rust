#![allow(clippy::all)]
// Collect internal node values (any node that is not Node(x, Leaf, Leaf)), preorder (OCaml 99 Problems #32).
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

pub fn collect_internal<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    match tree {
        Tree::Leaf => vec![],
        Tree::Node(_, l, r) if matches!(**l, Tree::Leaf) && matches!(**r, Tree::Leaf) => vec![],
        Tree::Node(v, l, r) => {
            let mut result = vec![v.clone()];
            result.extend(collect_internal(l));
            result.extend(collect_internal(r));
            result
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
    fn test_collect_internal_preorder() {
        assert_eq!(collect_internal(&sample()), vec![1, 3]);
    }

    #[test]
    fn test_single_node_has_no_internal() {
        let empty: Vec<i32> = vec![];
        assert_eq!(collect_internal(&Tree::node(9, Tree::leaf(), Tree::leaf())), empty);
    }

    #[test]
    fn test_empty_tree_has_no_internal() {
        let empty: Vec<i32> = vec![];
        assert_eq!(collect_internal::<i32>(&Tree::leaf()), empty);
    }
}
