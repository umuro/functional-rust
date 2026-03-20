#![allow(clippy::all)]
//! Map and Fold on Trees
//! See example.ml for OCaml reference

#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf,
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}

/// Apply `f` to every node value, producing a new tree of the same shape.
/// Mirrors OCaml's `let rec map_tree f = function | Leaf -> Leaf | Node(v,l,r) -> Node(f v, ...)`.
pub fn map_tree<T, U, F: Fn(T) -> U>(tree: Tree<T>, f: &F) -> Tree<U> {
    match tree {
        Tree::Leaf => Tree::Leaf,
        Tree::Node(v, l, r) => Tree::Node(
            f(v),
            Box::new(map_tree(*l, f)),
            Box::new(map_tree(*r, f)),
        ),
    }
}

/// Structural fold: reduce a tree to a single value by combining each node's value
/// with the results of folding its two subtrees.
/// `f(v, left_result, right_result)` — both subtrees fold with the same initial `acc`.
pub fn fold_tree<T, U, F>(tree: Tree<T>, acc: U, f: &F) -> U
where
    F: Fn(T, U, U) -> U,
    U: Clone,
{
    match tree {
        Tree::Leaf => acc,
        Tree::Node(v, l, r) => {
            let l_result = fold_tree(*l, acc.clone(), f);
            let r_result = fold_tree(*r, acc, f);
            f(v, l_result, r_result)
        }
    }
}

pub fn size<T>(tree: Tree<T>) -> usize {
    fold_tree(tree, 0usize, &|_, l, r| 1 + l + r)
}

pub fn depth<T>(tree: Tree<T>) -> usize {
    fold_tree(tree, 0usize, &|_, l, r| 1 + l.max(r))
}

pub fn sum(tree: Tree<i32>) -> i32 {
    fold_tree(tree, 0i32, &|v, l, r| v + l + r)
}

pub fn preorder<T: Clone>(tree: Tree<T>) -> Vec<T> {
    fold_tree(tree, vec![], &|v, l, r| {
        let mut result = vec![v];
        result.extend(l);
        result.extend(r);
        result
    })
}

pub fn inorder<T: Clone>(tree: Tree<T>) -> Vec<T> {
    fold_tree(tree, vec![], &|v, l, r| {
        let mut result = l;
        result.push(v);
        result.extend(r);
        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use Tree::{Leaf, Node};

    //      4
    //     / \
    //    2   6
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
            Box::new(Node(6, Box::new(Leaf), Box::new(Leaf))),
        )
    }

    #[test]
    fn test_size() {
        assert_eq!(size(sample()), 5);
        assert_eq!(size(Leaf::<i32>), 0);
    }

    #[test]
    fn test_depth() {
        assert_eq!(depth(sample()), 3);
        assert_eq!(depth(Leaf::<i32>), 0);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(sample()), 16); // 1+2+3+4+6
        assert_eq!(sum(Leaf), 0);
    }

    #[test]
    fn test_preorder() {
        assert_eq!(preorder(sample()), vec![4, 2, 1, 3, 6]);
    }

    #[test]
    fn test_inorder() {
        assert_eq!(inorder(sample()), vec![1, 2, 3, 4, 6]);
    }

    #[test]
    fn test_map_tree() {
        let doubled = map_tree(sample(), &|v| v * 2);
        assert_eq!(sum(doubled), 32); // 2+4+6+8+12
    }

    #[test]
    fn test_map_preserves_shape() {
        let t = map_tree(sample(), &|v| v.to_string());
        assert_eq!(preorder(t), vec!["4", "2", "1", "3", "6"]);
    }
}
