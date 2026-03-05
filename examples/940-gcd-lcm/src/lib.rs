/// # GCD and LCM — Euclidean Algorithm
///
/// Classic recursive algorithm, beautifully concise in both OCaml and Rust.

/// Recursive GCD using Euclid's algorithm.
/// Rust's pattern matching and tail recursion make this elegant.
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// LCM using the GCD identity: lcm(a,b) = |a*b| / gcd(a,b)
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b // divide first to avoid overflow
    }
}

/// GCD of a list using fold
pub fn gcd_list(nums: &[u64]) -> u64 {
    nums.iter().copied().reduce(gcd).unwrap_or(0)
}

/// LCM of a list
pub fn lcm_list(nums: &[u64]) -> u64 {
    nums.iter().copied().reduce(lcm).unwrap_or(0)
}

/// Iterative GCD (avoids potential stack overflow for very large inputs)
pub fn gcd_iterative(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(100, 75), 25);
        assert_eq!(gcd(17, 13), 1); // coprime
    }

    #[test]
    fn test_gcd_with_zero() {
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(4, 6), 12);
    }

    #[test]
    fn test_lcm_zero() {
        assert_eq!(lcm(0, 5), 0);
    }

    #[test]
    fn test_gcd_list() {
        assert_eq!(gcd_list(&[48, 36, 60, 12]), 12);
    }

    #[test]
    fn test_gcd_list_empty() {
        assert_eq!(gcd_list(&[]), 0);
    }

    #[test]
    fn test_lcm_list() {
        assert_eq!(lcm_list(&[2, 3, 4, 5]), 60);
    }

    #[test]
    fn test_iterative_matches_recursive() {
        assert_eq!(gcd(48, 18), gcd_iterative(48, 18));
        assert_eq!(gcd(100, 75), gcd_iterative(100, 75));
    }
}
