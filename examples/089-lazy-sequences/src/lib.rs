//! # Lazy Sequences
//!
//! Demonstrates Rust's lazily-evaluated iterators, mirroring OCaml's `Seq` module.
//! Rust iterators are lazy by default — computation only occurs when values are polled.

/// Returns an infinite iterator of natural numbers starting from 0.
///
/// # Examples
/// ```
/// use example_089_lazy_sequences::naturals;
/// let v: Vec<u64> = naturals().take(5).collect();
/// assert_eq!(v, [0, 1, 2, 3, 4]);
/// ```
pub fn naturals() -> impl Iterator<Item = u64> {
    0..
}

/// Returns an infinite iterator of Fibonacci numbers.
///
/// Produces the sequence 0, 1, 1, 2, 3, 5, 8, 13, …
///
/// # Examples
/// ```
/// use example_089_lazy_sequences::fibs;
/// let v: Vec<u64> = fibs().take(8).collect();
/// assert_eq!(v, [0, 1, 1, 2, 3, 5, 8, 13]);
/// ```
pub fn fibs() -> impl Iterator<Item = u64> {
    std::iter::successors(Some((0u64, 1u64)), |&(a, b)| Some((b, a + b))).map(|(a, _)| a)
}

/// Builds an iterator from a stateful function, mirroring OCaml's `from_fn`.
///
/// The closure receives the current index (starting at 0). Returning `None`
/// terminates the sequence; returning `Some(v)` yields `v` and advances the index.
///
/// # Examples
/// ```
/// use example_089_lazy_sequences::from_fn_indexed;
/// let powers: Vec<u64> = from_fn_indexed(|n| if n < 10 { Some(1u64 << n) } else { None }).collect();
/// assert_eq!(powers[..4], [1, 2, 4, 8]);
/// ```
pub fn from_fn_indexed<T, F>(f: F) -> impl Iterator<Item = T>
where
    F: FnMut(usize) -> Option<T>,
{
    let mut index = 0usize;
    let mut f = f;
    std::iter::from_fn(move || {
        let result = f(index)?;
        index += 1;
        Some(result)
    })
}

/// Returns the first `n` elements of an iterator as a `Vec`.
///
/// Convenience wrapper around `.take(n).collect()`.
///
/// # Examples
/// ```
/// use example_089_lazy_sequences::{naturals, take};
/// assert_eq!(take(3, naturals()), vec![0, 1, 2]);
/// ```
pub fn take<T>(n: usize, iter: impl Iterator<Item = T>) -> Vec<T> {
    iter.take(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naturals_first_five() {
        assert_eq!(take(5, naturals()), [0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_naturals_zero_elements() {
        assert_eq!(take(0, naturals()), [] as [u64; 0]);
    }

    #[test]
    fn test_naturals_one_element() {
        assert_eq!(take(1, naturals()), [0]);
    }

    #[test]
    fn test_fibs_first_eight() {
        assert_eq!(take(8, fibs()), [0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_fibs_first_one() {
        assert_eq!(take(1, fibs()), [0]);
    }

    #[test]
    fn test_fibs_first_two() {
        assert_eq!(take(2, fibs()), [0, 1]);
    }

    #[test]
    fn test_from_fn_indexed_powers_of_2() {
        let v: Vec<u64> =
            from_fn_indexed(|n| if n >= 10 { None } else { Some(1u64 << n) }).collect();
        assert_eq!(v.len(), 10);
        assert_eq!(&v[..4], [1, 2, 4, 8]);
        assert_eq!(v[9], 512);
    }

    #[test]
    fn test_from_fn_indexed_terminates() {
        let v: Vec<u32> =
            from_fn_indexed(|n| if n < 4 { Some(n as u32 * 3) } else { None }).collect();
        assert_eq!(v, [0, 3, 6, 9]);
    }

    #[test]
    fn test_from_fn_indexed_empty() {
        let v: Vec<u32> = from_fn_indexed(|_| None).collect();
        assert!(v.is_empty());
    }

    #[test]
    fn test_take_helper() {
        assert_eq!(take(4, naturals()), [0, 1, 2, 3]);
    }

    #[test]
    fn test_naturals_laziness() {
        // Confirms that only the requested elements are produced.
        let mut count = 0usize;
        let _: Vec<u64> = std::iter::from_fn(|| {
            count += 1;
            Some(count as u64)
        })
        .take(3)
        .collect();
        assert_eq!(count, 3);
    }
}
