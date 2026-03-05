//! 274. Numeric reductions: sum() and product()
//!
//! `sum()` and `product()` fold iterators of numbers with + and * respectively.
//! They're zero-cost abstractions over `fold` with the identity element built in.

/// Sum a slice of integers — idiomatic: let the iterator trait do the work.
pub fn sum_ints(nums: &[i32]) -> i32 {
    nums.iter().copied().sum()
}

/// Product of a slice of integers.
pub fn product_ints(nums: &[i32]) -> i32 {
    nums.iter().copied().product()
}

/// Factorial via a range product — no explicit identity or accumulator needed.
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Sum of squares: map + sum in one expression.
pub fn sum_of_squares(nums: &[i32]) -> i32 {
    nums.iter().map(|&x| x * x).sum()
}

/// Average of f64 prices using sum() then divide.
pub fn average(prices: &[f64]) -> Option<f64> {
    if prices.is_empty() {
        return None;
    }
    let total: f64 = prices.iter().copied().sum();
    Some(total / prices.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_empty() {
        assert_eq!(sum_ints(&[]), 0);
    }

    #[test]
    fn test_sum_typical() {
        assert_eq!(sum_ints(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_product_empty() {
        // product of empty = multiplicative identity = 1
        assert_eq!(product_ints(&[]), 1);
    }

    #[test]
    fn test_product_typical() {
        assert_eq!(product_ints(&[1, 2, 3, 4, 5]), 120);
    }

    #[test]
    fn test_factorial_zero() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_five() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_factorial_ten() {
        assert_eq!(factorial(10), 3_628_800);
    }

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(&[1, 2, 3, 4, 5]), 55);
    }

    #[test]
    fn test_average_empty() {
        assert_eq!(average(&[]), None);
    }

    #[test]
    fn test_average_prices() {
        let prices = [9.99f64, 14.50, 3.75, 22.00];
        let avg = average(&prices).unwrap();
        assert!((avg - 12.56).abs() < 0.01);
    }
}
