#![allow(clippy::all)]
/// Filter removes elements that don't satisfy the predicate.
/// Idiomatic Rust: uses iterator chains like the Rust standard library
pub fn filter<T: Clone>(predicate: fn(&T) -> bool, items: &[T]) -> Vec<T> {
    items.iter().filter(|x| predicate(x)).cloned().collect()
}

/// Filter using immutable recursion — closer to OCaml style.
/// Shows the explicit pattern matching on the list structure.
pub fn filter_recursive<T: Clone>(predicate: fn(&T) -> bool, items: &[T]) -> Vec<T> {
    match items {
        [] => vec![],
        [h, rest @ ..] => {
            let mut tail = filter_recursive(predicate, rest);
            if predicate(h) {
                let mut result = vec![h.clone()];
                result.append(&mut tail);
                result
            } else {
                tail
            }
        }
    }
}

/// Filter using fold/reduce pattern — functional accumulation.
/// Demonstrates left fold over the slice.
pub fn filter_fold<T: Clone>(predicate: fn(&T) -> bool, items: &[T]) -> Vec<T> {
    items.iter().fold(vec![], |mut acc, x| {
        if predicate(x) {
            acc.push(x.clone());
        }
        acc
    })
}

// Predicates
pub fn is_even(x: &i32) -> bool {
    x % 2 == 0
}

pub fn is_odd(x: &i32) -> bool {
    x % 2 != 0
}

pub fn is_positive(x: &i32) -> bool {
    *x > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_empty() {
        let nums: Vec<i32> = vec![];
        assert_eq!(filter(is_even, &nums), vec![]);
    }

    #[test]
    fn test_filter_single() {
        assert_eq!(filter(is_even, &[2]), vec![2]);
        assert_eq!(filter(is_even, &[3]), vec![]);
    }

    #[test]
    fn test_filter_multiple() {
        let nums = vec![-2, -1, 0, 1, 2, 3, 4];
        let evens = filter(is_even, &nums);
        assert_eq!(evens, vec![-2, 0, 2, 4]);
    }

    #[test]
    fn test_filter_all_pass() {
        let nums = vec![2, 4, 6];
        assert_eq!(filter(is_even, &nums), vec![2, 4, 6]);
    }

    #[test]
    fn test_filter_none_pass() {
        let nums = vec![1, 3, 5];
        let evens: Vec<i32> = filter(is_even, &nums);
        assert_eq!(evens, vec![]);
    }

    #[test]
    fn test_filter_positive() {
        let nums = vec![-2, -1, 0, 1, 2, 3, 4];
        assert_eq!(filter(is_positive, &nums), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_filter_odd() {
        let nums = vec![-2, -1, 0, 1, 2, 3, 4];
        assert_eq!(filter(is_odd, &nums), vec![-1, 1, 3]);
    }

    #[test]
    fn test_filter_recursive_multiple() {
        let nums = vec![-2, -1, 0, 1, 2, 3, 4];
        let evens = filter_recursive(is_even, &nums);
        assert_eq!(evens, vec![-2, 0, 2, 4]);
    }

    #[test]
    fn test_filter_recursive_positive() {
        let nums = vec![-2, -1, 0, 1, 2, 3, 4];
        assert_eq!(filter_recursive(is_positive, &nums), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_filter_fold_multiple() {
        let nums = vec![-2, -1, 0, 1, 2, 3, 4];
        let evens = filter_fold(is_even, &nums);
        assert_eq!(evens, vec![-2, 0, 2, 4]);
    }

    #[test]
    fn test_filter_fold_positive() {
        let nums = vec![-2, -1, 0, 1, 2, 3, 4];
        assert_eq!(filter_fold(is_positive, &nums), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_all_implementations_same() {
        let nums = vec![-2, -1, 0, 1, 2, 3, 4];
        let idiomatic = filter(is_even, &nums);
        let recursive = filter_recursive(is_even, &nums);
        let fold = filter_fold(is_even, &nums);
        assert_eq!(idiomatic, recursive);
        assert_eq!(recursive, fold);
    }
}
