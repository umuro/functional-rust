/// Map and Fold on Trees
///
/// Lifting map and fold from lists to binary trees. Once you define
/// `fold_tree`, you can express size, depth, sum, and traversals
/// without any explicit recursion — the fold does it all.

#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf,
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}

impl<T> Tree<T> {
    pub fn node(v: T, l: Tree<T>, r: Tree<T>) -> Self {
        Tree::Node(v, Box::new(l), Box::new(r))
    }
}

/// Map a function over every node value, producing a new tree.
pub fn map_tree<T, U>(tree: &Tree<T>, f: &impl Fn(&T) -> U) -> Tree<U> {
    match tree {
        Tree::Leaf => Tree::Leaf,
        Tree::Node(v, l, r) => Tree::node(f(v), map_tree(l, f), map_tree(r, f)),
    }
}

/// Fold (catamorphism) on a tree. The function `f` receives the node value
/// and the results of folding the left and right subtrees.
pub fn fold_tree<T, A>(tree: &Tree<T>, acc: A, f: &impl Fn(&T, A, A) -> A) -> A
where
    A: Clone,
{
    match tree {
        Tree::Leaf => acc,
        Tree::Node(v, l, r) => {
            let left = fold_tree(l, acc.clone(), f);
            let right = fold_tree(r, acc, f);
            f(v, left, right)
        }
    }
}

/// All derived via fold — no explicit recursion needed.
pub fn size<T>(t: &Tree<T>) -> usize {
    fold_tree(t, 0, &|_, l, r| 1 + l + r)
}

pub fn depth<T>(t: &Tree<T>) -> usize {
    fold_tree(t, 0, &|_, l, r| 1 + l.max(r))
}

pub fn sum(t: &Tree<i32>) -> i32 {
    fold_tree(t, 0, &|v, l, r| v + l + r)
}

pub fn preorder<T: Clone>(t: &Tree<T>) -> Vec<T> {
    fold_tree(t, vec![], &|v, l, r| {
        let mut result = vec![v.clone()];
        result.extend(l);
        result.extend(r);
        result
    })
}

pub fn inorder<T: Clone>(t: &Tree<T>) -> Vec<T> {
    fold_tree(t, vec![], &|v, l, r| {
        let mut result = l;
        result.push(v.clone());
        result.extend(r);
        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use Tree::*;

    fn sample() -> Tree<i32> {
        //      4
        //     / \
        //    2   6
        //   / \
        //  1   3
        Tree::node(
            4,
            Tree::node(2, Tree::node(1, Leaf, Leaf), Tree::node(3, Leaf, Leaf)),
            Tree::node(6, Leaf, Leaf),
        )
    }

    #[test]
    fn test_size() {
        assert_eq!(size(&sample()), 5);
        assert_eq!(size::<i32>(&Leaf), 0);
    }

    #[test]
    fn test_depth() {
        assert_eq!(depth(&sample()), 3);
        assert_eq!(depth::<i32>(&Leaf), 0);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(&sample()), 16);
        assert_eq!(sum(&Leaf), 0);
    }

    #[test]
    fn test_preorder() {
        assert_eq!(preorder(&sample()), vec![4, 2, 1, 3, 6]);
    }

    #[test]
    fn test_inorder() {
        assert_eq!(inorder(&sample()), vec![1, 2, 3, 4, 6]);
    }

    #[test]
    fn test_map_tree() {
        let doubled = map_tree(&sample(), &|v| v * 2);
        assert_eq!(sum(&doubled), 32);
        assert_eq!(preorder(&doubled), vec![8, 4, 2, 6, 12]);
    }

    #[test]
    fn test_single_node() {
        let t = Tree::node(42, Leaf, Leaf);
        assert_eq!(size(&t), 1);
        assert_eq!(sum(&t), 42);
        assert_eq!(preorder(&t), vec![42]);
    }
}
