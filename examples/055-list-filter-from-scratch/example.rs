//! # List Filter from Scratch
//! CS3110 — Deriving `List.filter`: predicate functions and preserving order.

/// Iterator-based filter — idiomatic Rust, mirrors OCaml's `List.filter`.
///
/// Uses `Iterator::filter` to select elements satisfying predicate `p`.
/// Preserves order. Zero-copy predicate — `p` borrows each element.
pub fn filter<T, P>(list: &[T], p: P) -> Vec<T>
where
    T: Clone,
    P: Fn(&T) -> bool,
{
    list.iter().filter(|x| p(x)).cloned().collect()
}

/// Recursive filter — structural translation of the OCaml pattern-match.
///
/// ```text
/// let rec filter p = function
///   | [] -> []
///   | h :: t -> if p h then h :: filter p t else filter p t
/// ```
///
/// Uses a `&dyn Fn` inner helper to avoid monomorphization explosion from
/// passing `&p` through recursive calls (which would wrap the type infinitely).
pub fn filter_rec<T, P>(list: &[T], p: P) -> Vec<T>
where
    T: Clone,
    P: Fn(&T) -> bool,
{
    fn go<T: Clone>(list: &[T], p: &dyn Fn(&T) -> bool) -> Vec<T> {
        match list {
            [] => vec![],
            [head, tail @ ..] => {
                let mut rest = go(tail, p);
                if p(head) {
                    let mut result = vec![head.clone()];
                    result.append(&mut rest);
                    result
                } else {
                    rest
                }
            }
        }
    }
    go(list, &p)
}

/// Fold-based filter — builds result via `fold`, applying predicate in accumulator.
pub fn filter_fold<T, P>(list: &[T], p: P) -> Vec<T>
where
    T: Clone,
    P: Fn(&T) -> bool,
{
    list.iter().fold(Vec::new(), |mut acc, x| {
        if p(x) {
            acc.push(x.clone());
        }
        acc
    })
}

/// Convenience: keep even integers.
pub fn evens(nums: &[i32]) -> Vec<i32> {
    filter(nums, |n| n % 2 == 0)
}

/// Convenience: keep odd integers.
pub fn odds(nums: &[i32]) -> Vec<i32> {
    filter(nums, |n| n % 2 != 0)
}

/// Convenience: keep positive integers.
pub fn pos(nums: &[i32]) -> Vec<i32> {
    filter(nums, |n| *n > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMS: &[i32] = &[-3, -1, 0, 2, 4, 5, 7];

    // --- evens / odds / pos (matches OCaml demo output) ---

    #[test]
    fn test_evens() {
        assert_eq!(evens(NUMS), vec![0, 2, 4]);
    }

    #[test]
    fn test_odds() {
        assert_eq!(odds(NUMS), vec![-3, -1, 5, 7]);
    }

    #[test]
    fn test_pos() {
        assert_eq!(pos(NUMS), vec![2, 4, 5, 7]);
    }

    // --- all three implementations agree ---

    #[test]
    fn test_all_impls_agree() {
        let is_even = |n: &i32| n % 2 == 0;
        let a = filter(NUMS, is_even);
        let b = filter_rec(NUMS, is_even);
        let c = filter_fold(NUMS, is_even);
        assert_eq!(a, b);
        assert_eq!(b, c);
    }

    // --- edge cases ---

    #[test]
    fn test_empty() {
        let empty: &[i32] = &[];
        assert_eq!(filter(empty, |_| true), Vec::<i32>::new());
        assert_eq!(filter_rec(empty, |_| true), Vec::<i32>::new());
        assert_eq!(filter_fold(empty, |_| true), Vec::<i32>::new());
    }

    #[test]
    fn test_none_match() {
        assert_eq!(filter(&[1, 3, 5], |n| n % 2 == 0), Vec::<i32>::new());
        assert_eq!(filter_rec(&[1, 3, 5], |n| n % 2 == 0), Vec::<i32>::new());
        assert_eq!(filter_fold(&[1, 3, 5], |n| n % 2 == 0), Vec::<i32>::new());
    }

    #[test]
    fn test_all_match() {
        assert_eq!(filter(&[2, 4, 6], |n| n % 2 == 0), vec![2, 4, 6]);
    }

    #[test]
    fn test_order_preserved() {
        // Filter must keep relative order of surviving elements.
        let input = &[5, 1, 4, 2, 3];
        let expected = vec![4, 2];
        assert_eq!(filter(input, |n| n % 2 == 0), expected);
        assert_eq!(filter_rec(input, |n| n % 2 == 0), expected);
        assert_eq!(filter_fold(input, |n| n % 2 == 0), expected);
    }

    #[test]
    fn test_generic_strings() {
        let words = vec!["hello", "hi", "world", "hey"];
        let h_words: Vec<&str> = filter(&words, |w| w.starts_with('h'));
        assert_eq!(h_words, vec!["hello", "hi", "hey"]);
    }
}

fn main() {
    println!("{:?}", evens(NUMS), vec![0, 2, 4]);
    println!("{:?}", odds(NUMS), vec![-3, -1, 5, 7]);
    println!("{:?}", pos(NUMS), vec![2, 4, 5, 7]);
}
