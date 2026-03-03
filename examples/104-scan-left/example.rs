//! # Scan Left — Running Accumulation
//!
//! `scan` returns all intermediate results of a fold.
//! Rust's `Iterator::scan` method does this lazily.

// ---------------------------------------------------------------------------
// Approach A: Iterator::scan (idiomatic Rust)
// ---------------------------------------------------------------------------

pub fn running_sum(xs: &[i32]) -> Vec<i32> {
    let mut acc = 0;
    std::iter::once(0)
        .chain(xs.iter().map(move |&x| {
            acc += x;
            acc
        }))
        .collect()
}

pub fn running_max(xs: &[i32]) -> Vec<i32> {
    xs.iter()
        .scan(i32::MIN, |state, &x| {
            *state = (*state).max(x);
            Some(*state)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Approach B: Manual scan_left (mirrors OCaml)
// ---------------------------------------------------------------------------

pub fn scan_left<T: Clone, U: Clone>(f: impl Fn(&T, &U) -> T, init: T, xs: &[U]) -> Vec<T> {
    let mut result = Vec::with_capacity(xs.len() + 1);
    result.push(init.clone());
    let mut acc = init;
    for x in xs {
        acc = f(&acc, x);
        result.push(acc.clone());
    }
    result
}

// ---------------------------------------------------------------------------
// Approach C: Fold-based scan
// ---------------------------------------------------------------------------

pub fn scan_fold<T: Clone>(f: impl Fn(T, T) -> T, init: T, xs: &[T]) -> Vec<T> {
    xs.iter().fold(vec![init.clone()], |mut res, x| {
        let last = res.last().unwrap().clone();
        res.push(f(last, x.clone()));
        res
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(running_sum(&[1, 2, 3, 4, 5]), vec![0, 1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_running_max() {
        assert_eq!(running_max(&[3, 1, 4, 1, 5, 9, 2, 6]), vec![3, 3, 4, 4, 5, 9, 9, 9]);
    }

    #[test]
    fn test_scan_left() {
        let result = scan_left(|a: &i32, b: &i32| a + b, 0, &[1, 2, 3, 4, 5]);
        assert_eq!(result, vec![0, 1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_scan_empty() {
        assert_eq!(scan_left(|a: &i32, b: &i32| a + b, 0, &[]), vec![0]);
    }

    #[test]
    fn test_scan_fold() {
        assert_eq!(scan_fold(|a, b| a + b, 0, &[1, 2, 3]), vec![0, 1, 3, 6]);
    }
}

fn main() {
    println!("{:?}", running_sum(&[1, 2, 3, 4, 5]), vec![0, 1, 3, 6, 10, 15]);
    println!("{:?}", running_max(&[3, 1, 4, 1, 5, 9, 2, 6]), vec![3, 3, 4, 4, 5, 9, 9, 9]);
    println!("{:?}", result, vec![0, 1, 3, 6, 10, 15]);
}
