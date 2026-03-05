//! Seq.map, Seq.filter — Lazy Transformations
//!
//! Demonstrates lazy iterator pipelines in Rust as the direct equivalent of
//! OCaml's `Seq.map` and `Seq.filter` on infinite sequences.

/// Idiomatic: infinite range + map + filter + take, all lazy until collect.
pub fn even_squares_idiomatic(k: usize) -> Vec<u64> {
    (1u64..)
        .map(|n| n * n)
        .filter(|n| n % 2 == 0)
        .take(k)
        .collect()
}

/// Functional: uses `std::iter::successors` to mirror OCaml's `Seq.unfold`.
pub fn even_squares_with_successors(k: usize) -> Vec<u64> {
    std::iter::successors(Some(1u64), |&n| Some(n + 1))
        .map(|n| n * n)
        .filter(|n| n % 2 == 0)
        .take(k)
        .collect()
}

/// Generic: accepts any iterator, applies map-then-filter, returns Vec.
pub fn map_then_filter<I, T, U, F, P>(iter: I, f: F, p: P) -> Vec<U>
where
    I: Iterator<Item = T>,
    F: Fn(T) -> U,
    P: Fn(&U) -> bool,
{
    iter.map(f).filter(|u| p(u)).collect()
}

fn main() {
    // Mirror the OCaml output: 4 16 36 64 100 144 196 256
    let result = even_squares_idiomatic(8);
    println!("even_squares (idiomatic, k=8): {:?}", result);

    let result2 = even_squares_with_successors(8);
    println!("even_squares (successors, k=8): {:?}", result2);

    let generic: Vec<u64> = map_then_filter(1u64..=10, |n| n * n, |&s| s % 2 == 0);
    println!("map_then_filter even squares 1..=10: {:?}", generic);
}

/* Output:
   even_squares (idiomatic, k=8): [4, 16, 36, 64, 100, 144, 196, 256]
   even_squares (successors, k=8): [4, 16, 36, 64, 100, 144, 196, 256]
   map_then_filter even squares 1..=10: [4, 16, 36, 64, 100]
*/
