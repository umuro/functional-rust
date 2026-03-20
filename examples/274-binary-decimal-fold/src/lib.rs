#![allow(clippy::all)]
/// Solution 1: Idiomatic Rust — fold over chars, propagate errors
pub fn binary_to_decimal(s: &str) -> Result<u64, String> {
    s.chars().try_fold(0u64, |acc, c| match c {
        '0' => Ok(acc * 2),
        '1' => Ok(acc * 2 + 1),
        _ => Err(format!("invalid binary digit: {c}")),
    })
}

/// Solution 2: Recursive — mirrors the OCaml `go` helper
pub fn decimal_to_binary(n: u64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    fn go(n: u64, acc: String) -> String {
        if n == 0 {
            acc
        } else {
            go(n / 2, format!("{}{}", n % 2, acc))
        }
    }
    go(n, String::new())
}

/// Solution 3: Idiomatic — build binary string with iterators (no recursion)
pub fn decimal_to_binary_iter(n: u64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut bits = Vec::new();
    let mut x = n;
    while x > 0 {
        bits.push((x % 2) as u8);
        x /= 2;
    }
    bits.iter().rev().map(|b| b.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_to_decimal_basic() {
        assert_eq!(binary_to_decimal("1010"), Ok(10));
        assert_eq!(binary_to_decimal("11111"), Ok(31));
    }

    #[test]
    fn test_binary_to_decimal_zero() {
        assert_eq!(binary_to_decimal("0"), Ok(0));
        assert_eq!(binary_to_decimal("0000"), Ok(0));
    }

    #[test]
    fn test_binary_to_decimal_invalid() {
        assert!(binary_to_decimal("1012").is_err());
        assert!(binary_to_decimal("10a0").is_err());
    }

    #[test]
    fn test_decimal_to_binary_basic() {
        assert_eq!(decimal_to_binary(10), "1010");
        assert_eq!(decimal_to_binary(31), "11111");
    }

    #[test]
    fn test_decimal_to_binary_zero() {
        assert_eq!(decimal_to_binary(0), "0");
    }

    #[test]
    fn test_decimal_to_binary_iter_matches_recursive() {
        for n in [0u64, 1, 2, 10, 31, 42, 255, 1024] {
            assert_eq!(decimal_to_binary(n), decimal_to_binary_iter(n));
        }
    }

    #[test]
    fn test_roundtrip() {
        for s in ["1010", "11111", "101010"] {
            let d = binary_to_decimal(s).unwrap();
            assert_eq!(decimal_to_binary(d), s);
        }
    }
}
