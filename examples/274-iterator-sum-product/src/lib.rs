#![allow(clippy::all)]
//! 274. Numeric reductions: sum() and product()
//!
//! `sum()` and `product()` fold iterators of numbers with + and * respectively.

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum_gauss() {
        let sum: i32 = (1..=100).sum();
        assert_eq!(sum, 5050);
    }

    #[test]
    fn test_product_factorial() {
        let fact5: u64 = (1u64..=5).product();
        assert_eq!(fact5, 120);
    }

    #[test]
    fn test_sum_empty() {
        let s: i32 = Vec::<i32>::new().into_iter().sum();
        assert_eq!(s, 0);
    }

    #[test]
    fn test_product_empty() {
        let p: i32 = Vec::<i32>::new().into_iter().product();
        assert_eq!(p, 1); // identity element
    }
}
