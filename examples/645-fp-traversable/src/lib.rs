//! # Traversable
//!
//! Traversable extends Foldable with the ability to traverse a structure
//! while applying effects (like Option, Result).

use std::collections::VecDeque;

/// Traverse Vec with Option effects
pub fn traverse_option<A, B, F>(xs: Vec<A>, mut f: F) -> Option<Vec<B>>
where
    F: FnMut(A) -> Option<B>,
{
    let mut result = Vec::with_capacity(xs.len());
    for x in xs {
        match f(x) {
            Some(y) => result.push(y),
            None => return None,
        }
    }
    Some(result)
}

/// Sequence Vec<Option<A>> into Option<Vec<A>>
pub fn sequence_option<A>(xs: Vec<Option<A>>) -> Option<Vec<A>> {
    traverse_option(xs, |x| x)
}

/// Traverse Vec with Result effects
pub fn traverse_result<A, B, E, F>(xs: Vec<A>, mut f: F) -> Result<Vec<B>, E>
where
    F: FnMut(A) -> Result<B, E>,
{
    let mut result = Vec::with_capacity(xs.len());
    for x in xs {
        result.push(f(x)?);
    }
    Ok(result)
}

/// Sequence Vec<Result<A, E>> into Result<Vec<A>, E>
pub fn sequence_result<A, E>(xs: Vec<Result<A, E>>) -> Result<Vec<A>, E> {
    traverse_result(xs, |x| x)
}

// Approach 2: Option traverse
pub mod option_traverse {
    pub fn traverse<A, B, F>(opt: Option<A>, mut f: F) -> Option<Option<B>>
    where
        F: FnMut(A) -> Option<B>,
    {
        match opt {
            None => Some(None),
            Some(a) => f(a).map(Some),
        }
    }

    pub fn sequence<A>(opt: Option<Option<A>>) -> Option<Option<A>> {
        traverse(opt, |x| x)
    }
}

// Approach 3: Tree traverse
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

pub fn traverse_tree_option<A, B, F>(tree: Tree<A>, f: &mut F) -> Option<Tree<B>>
where
    F: FnMut(A) -> Option<B>,
{
    match tree {
        Tree::Leaf => Some(Tree::Leaf),
        Tree::Node(left, val, right) => {
            let new_left = traverse_tree_option(*left, f)?;
            let new_val = f(val)?;
            let new_right = traverse_tree_option(*right, f)?;
            Some(Tree::node(new_left, new_val, new_right))
        }
    }
}

/// Map and filter in one pass (catamorphism with filtering)
pub fn filter_map<A, B, F>(xs: Vec<A>, mut f: F) -> Vec<B>
where
    F: FnMut(A) -> Option<B>,
{
    xs.into_iter().filter_map(|x| f(x)).collect()
}

/// Partition based on Result
pub fn partition_result<A, B, E>(xs: Vec<Result<A, E>>) -> (Vec<A>, Vec<E>) {
    let mut oks = Vec::new();
    let mut errs = Vec::new();
    for x in xs {
        match x {
            Ok(a) => oks.push(a),
            Err(e) => errs.push(e),
        }
    }
    (oks, errs)
}

/// Traverse with state (stateful mapping)
pub fn map_accum<A, B, S, F>(xs: Vec<A>, mut state: S, mut f: F) -> (S, Vec<B>)
where
    F: FnMut(S, A) -> (S, B),
{
    let mut result = Vec::with_capacity(xs.len());
    for x in xs {
        let (new_state, b) = f(state, x);
        state = new_state;
        result.push(b);
    }
    (state, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traverse_option_all_some() {
        let xs = vec![1, 2, 3];
        let result = traverse_option(xs, |x| Some(x * 2));
        assert_eq!(result, Some(vec![2, 4, 6]));
    }

    #[test]
    fn test_traverse_option_with_none() {
        let xs = vec![1, 2, 3];
        let result = traverse_option(xs, |x| if x == 2 { None } else { Some(x) });
        assert_eq!(result, None);
    }

    #[test]
    fn test_sequence_option() {
        let xs = vec![Some(1), Some(2), Some(3)];
        assert_eq!(sequence_option(xs), Some(vec![1, 2, 3]));
        
        let xs = vec![Some(1), None, Some(3)];
        assert_eq!(sequence_option(xs), None);
    }

    #[test]
    fn test_traverse_result() {
        let xs = vec![1, 2, 3];
        let result: Result<Vec<i32>, &str> = traverse_result(xs, |x| Ok(x * 2));
        assert_eq!(result, Ok(vec![2, 4, 6]));
    }

    #[test]
    fn test_traverse_result_error() {
        let xs = vec![1, 2, 3];
        let result: Result<Vec<i32>, &str> = traverse_result(xs, |x| {
            if x == 2 { Err("error") } else { Ok(x) }
        });
        assert_eq!(result, Err("error"));
    }

    #[test]
    fn test_tree_traverse() {
        let tree = Tree::node(Tree::leaf(), 1, Tree::node(Tree::leaf(), 2, Tree::leaf()));
        let result = traverse_tree_option(tree, &mut |x| Some(x * 10));
        let expected = Tree::node(Tree::leaf(), 10, Tree::node(Tree::leaf(), 20, Tree::leaf()));
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_filter_map() {
        let xs = vec![1, 2, 3, 4, 5];
        let result = filter_map(xs, |x| if x % 2 == 0 { Some(x * 10) } else { None });
        assert_eq!(result, vec![20, 40]);
    }

    #[test]
    fn test_partition_result() {
        let xs: Vec<Result<i32, &str>> = vec![Ok(1), Err("a"), Ok(2), Err("b")];
        let (oks, errs) = partition_result(xs);
        assert_eq!(oks, vec![1, 2]);
        assert_eq!(errs, vec!["a", "b"]);
    }

    #[test]
    fn test_map_accum() {
        let xs = vec![1, 2, 3];
        let (total, running_sums) = map_accum(xs, 0, |acc, x| {
            let new_acc = acc + x;
            (new_acc, new_acc)
        });
        assert_eq!(total, 6);
        assert_eq!(running_sums, vec![1, 3, 6]);
    }

    #[test]
    fn test_option_traverse() {
        use option_traverse::*;
        assert_eq!(traverse(Some(5), |x| Some(x * 2)), Some(Some(10)));
        assert_eq!(traverse(Some(5), |_: i32| None::<i32>), None);
        assert_eq!(traverse(None::<i32>, |x| Some(x * 2)), Some(None));
    }
}
