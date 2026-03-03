/// Difference of Squares
///
/// Ownership: All values are Copy integers. No ownership concerns.

/// Square of the sum of first n natural numbers
pub fn square_of_sum(n: u64) -> u64 {
    let s: u64 = (1..=n).sum();
    s * s
}

/// Sum of the squares of first n natural numbers
pub fn sum_of_squares(n: u64) -> u64 {
    (1..=n).map(|x| x * x).sum()
}

/// Difference
pub fn difference(n: u64) -> u64 {
    square_of_sum(n) - sum_of_squares(n)
}

/// Version 2: Using closed-form formulas (O(1))
pub fn square_of_sum_formula(n: u64) -> u64 {
    let s = n * (n + 1) / 2;
    s * s
}

pub fn sum_of_squares_formula(n: u64) -> u64 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference_formula(n: u64) -> u64 {
    square_of_sum_formula(n) - sum_of_squares_formula(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_of_sum() {
        assert_eq!(square_of_sum(10), 3025);
    }

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(10), 385);
    }

    #[test]
    fn test_difference() {
        assert_eq!(difference(10), 2640);
    }

    #[test]
    fn test_one() {
        assert_eq!(difference(1), 0);
    }

    #[test]
    fn test_formula_matches() {
        for n in 1..=100 {
            assert_eq!(difference(n), difference_formula(n));
        }
    }

    #[test]
    fn test_large() {
        assert_eq!(difference_formula(100), 25164150);
    }
}

fn main() {
    println!("{:?}", square_of_sum(10), 3025);
    println!("{:?}", sum_of_squares(10), 385);
    println!("{:?}", difference(10), 2640);
}
