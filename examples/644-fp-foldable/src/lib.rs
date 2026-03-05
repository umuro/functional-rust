//! # Foldable
//!
//! Foldable represents data structures that can be folded to a summary value.

/// Trait for foldable data structures
pub trait Foldable {
    type Item;
    
    fn fold_right<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(Self::Item, B) -> B;
    
    fn fold_left<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B;
}

// Approach 1: Vec as Foldable
impl<T> Foldable for Vec<T> {
    type Item = T;
    
    fn fold_right<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(T, B) -> B,
    {
        self.into_iter().rev().fold(init, |acc, x| f(x, acc))
    }
    
    fn fold_left<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, T) -> B,
    {
        self.into_iter().fold(init, |acc, x| f(acc, x))
    }
}

// Approach 2: Option as Foldable
impl<T> Foldable for Option<T> {
    type Item = T;
    
    fn fold_right<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(T, B) -> B,
    {
        match self {
            Some(x) => f(x, init),
            None => init,
        }
    }
    
    fn fold_left<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, T) -> B,
    {
        match self {
            Some(x) => f(init, x),
            None => init,
        }
    }
}

// Approach 3: Binary tree as Foldable
#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T> Tree<T> {
    pub fn leaf() -> Self { Tree::Leaf }
    
    pub fn node(left: Tree<T>, value: T, right: Tree<T>) -> Self {
        Tree::Node(Box::new(left), value, Box::new(right))
    }
}

impl<T> Foldable for Tree<T> {
    type Item = T;
    
    fn fold_right<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(T, B) -> B,
    {
        match self {
            Tree::Leaf => init,
            Tree::Node(left, val, right) => {
                let right_folded = right.fold_right(init, &mut f);
                let with_val = f(val, right_folded);
                left.fold_right(with_val, f)
            }
        }
    }
    
    fn fold_left<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, T) -> B,
    {
        match self {
            Tree::Leaf => init,
            Tree::Node(left, val, right) => {
                let left_folded = left.fold_left(init, &mut f);
                let with_val = f(left_folded, val);
                right.fold_left(with_val, f)
            }
        }
    }
}

/// Derived operations from Foldable
pub fn to_list<F: Foldable>(foldable: F) -> Vec<F::Item> {
    foldable.fold_right(vec![], |x, mut acc| {
        acc.insert(0, x);
        acc
    })
}

pub fn length<F: Foldable>(foldable: F) -> usize {
    foldable.fold_left(0, |acc, _| acc + 1)
}

pub fn sum<F: Foldable<Item = i32>>(foldable: F) -> i32 {
    foldable.fold_left(0, |acc, x| acc + x)
}

pub fn any<F: Foldable, P>(foldable: F, mut predicate: P) -> bool
where
    P: FnMut(&F::Item) -> bool,
{
    foldable.fold_left(false, |acc, x| acc || predicate(&x))
}

pub fn all<F: Foldable, P>(foldable: F, mut predicate: P) -> bool
where
    P: FnMut(&F::Item) -> bool,
{
    foldable.fold_left(true, |acc, x| acc && predicate(&x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_fold_left() {
        let v = vec![1, 2, 3, 4];
        let result = v.fold_left(0, |acc, x| acc + x);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_vec_fold_right() {
        let v = vec![1, 2, 3];
        let result = v.fold_right(vec![], |x, mut acc| {
            acc.insert(0, x);
            acc
        });
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_option_fold_some() {
        let opt = Some(42);
        let result = opt.fold_left(0, |acc, x| acc + x);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_option_fold_none() {
        let opt: Option<i32> = None;
        let result = opt.fold_left(100, |acc, x| acc + x);
        assert_eq!(result, 100);
    }

    #[test]
    fn test_tree_fold() {
        let tree = Tree::node(
            Tree::node(Tree::leaf(), 1, Tree::leaf()),
            2,
            Tree::node(Tree::leaf(), 3, Tree::leaf()),
        );
        let result = tree.fold_left(0, |acc, x| acc + x);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_length() {
        assert_eq!(length(vec![1, 2, 3, 4, 5]), 5);
        assert_eq!(length(Some(42)), 1);
        assert_eq!(length(None::<i32>), 0);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn test_any() {
        assert!(any(vec![1, 2, 3], |x| *x > 2));
        assert!(!any(vec![1, 2, 3], |x| *x > 5));
    }

    #[test]
    fn test_all() {
        assert!(all(vec![2, 4, 6], |x| *x % 2 == 0));
        assert!(!all(vec![2, 3, 6], |x| *x % 2 == 0));
    }

    #[test]
    fn test_tree_to_list() {
        let tree = Tree::node(
            Tree::node(Tree::leaf(), 1, Tree::leaf()),
            2,
            Tree::node(Tree::leaf(), 3, Tree::leaf()),
        );
        assert_eq!(to_list(tree), vec![1, 2, 3]);
    }
}
