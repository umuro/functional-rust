#![allow(clippy::all)]
/// Perfect Numbers — Classification
///
/// Ownership: All values are Copy integers and enums. No heap allocation.

#[derive(Debug, PartialEq)]
pub enum Classification {
    Perfect,
    Abundant,
    Deficient,
    Invalid,
}

pub fn sum_of_divisors(n: u64) -> u64 {
    (1..n).filter(|&d| n.is_multiple_of(d)).sum()
}

pub fn classify(n: u64) -> Classification {
    if n == 0 {
        return Classification::Invalid;
    }
    let s = sum_of_divisors(n);
    match s.cmp(&n) {
        std::cmp::Ordering::Equal => Classification::Perfect,
        std::cmp::Ordering::Greater => Classification::Abundant,
        std::cmp::Ordering::Less => Classification::Deficient,
    }
}

/// Version 2: Optimized — only check up to sqrt(n)
pub fn sum_of_divisors_fast(n: u64) -> u64 {
    if n <= 1 {
        return if n == 1 { 0 } else { 0 };
    }
    let mut sum = 1u64; // 1 is always a proper divisor for n > 1
    let mut i = 2;
    while i * i <= n {
        if n.is_multiple_of(i) {
            sum += i;
            if i != n / i {
                sum += n / i;
            }
        }
        i += 1;
    }
    sum
}

/// Version 3: Iterator with flat_map for divisor pairs
pub fn sum_of_divisors_iter(n: u64) -> u64 {
    if n <= 1 {
        return if n == 1 { 0 } else { 0 };
    }
    (2..)
        .take_while(|&i| i * i <= n)
        .flat_map(|i| {
            if n.is_multiple_of(i) {
                if i == n / i {
                    vec![i]
                } else {
                    vec![i, n / i]
                }
            } else {
                vec![]
            }
        })
        .sum::<u64>()
        + 1 // 1 is always a divisor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_6() {
        assert_eq!(classify(6), Classification::Perfect);
    }

    #[test]
    fn test_perfect_28() {
        assert_eq!(classify(28), Classification::Perfect);
    }

    #[test]
    fn test_abundant() {
        assert_eq!(classify(12), Classification::Abundant);
    }

    #[test]
    fn test_deficient() {
        assert_eq!(classify(7), Classification::Deficient);
    }

    #[test]
    fn test_zero() {
        assert_eq!(classify(0), Classification::Invalid);
    }

    #[test]
    fn test_one() {
        assert_eq!(classify(1), Classification::Deficient);
    }

    #[test]
    fn test_fast_matches_naive() {
        for n in 1..=1000 {
            assert_eq!(
                sum_of_divisors(n),
                sum_of_divisors_fast(n),
                "mismatch at n={}",
                n
            );
        }
    }
}
