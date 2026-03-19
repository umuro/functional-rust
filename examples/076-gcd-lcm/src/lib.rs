/// GCD using Euclidean algorithm
///
/// Ownership insight: All values are Copy (integers), so ownership
/// is trivial here. The recursive structure mirrors OCaml exactly.
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// LCM using GCD
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

/// GCD of a slice — uses fold pattern like OCaml List.fold_left
/// Ownership: slice is borrowed, no allocation needed
pub fn gcd_list(nums: &[u64]) -> u64 {
    nums.iter().copied().reduce(gcd).unwrap_or(0)
}

/// Iterator-based GCD — more idiomatic Rust
pub fn gcd_iter(nums: impl IntoIterator<Item = u64>) -> u64 {
    nums.into_iter().reduce(gcd).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(100, 75), 25);
    }

    #[test]
    fn test_gcd_zero() {
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(0, 5), 0);
    }

    #[test]
    fn test_gcd_list() {
        assert_eq!(gcd_list(&[48, 36, 60, 12]), 12);
        assert_eq!(gcd_list(&[]), 0);
    }

    #[test]
    fn test_gcd_iter() {
        assert_eq!(gcd_iter(vec![48, 36, 60, 12]), 12);
    }
}
