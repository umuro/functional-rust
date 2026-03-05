//! Seq — Zip and Iterate
//!
//! OCaml's `Seq.zip` pairs two lazy sequences element-by-element.
//! OCaml's `Seq.iterate f x` generates `x, f(x), f(f(x)), …` infinitely.
//!
//! Rust's `Iterator::zip` and `std::iter::successors` are the direct
//! equivalents: both are lazy and only evaluated on demand.

/// Zip two slices into a vector of pairs.
///
/// Mirrors `Seq.zip letters numbers |> List.of_seq`.
/// The result length equals the shorter of the two inputs — identical
/// to OCaml's behaviour.
///
/// Takes slices of `Copy` types to avoid unnecessary allocations.
pub fn zip_slices<A: Copy, B: Copy>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    a.iter().copied().zip(b.iter().copied()).collect()
}

/// The Collatz step function: n/2 if even, 3n+1 if odd.
pub fn collatz(n: u64) -> u64 {
    if n.is_multiple_of(2) {
        n / 2
    } else {
        3 * n + 1
    }
}

/// Collect the first `n` terms of the infinite sequence produced by
/// repeatedly applying `f` to `start`.
///
/// Mirrors `Seq.iterate f start |> Seq.take n |> List.of_seq`.
///
/// Uses `std::iter::successors`, which is the idiomatic Rust encoding of
/// `Seq.iterate`: each step receives a reference to the previous value and
/// returns `Some(next)` to continue or `None` to stop.
pub fn iterate<T, F>(f: F, start: T, n: usize) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> T,
{
    std::iter::successors(Some(start), |prev| Some(f(prev)))
        .take(n)
        .collect()
}

/// Functional / recursive style — explicit accumulator mirrors OCaml's
/// `let rec iterate_rec f x acc n = …` pattern.
pub fn iterate_recursive<T, F>(f: &F, start: T, n: usize) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> T,
{
    fn go<T: Clone, F: Fn(&T) -> T>(f: &F, current: T, remaining: usize, acc: &mut Vec<T>) {
        if remaining == 0 {
            return;
        }
        let next = f(&current);
        acc.push(current);
        go(f, next, remaining - 1, acc);
    }

    let mut acc = Vec::with_capacity(n);
    go(f, start, n, &mut acc);
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- zip_slices ---

    #[test]
    fn test_zip_empty() {
        let result = zip_slices::<char, i32>(&[], &[]);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_zip_chars_and_numbers_matches_ocaml() {
        // OCaml: Seq.zip (List.to_seq ['a';'b';'c';'d']) (List.to_seq [1;2;3;4])
        let letters = ['a', 'b', 'c', 'd'];
        let numbers = [1i32, 2, 3, 4];
        assert_eq!(
            zip_slices(&letters, &numbers),
            vec![('a', 1), ('b', 2), ('c', 3), ('d', 4)]
        );
    }

    #[test]
    fn test_zip_stops_at_shorter() {
        // zip stops at the shorter input — same as OCaml Seq.zip
        let a = [1, 2, 3];
        let b = [10, 20];
        assert_eq!(zip_slices(&a, &b), vec![(1, 10), (2, 20)]);
    }

    #[test]
    fn test_zip_single_element() {
        assert_eq!(zip_slices(&['x'], &[42i32]), vec![('x', 42)]);
    }

    // --- iterate (successors) ---

    #[test]
    fn test_iterate_collatz_20_steps() {
        // OCaml: Seq.iterate collatz 27 |> Seq.take 20 |> List.of_seq
        let result = iterate(|&n| collatz(n), 27u64, 20);
        assert_eq!(
            result,
            vec![
                27, 82, 41, 124, 62, 31, 94, 47, 142, 71, 214, 107, 322, 161, 484, 242, 121, 364,
                182, 91
            ]
        );
    }

    #[test]
    fn test_iterate_zero_steps() {
        let result = iterate(|&n: &u64| n + 1, 0u64, 0);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_iterate_double_sequence() {
        // start=1, f=*2 → 1, 2, 4, 8, 16
        let result = iterate(|&n: &u64| n * 2, 1u64, 5);
        assert_eq!(result, vec![1, 2, 4, 8, 16]);
    }

    #[test]
    fn test_iterate_identity_string() {
        // f = clone (identity-like) → same value repeated
        let result = iterate(|s: &String| s.clone(), "hi".to_string(), 3);
        assert_eq!(result, vec!["hi", "hi", "hi"]);
    }

    // --- iterate_recursive ---

    #[test]
    fn test_iterate_recursive_matches_successors() {
        let want = iterate(|&n| collatz(n), 27u64, 20);
        let got = iterate_recursive(&|n: &u64| collatz(*n), 27u64, 20);
        assert_eq!(got, want);
    }

    #[test]
    fn test_iterate_recursive_empty() {
        let result = iterate_recursive(&|n: &u64| n + 1, 0u64, 0);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_iterate_recursive_single() {
        let result = iterate_recursive(&|n: &u64| n + 1, 5u64, 1);
        assert_eq!(result, vec![5]);
    }
}
