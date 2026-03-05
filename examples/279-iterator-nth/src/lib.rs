//! 279. Random access with nth()
//!
//! `nth(n)` returns `Option<T>` at index n, consuming elements 0..n in the process.


#[cfg(test)]
mod tests {
    #[test]
    fn test_nth_basic() {
        let v = [10i32, 20, 30, 40];
        assert_eq!(v.iter().nth(2), Some(&30));
    }

    #[test]
    fn test_nth_out_of_bounds() {
        let v = [1i32, 2];
        assert_eq!(v.iter().nth(5), None);
    }

    #[test]
    fn test_nth_advances_iterator() {
        let mut it = [1i32, 2, 3, 4, 5].iter();
        assert_eq!(it.nth(1), Some(&2)); // consumes 1,2
        assert_eq!(it.nth(0), Some(&3)); // now at 3
    }

    #[test]
    fn test_nth_zero() {
        let v = [99i32];
        assert_eq!(v.iter().nth(0), Some(&99));
    }
}
