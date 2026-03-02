//! # Flatten Nested List
//! OCaml 99 Problems #7 — Flatten an arbitrarily nested list structure.

/// Mirrors OCaml's `type 'a node = One of 'a | Many of 'a node list`.
/// Rust enum with owned data — no GC, explicit ownership.
#[derive(Debug, PartialEq, Clone)]
pub enum Node<T> {
    One(T),
    Many(Vec<Node<T>>),
}

/// Idiomatic Rust: use `flat_map` with recursive flattening.
/// Requires `Clone` because we extract values from a borrowed structure.
pub fn flatten<T: Clone>(list: &[Node<T>]) -> Vec<T> {
    list.iter()
        .flat_map(|node| match node {
            Node::One(x) => vec![x.clone()],
            Node::Many(xs) => flatten(xs),
        })
        .collect()
}

/// Functional style: tail-recursive with accumulator (mirrors OCaml's `aux acc`).
/// Uses a stack to avoid actual recursion — Rust doesn't guarantee TCO.
pub fn flatten_stack<T: Clone>(list: &[Node<T>]) -> Vec<T> {
    let mut result = Vec::new();
    // Explicit stack: process nodes right-to-left so result is in order
    let mut stack: Vec<&Node<T>> = list.iter().rev().collect();
    while let Some(node) = stack.pop() {
        match node {
            Node::One(x) => result.push(x.clone()),
            Node::Many(xs) => {
                for child in xs.iter().rev() {
                    stack.push(child);
                }
            }
        }
    }
    result
}

/// Consuming version: takes ownership, no cloning needed.
/// This is the most efficient Rust approach when you own the data.
pub fn flatten_owned<T>(list: Vec<Node<T>>) -> Vec<T> {
    let mut result = Vec::new();
    let mut stack: Vec<Node<T>> = list.into_iter().rev().collect();
    while let Some(node) = stack.pop() {
        match node {
            Node::One(x) => result.push(x),
            Node::Many(xs) => {
                for child in xs.into_iter().rev() {
                    stack.push(child);
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use Node::*;

    #[test]
    fn test_nested() {
        let input = vec![
            One(1),
            Many(vec![One(2), Many(vec![One(3), One(4)])]),
            One(5),
        ];
        assert_eq!(flatten(&input), vec![1, 2, 3, 4, 5]);
        assert_eq!(flatten_stack(&input), vec![1, 2, 3, 4, 5]);
        assert_eq!(flatten_owned(input), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(flatten::<i32>(&[]), Vec::<i32>::new());
        assert_eq!(flatten_stack::<i32>(&[]), Vec::<i32>::new());
        assert_eq!(flatten_owned::<i32>(vec![]), Vec::<i32>::new());
    }

    #[test]
    fn test_single() {
        assert_eq!(flatten(&[One(42)]), vec![42]);
    }

    #[test]
    fn test_flat_list() {
        let input = vec![One(1), One(2), One(3)];
        assert_eq!(flatten(&input), vec![1, 2, 3]);
    }

    #[test]
    fn test_deeply_nested() {
        let input = vec![Many(vec![Many(vec![Many(vec![One(1)])])])];
        assert_eq!(flatten(&input), vec![1]);
    }

    #[test]
    fn test_empty_many() {
        let input: Vec<Node<i32>> = vec![Many(vec![]), One(1)];
        assert_eq!(flatten(&input), vec![1]);
    }
}
