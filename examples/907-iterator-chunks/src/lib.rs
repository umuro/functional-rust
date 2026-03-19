#![allow(clippy::all)]
//! 263. Fixed-size chunks iteration
//!
//! `chunks(n)` splits a slice into non-overlapping sub-slices of at most n elements.
//! `chunks_exact(n)` yields only full-size chunks; the remainder is accessible separately.

/// Sum each chunk of size `n` in a slice. Returns a Vec of chunk sums.
///
/// Uses `chunks(n)` — the last chunk may be shorter if `len % n != 0`.
pub fn chunk_sums(data: &[i32], n: usize) -> Vec<i32> {
    data.chunks(n).map(|c| c.iter().sum()).collect()
}

/// Split a slice into owned Vec-of-Vecs with at most `n` elements each.
pub fn chunks_owned<T: Clone>(data: &[T], n: usize) -> Vec<Vec<T>> {
    data.chunks(n).map(<[T]>::to_vec).collect()
}

/// Return only the full chunks of size `n`, discarding any remainder.
pub fn full_chunks<T: Clone>(data: &[T], n: usize) -> Vec<Vec<T>> {
    data.chunks_exact(n).map(<[T]>::to_vec).collect()
}

/// Return the remainder after taking all full chunks of size `n`.
pub fn chunks_remainder<T>(data: &[T], n: usize) -> &[T] {
    data.chunks_exact(n).remainder()
}

/// Functional / recursive OCaml-style chunking (no std chunk helpers).
pub fn chunks_recursive<T: Clone>(data: &[T], n: usize) -> Vec<Vec<T>> {
    if data.is_empty() || n == 0 {
        return vec![];
    }
    let (head, tail) = data.split_at(n.min(data.len()));
    let mut result = vec![head.to_vec()];
    result.extend(chunks_recursive(tail, n));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_sums_even_division() {
        let data = [1, 2, 3, 4, 5, 6];
        assert_eq!(chunk_sums(&data, 3), vec![6, 15]);
    }

    #[test]
    fn test_chunk_sums_with_remainder() {
        let data = [1, 2, 3, 4, 5, 6, 7];
        // chunks: [1,2,3]=6, [4,5,6]=15, [7]=7
        assert_eq!(chunk_sums(&data, 3), vec![6, 15, 7]);
    }

    #[test]
    fn test_chunk_sums_empty() {
        assert_eq!(chunk_sums(&[], 3), Vec::<i32>::new());
    }

    #[test]
    fn test_chunks_owned_shape() {
        let data = [1, 2, 3, 4, 5];
        let result = chunks_owned(&data, 2);
        assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);
    }

    #[test]
    fn test_full_chunks_drops_remainder() {
        let data = [1, 2, 3, 4, 5, 6, 7];
        let result = full_chunks(&data, 3);
        assert_eq!(result, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_chunks_remainder() {
        let data = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(chunks_remainder(&data, 3), &[7]);
    }

    #[test]
    fn test_chunks_remainder_empty_when_evenly_divisible() {
        let data = [1, 2, 3, 4, 5, 6];
        assert_eq!(chunks_remainder(&data, 3), &[] as &[i32]);
    }

    #[test]
    fn test_chunks_recursive_matches_std() {
        let data: Vec<i32> = (1..=7).collect();
        let recursive = chunks_recursive(&data, 3);
        let std_chunks: Vec<Vec<i32>> = data.chunks(3).map(|c| c.to_vec()).collect();
        assert_eq!(recursive, std_chunks);
    }

    #[test]
    fn test_chunks_recursive_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(chunks_recursive(&empty, 3), Vec::<Vec<i32>>::new());
    }
}
