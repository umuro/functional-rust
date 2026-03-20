#![allow(clippy::all)]
//! Binary Tree — Size, Membership, Traversal
//! See example.ml for OCaml reference

#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf,
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}

impl<T: PartialEq> Tree<T> {
    /// Number of nodes in the tree.
    pub fn size(&self) -> usize {
        match self {
            Tree::Leaf => 0,
            Tree::Node(_, l, r) => 1 + l.size() + r.size(),
        }
    }

    /// Height of the tree (number of edges on longest root-to-leaf path).
    pub fn depth(&self) -> usize {
        match self {
            Tree::Leaf => 0,
            Tree::Node(_, l, r) => 1 + l.depth().max(r.depth()),
        }
    }

    /// Returns true if `x` is stored anywhere in the tree.
    pub fn mem(&self, x: &T) -> bool {
        match self {
            Tree::Leaf => false,
            Tree::Node(v, l, r) => v == x || l.mem(x) || r.mem(x),
        }
    }
}

/// Preorder traversal (root, left, right) using a mutable accumulator — linear time.
/// Mirrors OCaml's `let rec go acc = function | Leaf -> acc | Node(v,l,r) -> v :: go (go acc r) l`.
pub fn preorder<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    fn go<T: Clone>(tree: &Tree<T>, acc: &mut Vec<T>) {
        if let Tree::Node(v, l, r) = tree {
            acc.push(v.clone());
            go(l, acc);
            go(r, acc);
        }
    }
    let mut result = Vec::new();
    go(tree, &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use Tree::{Leaf, Node};

    //      4
    //     / \
    //    2   5
    //   / \
    //  1   3
    fn sample() -> Tree<i32> {
        Node(
            4,
            Box::new(Node(
                2,
                Box::new(Node(1, Box::new(Leaf), Box::new(Leaf))),
                Box::new(Node(3, Box::new(Leaf), Box::new(Leaf))),
            )),
            Box::new(Node(5, Box::new(Leaf), Box::new(Leaf))),
        )
    }

    #[test]
    fn test_size() {
        assert_eq!(sample().size(), 5);
        assert_eq!(Leaf::<i32>.size(), 0);
    }

    #[test]
    fn test_depth() {
        assert_eq!(sample().depth(), 3);
        assert_eq!(Leaf::<i32>.depth(), 0);
    }

    #[test]
    fn test_mem() {
        let t = sample();
        assert!(t.mem(&3));
        assert!(!t.mem(&99));
        assert!(!Leaf::<i32>.mem(&1));
    }

    #[test]
    fn test_preorder() {
        assert_eq!(preorder(&sample()), vec![4, 2, 1, 3, 5]);
        assert_eq!(preorder(&Leaf::<i32>), Vec::<i32>::new());
    }

    #[test]
    fn test_single_node() {
        let t: Tree<i32> = Node(42, Box::new(Leaf), Box::new(Leaf));
        assert_eq!(t.size(), 1);
        assert_eq!(t.depth(), 1);
        assert!(t.mem(&42));
        assert_eq!(preorder(&t), vec![42]);
    }
}
