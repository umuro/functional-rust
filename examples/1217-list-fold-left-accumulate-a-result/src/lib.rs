#![allow(dead_code)]
//! `List.fold_left` — accumulate a result by walking a list left-to-right.
//!
//! Given a combining function `f : 'acc -> 'elt -> 'acc`, a seed
//! `init : 'acc`, and a list `[x1; x2; ...; xn]`, fold_left computes
//! `f (... (f (f init x1) x2) ...) xn`.
//!
//! Three Rust translations are shown:
//!   * idiomatic iterator chain using `Iterator::fold`,
//!   * explicit pattern-matched recursion (closest to OCaml),
//!   * a plain `for` loop with a mutable accumulator (imperative baseline).
//!
//! The combining function takes the accumulator **by value** and returns a
//! new one, mirroring OCaml's pure fold rather than a mutate-in-place
//! `for_each`.

/// Idiomatic Rust: `iter().fold(init, f)`.
///
/// `Iterator::fold` is the exact structural analogue of OCaml's
/// `List.fold_left`.  The closure consumes the accumulator and an element
/// reference and returns the new accumulator.
pub fn fold_left<T, A, F>(f: F, init: A, items: &[T]) -> A
where
    F: Fn(A, &T) -> A,
{
    items.iter().fold(init, f)
}

/// Functional / recursive translation — the direct OCaml parallel.
/// Matches on the slice shape `[] | [h, rest @ ..]`, mirroring
/// `let rec fold_left f acc = function [] -> acc | x :: xs -> ...`.
///
/// Note the tail-recursive shape: the recursive call is the outermost
/// expression, so an optimising compiler can reuse the stack frame.
pub fn fold_left_recursive<T, A, F>(f: &F, acc: A, items: &[T]) -> A
where
    F: Fn(A, &T) -> A,
{
    match items {
        [] => acc,
        [h, rest @ ..] => fold_left_recursive(f, f(acc, h), rest),
    }
}

/// Imperative translation — explicit mutable accumulator and `for` loop.
/// Clippy will normally push you towards `fold`, but this form makes the
/// "left-to-right threading of state" visible for pedagogy.
pub fn fold_left_loop<T, A, F>(f: F, init: A, items: &[T]) -> A
where
    F: Fn(A, &T) -> A,
{
    let mut acc = init;
    for x in items {
        acc = f(acc, x);
    }
    acc
}

// --- Concrete applications matching the OCaml example -------------------

/// Sum of a slice of integers — `List.fold_left (+) 0 numbers`.
pub fn sum(numbers: &[i32]) -> i32 {
    fold_left(|acc, x| acc + x, 0, numbers)
}

/// Product of a slice of integers — `List.fold_left ( * ) 1 numbers`.
pub fn product(numbers: &[i32]) -> i32 {
    fold_left(|acc, x| acc * x, 1, numbers)
}

/// Concatenate integers into a labelled string — mirrors the OCaml
/// `fold_left (fun acc x -> acc ^ " " ^ string_of_int x) "Numbers:" numbers`.
pub fn concat_labelled(label: &str, numbers: &[i32]) -> String {
    fold_left(
        |acc, x| acc + " " + &x.to_string(),
        label.to_string(),
        numbers,
    )
}

/// Reverse a slice via fold — the canonical demonstration that `fold_left`
/// can build structure, not just reduce to a scalar.
/// Equivalent to `List.fold_left (fun acc x -> x :: acc) [] lst`.
pub fn reverse<T: Copy>(items: &[T]) -> Vec<T> {
    fold_left(
        |mut acc, x| {
            acc.insert(0, *x);
            acc
        },
        Vec::new(),
        items,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS: &[i32] = &[1, 2, 3, 4, 5];

    // --- generic fold_left --------------------------------------------

    #[test]
    fn test_fold_left_sum_empty() {
        let empty: [i32; 0] = [];
        assert_eq!(fold_left(|acc, x| acc + x, 0, &empty), 0);
    }

    #[test]
    fn test_fold_left_sum_single() {
        assert_eq!(fold_left(|acc, x| acc + x, 0, &[7]), 7);
    }

    #[test]
    fn test_fold_left_sum_many() {
        assert_eq!(fold_left(|acc, x| acc + x, 0, NUMBERS), 15);
    }

    #[test]
    fn test_fold_left_product() {
        assert_eq!(fold_left(|acc, x| acc * x, 1, NUMBERS), 120);
    }

    #[test]
    fn test_fold_left_order_is_left_to_right() {
        // subtraction is not associative — left fold computes
        // ((((0 - 1) - 2) - 3) - 4) - 5 = -15
        assert_eq!(fold_left(|acc, x| acc - x, 0, NUMBERS), -15);
    }

    #[test]
    fn test_fold_left_with_non_commutative_concat() {
        // String concatenation exposes fold direction.
        let out = fold_left(|acc, x: &&str| acc + x, String::from("<"), &["a", "b", "c"]);
        assert_eq!(out, "<abc");
    }

    // --- concrete helpers ---------------------------------------------

    #[test]
    fn test_sum_helper() {
        assert_eq!(sum(NUMBERS), 15);
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn test_product_helper() {
        assert_eq!(product(NUMBERS), 120);
        assert_eq!(product(&[]), 1);
    }

    #[test]
    fn test_concat_labelled_matches_ocaml() {
        assert_eq!(concat_labelled("Numbers:", NUMBERS), "Numbers: 1 2 3 4 5");
    }

    #[test]
    fn test_concat_labelled_empty_numbers() {
        assert_eq!(concat_labelled("Numbers:", &[]), "Numbers:");
    }

    #[test]
    fn test_reverse_via_fold() {
        assert_eq!(reverse(NUMBERS), vec![5, 4, 3, 2, 1]);
        assert_eq!(reverse::<i32>(&[]), Vec::<i32>::new());
    }

    // --- recursive variant --------------------------------------------

    #[test]
    fn test_fold_left_recursive_sum() {
        assert_eq!(fold_left_recursive(&|acc, x| acc + x, 0, NUMBERS), 15);
    }

    #[test]
    fn test_fold_left_recursive_empty() {
        let empty: [i32; 0] = [];
        assert_eq!(fold_left_recursive(&|acc, x| acc + x, 42, &empty), 42);
    }

    // --- loop variant -------------------------------------------------

    #[test]
    fn test_fold_left_loop_product() {
        assert_eq!(fold_left_loop(|acc, x| acc * x, 1, NUMBERS), 120);
    }

    // --- cross-check --------------------------------------------------

    #[test]
    fn test_all_implementations_agree() {
        let a = fold_left(|acc, x| acc - x, 100, NUMBERS);
        let b = fold_left_recursive(&|acc, x| acc - x, 100, NUMBERS);
        let c = fold_left_loop(|acc, x| acc - x, 100, NUMBERS);
        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(a, 85);
    }
}
