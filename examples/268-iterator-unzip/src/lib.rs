//! 268. Splitting pairs with unzip()
//!
//! `unzip()` splits an iterator of `(A, B)` tuples into two separate collections.

#[cfg(test)]
mod tests {
    #[test]
    fn test_unzip_basic() {
        let pairs = vec![(1i32, 'a'), (2, 'b'), (3, 'c')];
        let (nums, chars): (Vec<i32>, Vec<char>) = pairs.into_iter().unzip();
        assert_eq!(nums, vec![1, 2, 3]);
        assert_eq!(chars, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_unzip_roundtrip() {
        let a = vec![1i32, 2, 3];
        let b = vec![4i32, 5, 6];
        let (a2, b2): (Vec<i32>, Vec<i32>) =
            a.iter().copied().zip(b.iter().copied()).unzip();
        assert_eq!(a, a2);
        assert_eq!(b, b2);
    }

    #[test]
    fn test_unzip_empty() {
        let empty: Vec<(i32, i32)> = vec![];
        let (a, b): (Vec<i32>, Vec<i32>) = empty.into_iter().unzip();
        assert!(a.is_empty() && b.is_empty());
    }
}
