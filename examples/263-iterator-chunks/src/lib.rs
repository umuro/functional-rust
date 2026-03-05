//! 263. Fixed-size chunks iteration
//!
//! `chunks(n)` splits a slice into non-overlapping sub-slices of at most n elements.

#[cfg(test)]
mod tests {
    #[test]
    fn test_chunks_basic() {
        let data = [1i32, 2, 3, 4, 5];
        let chunks: Vec<&[i32]> = data.chunks(2).collect();
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], &[1, 2]);
        assert_eq!(chunks[2], &[5]);
    }

    #[test]
    fn test_chunks_exact_remainder() {
        let data = [1i32, 2, 3, 4, 5];
        let exact = data.chunks_exact(2);
        assert_eq!(exact.remainder(), &[5]);
    }

    #[test]
    fn test_chunks_divisible() {
        let data = [1i32, 2, 3, 4];
        let chunks: Vec<_> = data.chunks(2).collect();
        assert_eq!(chunks.len(), 2);
        assert!(chunks.iter().all(|c| c.len() == 2));
    }
}
