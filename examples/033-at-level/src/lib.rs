#![allow(clippy::all)]
// Collect node values at a given 1-based level (root = level 1) (OCaml 99 Problems #33).
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

pub fn at_level<T: Clone>(tree: &Tree<T>, level: usize) -> Vec<T> {
    match tree {
        Tree::Leaf => vec![],
        Tree::Node(v, _, _) if level == 1 => vec![v.clone()],
        Tree::Node(_, l, r) if level > 1 => {
            let mut result = at_level(l, level - 1);
            result.extend(at_level(r, level - 1));
            result
        }
        Tree::Node(_, _, _) => vec![], // level == 0: no 0-th level in a 1-based scheme
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
    fn test_level_1_is_root() {
        assert_eq!(at_level(&sample(), 1), vec![1]);
    }

    #[test]
    fn test_level_2_is_roots_children() {
        assert_eq!(at_level(&sample(), 2), vec![2, 3]);
    }

    #[test]
    fn test_level_3() {
        assert_eq!(at_level(&sample(), 3), vec![4]);
    }

    #[test]
    fn test_level_beyond_depth_is_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(at_level(&sample(), 4), empty);
    }
}
