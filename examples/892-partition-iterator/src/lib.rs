#![allow(clippy::all)]
//! Example 892: Partition Iterator
//! Split a collection into two groups by a predicate — single pass, two outputs.

// === Approach 1: Basic partition using Iterator::partition ===

/// Split integers into even and odd — idiomatic Rust, one pass.
pub fn split_even_odd(data: &[i32]) -> (Vec<i32>, Vec<i32>) {
    data.iter().partition(|&&x| x % 2 == 0)
}

/// Split integers into positive and non-positive.
pub fn split_positive(data: &[i32]) -> (Vec<i32>, Vec<i32>) {
    data.iter().partition(|&&x| x > 0)
}

// === Approach 2: Multi-way partition (fold-based, mirrors OCaml's fold_right) ===

/// Partition into three groups by two predicates.
/// Elements matching `p1` go to `a`, those matching `p2` (but not `p1`) to `b`,
/// and the rest to `c`.
pub fn partition3<T: Clone>(
    data: &[T],
    p1: impl Fn(&T) -> bool,
    p2: impl Fn(&T) -> bool,
) -> (Vec<T>, Vec<T>, Vec<T>) {
    data.iter().cloned().fold(
        (Vec::new(), Vec::new(), Vec::new()),
        |(mut a, mut b, mut c), x| {
            if p1(&x) {
                a.push(x);
            } else if p2(&x) {
                b.push(x);
            } else {
                c.push(x);
            }
            (a, b, c)
        },
    )
}

/// Classify integers into negatives, zeros, and positives.
pub fn classify_numbers(data: &[i32]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    partition3(data, |&x| x < 0, |&x| x == 0)
}

// === Approach 3: partition_map — route elements to Left or Right with transformation ===

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

/// Partition with simultaneous transformation.
/// Each element is mapped to `Either::Left` or `Either::Right`; results are collected
/// into two separate `Vec`s — mirrors OCaml's `Either`-based `partition_map`.
pub fn partition_map<T, L, R>(data: &[T], f: impl Fn(&T) -> Either<L, R>) -> (Vec<L>, Vec<R>) {
    data.iter()
        .fold((Vec::new(), Vec::new()), |(mut lefts, mut rights), x| {
            match f(x) {
                Either::Left(l) => lefts.push(l),
                Either::Right(r) => rights.push(r),
            }
            (lefts, rights)
        })
}

/// Parse strings into successes (i32) and failures (original string).
pub fn parse_numbers<'a>(data: &[&'a str]) -> (Vec<i32>, Vec<&'a str>) {
    partition_map(data, |s| match s.parse::<i32>() {
        Ok(n) => Either::Left(n),
        Err(_) => Either::Right(*s),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- split_even_odd ---

    #[test]
    fn test_split_even_odd_empty() {
        let (evens, odds) = split_even_odd(&[]);
        assert_eq!(evens, Vec::<i32>::new());
        assert_eq!(odds, Vec::<i32>::new());
    }

    #[test]
    fn test_split_even_odd_mixed() {
        let (evens, odds) = split_even_odd(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_split_even_odd_all_even() {
        let (evens, odds) = split_even_odd(&[2, 4, 8]);
        assert_eq!(evens, vec![2, 4, 8]);
        assert!(odds.is_empty());
    }

    #[test]
    fn test_split_even_odd_negatives() {
        let (evens, odds) = split_even_odd(&[-4, -3, 0, 1]);
        assert_eq!(evens, vec![-4, 0]);
        assert_eq!(odds, vec![-3, 1]);
    }

    // --- split_positive ---

    #[test]
    fn test_split_positive_mixed() {
        let (pos, non_pos) = split_positive(&[-2, -1, 0, 1, 2]);
        assert_eq!(pos, vec![1, 2]);
        assert_eq!(non_pos, vec![-2, -1, 0]);
    }

    // --- classify_numbers ---

    #[test]
    fn test_classify_numbers() {
        let (neg, zero, pos) = classify_numbers(&[-3, -1, 0, 0, 2, 5]);
        assert_eq!(neg, vec![-3, -1]);
        assert_eq!(zero, vec![0, 0]);
        assert_eq!(pos, vec![2, 5]);
    }

    #[test]
    fn test_classify_numbers_empty() {
        let (neg, zero, pos) = classify_numbers(&[]);
        assert!(neg.is_empty());
        assert!(zero.is_empty());
        assert!(pos.is_empty());
    }

    // --- parse_numbers (partition_map) ---

    #[test]
    fn test_parse_numbers_mixed() {
        let (parsed, failed) = parse_numbers(&["1", "two", "3", "four", "5"]);
        assert_eq!(parsed, vec![1, 3, 5]);
        assert_eq!(failed, vec!["two", "four"]);
    }

    #[test]
    fn test_parse_numbers_all_valid() {
        let (parsed, failed) = parse_numbers(&["10", "20", "30"]);
        assert_eq!(parsed, vec![10, 20, 30]);
        assert!(failed.is_empty());
    }

    #[test]
    fn test_parse_numbers_all_invalid() {
        let (parsed, failed) = parse_numbers(&["a", "b", "c"]);
        assert!(parsed.is_empty());
        assert_eq!(failed, vec!["a", "b", "c"]);
    }
}
