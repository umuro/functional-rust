#![allow(clippy::all)]
//! 256. Chaining iterators with chain()
//!
//! `chain()` concatenates two iterators lazily — no allocation, just composition.

#[cfg(test)]
mod tests {
    #[test]
    fn test_chain_basic() {
        let a = [1i32, 2, 3];
        let b = [4i32, 5, 6];
        let result: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_chain_empty() {
        let a: Vec<i32> = vec![];
        let b = vec![1, 2];
        let result: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_chain_count() {
        let a = [1i32, 2, 3];
        let b = [4i32, 5];
        assert_eq!(a.iter().chain(b.iter()).count(), 5);
    }
}
