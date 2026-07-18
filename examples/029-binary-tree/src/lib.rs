#![allow(clippy::all)]
// Binary tree ADT (OCaml 99 Problems #29): the central recursive type for problems 29-40.
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

pub fn size<T>(tree: &Tree<T>) -> usize {
    match tree {
        Tree::Leaf => 0,
        Tree::Node(_, l, r) => 1 + size(l) + size(r),
    }
}

pub fn depth<T>(tree: &Tree<T>) -> usize {
    match tree {
        Tree::Leaf => 0,
        Tree::Node(_, l, r) => 1 + depth(l).max(depth(r)),
    }
}

pub fn mem<T: PartialEq>(tree: &Tree<T>, x: &T) -> bool {
    match tree {
        Tree::Leaf => false,
        Tree::Node(v, l, r) => v == x || mem(l, x) || mem(r, x),
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
    fn test_size() {
        assert_eq!(size(&sample()), 4);
        assert_eq!(size::<i32>(&Tree::leaf()), 0);
    }

    #[test]
    fn test_depth() {
        assert_eq!(depth(&sample()), 3);
        assert_eq!(depth::<i32>(&Tree::leaf()), 0);
    }

    #[test]
    fn test_mem() {
        assert!(mem(&sample(), &4));
        assert!(mem(&sample(), &1));
        assert!(!mem(&sample(), &5));
    }
}
