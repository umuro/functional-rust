#![allow(clippy::all)]
//! 257. Pairing elements with zip()
//!
//! `zip()` pairs elements from two iterators, stopping at the shorter one.

#[cfg(test)]
mod tests {
    #[test]
    fn test_zip_basic() {
        let a = [1i32, 2, 3];
        let b = ["x", "y", "z"];
        let result: Vec<_> = a.iter().zip(b.iter()).collect();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], (&1, &"x"));
    }

    #[test]
    fn test_zip_truncates() {
        let a = [1i32, 2, 3, 4, 5];
        let b = [10i32, 20];
        let result: Vec<_> = a.iter().zip(b.iter()).collect();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_zip_into_hashmap() {
        let keys = vec!["a", "b"];
        let vals = vec![1i32, 2];
        let map: std::collections::HashMap<_, _> = keys.into_iter().zip(vals).collect();
        assert_eq!(map["a"], 1);
        assert_eq!(map["b"], 2);
    }
}
