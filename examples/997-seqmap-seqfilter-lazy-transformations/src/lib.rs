//! Seq.map, Seq.filter — Lazy Transformations
//!
//! OCaml's `Seq` module provides lazy sequences. Rust's `Iterator` trait is the
//! direct equivalent: both are lazy, composable, and evaluated only on demand.

/// Solution 1: Idiomatic Rust — iterator chain mirrors OCaml's Seq pipeline.
///
/// `(1..).map(|n| n * n).filter(|n| n % 2 == 0).take(k)` is the exact
/// structural equivalent of the OCaml:
///   `naturals |> Seq.map (fun n -> n*n) |> Seq.filter (fun n -> n mod 2 = 0) |> Seq.take k`
pub fn even_squares_idiomatic(k: usize) -> Vec<u64> {
    (1u64..)
        .map(|n| n * n)
        .filter(|n| n % 2 == 0)
        .take(k)
        .collect()
}

/// Solution 2: Functional / explicit — uses a custom infinite-sequence generator
/// via `std::iter::successors`, mirroring OCaml's `Seq.unfold`.
///
/// `Seq.unfold (fun n -> Some (n, n + 1)) 1` ↔ `successors(Some(1u64), |&n| Some(n + 1))`
pub fn even_squares_with_successors(k: usize) -> Vec<u64> {
    std::iter::successors(Some(1u64), |&n| Some(n + 1))
        .map(|n| n * n)
        .filter(|n| n % 2 == 0)
        .take(k)
        .collect()
}

/// Solution 3: Generic lazy transformer — accepts any iterator and applies
/// map-then-filter lazily, returning a `Vec`. Shows how Rust encodes the
/// OCaml `Seq.map f |> Seq.filter p` pipeline as a reusable function.
pub fn map_then_filter<I, T, U, F, P>(iter: I, f: F, p: P) -> Vec<U>
where
    I: Iterator<Item = T>,
    F: Fn(T) -> U,
    P: Fn(&U) -> bool,
{
    iter.map(f).filter(|u| p(u)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_squares_idiomatic_empty() {
        assert_eq!(even_squares_idiomatic(0), Vec::<u64>::new());
    }

    #[test]
    fn test_even_squares_idiomatic_first_eight() {
        // OCaml output: 4 16 36 64 100 144 196 256
        assert_eq!(
            even_squares_idiomatic(8),
            vec![4, 16, 36, 64, 100, 144, 196, 256]
        );
    }

    #[test]
    fn test_even_squares_idiomatic_single() {
        // 2² = 4 is the first even square
        assert_eq!(even_squares_idiomatic(1), vec![4]);
    }

    #[test]
    fn test_even_squares_with_successors_matches_idiomatic() {
        let want = even_squares_idiomatic(8);
        let got = even_squares_with_successors(8);
        assert_eq!(got, want);
    }

    #[test]
    fn test_even_squares_with_successors_empty() {
        assert_eq!(even_squares_with_successors(0), Vec::<u64>::new());
    }

    #[test]
    fn test_map_then_filter_squares_even() {
        let result = map_then_filter(1u64..=10, |n| n * n, |&s| s % 2 == 0);
        // Even squares from 1..=10: 2²=4, 4²=16, 6²=36, 8²=64, 10²=100
        assert_eq!(result, vec![4, 16, 36, 64, 100]);
    }

    #[test]
    fn test_map_then_filter_no_match() {
        // Odd numbers squared are odd; filter for even → empty
        let result = map_then_filter([1u64, 3, 5, 7].into_iter(), |n| n * n, |&s| s % 2 == 0);
        assert_eq!(result, Vec::<u64>::new());
    }

    #[test]
    fn test_map_then_filter_strings() {
        let words = ["hello", "world", "rust", "ocaml", "seq"];
        let result: Vec<String> =
            map_then_filter(words.iter().copied(), |s| s.to_uppercase(), |s| s.len() > 4);
        assert_eq!(result, vec!["HELLO", "WORLD", "OCAML"]);
    }

    #[test]
    fn test_lazy_does_not_overflow() {
        // Confirms the chain is truly lazy — we only evaluate 3 elements
        // from an infinite range, not the whole u64 space.
        let result = even_squares_idiomatic(3);
        assert_eq!(result, vec![4, 16, 36]);
    }
}
