#![allow(clippy::all)]
/// Catamorphism — Generalized Fold on ADTs
///
/// Ownership insight: The catamorphism takes closures by reference (&dyn Fn).
/// The tree is borrowed for folding, owned for mirror (which builds new tree).

#[derive(Debug, PartialEq, Clone)]
pub enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T> Tree<T> {
    pub fn node(left: Tree<T>, val: T, right: Tree<T>) -> Self {
        Tree::Node(Box::new(left), val, Box::new(right))
    }
}

/// Catamorphism: replaces constructors with functions
/// leaf_fn replaces Leaf, node_fn replaces Node
pub fn cata<T, R>(tree: &Tree<T>, leaf_val: R, node_fn: &dyn Fn(R, &T, R) -> R) -> R
where
    R: Clone,
{
    match tree {
        Tree::Leaf => leaf_val,
        Tree::Node(l, v, r) => {
            let left = cata(l, leaf_val.clone(), node_fn);
            let right = cata(r, leaf_val.clone(), node_fn);
            node_fn(left, v, right)
        }
    }
}

pub fn size<T>(tree: &Tree<T>) -> usize {
    cata(tree, 0, &|l, _, r| 1 + l + r)
}

pub fn sum(tree: &Tree<i64>) -> i64 {
    cata(tree, 0, &|l, v, r| l + v + r)
}

pub fn height<T>(tree: &Tree<T>) -> usize {
    cata(tree, 0, &|l, _, r| 1 + l.max(r))
}

/// Mirror requires building a new tree — different signature
pub fn mirror<T: Clone>(tree: &Tree<T>) -> Tree<T> {
    match tree {
        Tree::Leaf => Tree::Leaf,
        Tree::Node(l, v, r) => Tree::node(mirror(r), v.clone(), mirror(l)),
    }
}

/// In-order traversal to list
pub fn to_vec<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    match tree {
        Tree::Leaf => vec![],
        Tree::Node(l, v, r) => {
            let mut result = to_vec(l);
            result.push(v.clone());
            result.extend(to_vec(r));
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Tree<i64> {
        Tree::node(
            Tree::node(Tree::Leaf, 1, Tree::Leaf),
            2,
            Tree::node(Tree::Leaf, 3, Tree::Leaf),
        )
    }

    #[test]
    fn test_size() {
        assert_eq!(size(&sample()), 3);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(&sample()), 6);
    }

    #[test]
    fn test_height() {
        assert_eq!(height(&sample()), 2);
    }

    #[test]
    fn test_mirror() {
        let m = mirror(&sample());
        assert_eq!(to_vec(&m), vec![3, 2, 1]);
    }

    #[test]
    fn test_to_vec() {
        assert_eq!(to_vec(&sample()), vec![1, 2, 3]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(size::<i64>(&Tree::Leaf), 0);
        assert_eq!(height::<i64>(&Tree::Leaf), 0);
    }
}
