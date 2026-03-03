//! # List Map from Scratch
//! Deriving `List.map` from scratch — the Abstraction Principle.
//!
//! Three implementations showing how `map` generalises element-by-element
//! list transformation: iterator adapter, explicit recursion, and fold.

/// Iterator-based map: apply `f` to every element and collect.
///
/// This is idiomatic Rust — the standard library's `Iterator::map` does
/// exactly this under the hood, but we call `.collect()` explicitly to
/// mirror the OCaml `'a list -> 'b list` shape.
pub fn map<A, B, F>(list: &[A], f: F) -> Vec<B>
where
    F: Fn(&A) -> B,
{
    list.iter().map(f).collect()
}

/// Recursive map: head-cons pattern mirroring the OCaml definition.
///
/// ```ocaml
/// let rec map f = function
///   | [] -> []
///   | h :: t -> let h' = f h in h' :: map f t
/// ```
pub fn map_recursive<A, B, F>(list: &[A], f: F) -> Vec<B>
where
    F: Fn(&A) -> B,
{
    match list {
        [] => vec![],
        [head, tail @ ..] => {
            let mut result = vec![f(head)];
            result.extend(map_recursive(tail, f));
            result
        }
    }
}

/// Fold-based map: build the result left-to-right with `fold`.
///
/// Demonstrates that `map` is a special case of `fold` where each step
/// appends a transformed element.
pub fn map_fold<A, B, F>(list: &[A], f: F) -> Vec<B>
where
    F: Fn(&A) -> B,
{
    list.iter().fold(Vec::new(), |mut acc, x| {
        acc.push(f(x));
        acc
    })
}

// Partially-applied helpers matching the OCaml `add1`, `to_string`, `double`.

pub fn add1(list: &[i32]) -> Vec<i32> {
    map(list, |x| x + 1)
}

pub fn to_string(list: &[i32]) -> Vec<String> {
    map(list, |x| x.to_string())
}

pub fn double(list: &[i32]) -> Vec<i32> {
    map(list, |x| x * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMS: &[i32] = &[1, 2, 3, 4, 5];

    // --- iterator map ---

    #[test]
    fn map_add1() {
        assert_eq!(map(NUMS, |x| x + 1), vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn map_to_string() {
        assert_eq!(map(NUMS, |x| x.to_string()), vec!["1", "2", "3", "4", "5"]);
    }

    #[test]
    fn map_double() {
        assert_eq!(map(NUMS, |x| x * 2), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn map_empty() {
        assert_eq!(map::<i32, i32, _>(&[], |x| x + 1), vec![]);
    }

    // --- recursive map ---

    #[test]
    fn map_recursive_add1() {
        assert_eq!(map_recursive(NUMS, |x| x + 1), vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn map_recursive_double() {
        assert_eq!(map_recursive(NUMS, |x| x * 2), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn map_recursive_empty() {
        assert_eq!(map_recursive::<i32, i32, _>(&[], |x| x + 1), vec![]);
    }

    // --- fold map ---

    #[test]
    fn map_fold_add1() {
        assert_eq!(map_fold(NUMS, |x| x + 1), vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn map_fold_double() {
        assert_eq!(map_fold(NUMS, |x| x * 2), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn map_fold_empty() {
        assert_eq!(map_fold::<i32, i32, _>(&[], |x| x + 1), vec![]);
    }

    // --- helpers ---

    #[test]
    fn helpers_match_ocaml_output() {
        assert_eq!(add1(NUMS), vec![2, 3, 4, 5, 6]);
        assert_eq!(to_string(NUMS), vec!["1", "2", "3", "4", "5"]);
        assert_eq!(double(NUMS), vec![2, 4, 6, 8, 10]);
    }

    // All three implementations must agree
    #[test]
    fn all_implementations_agree() {
        let f = |x: &i32| x * x;
        assert_eq!(map(NUMS, f), map_recursive(NUMS, f));
        assert_eq!(map(NUMS, f), map_fold(NUMS, f));
    }
}

fn main() {
    println!("{:?}", map(NUMS, |x| x + 1), vec![2, 3, 4, 5, 6]);
    println!("{:?}", map(NUMS, |x| x.to_string()), vec!["1", "2", "3", "4", "5"]);
    println!("{:?}", map(NUMS, |x| x * 2), vec![2, 4, 6, 8, 10]);
}
