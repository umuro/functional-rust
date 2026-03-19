#![allow(clippy::manual_is_multiple_of)]
#![allow(unused_variables)]
#![allow(clippy::match_like_matches)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::char_lit_as_u8)]
#![allow(clippy::while_let_loop)]
#![allow(clippy::manual_strip)]
#![allow(clippy::useless_vec)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure)]
#![allow(unused_imports)]
#![allow(dead_code)]
// 071: GCD and LCM

// Approach 1: Recursive GCD (Euclidean)
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b).abs() / gcd(a, b)
    }
}

// Approach 2: Iterative GCD
fn gcd_iter(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// GCD/LCM of a slice
fn gcd_list(v: &[i64]) -> i64 {
    v.iter().copied().reduce(gcd).unwrap_or(0)
}

fn lcm_list(v: &[i64]) -> i64 {
    v.iter().copied().reduce(lcm).unwrap_or(1)
}

// Approach 3: Extended GCD
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 5), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(100, 0), 100);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(3, 5), 15);
        assert_eq!(lcm(0, 5), 0);
    }

    #[test]
    fn test_gcd_iter() {
        assert_eq!(gcd_iter(12, 8), 4);
        assert_eq!(gcd_iter(-12, 8), 4);
    }

    #[test]
    fn test_list_ops() {
        assert_eq!(gcd_list(&[12, 18, 24]), 6);
        assert_eq!(lcm_list(&[4, 6, 8]), 24);
    }

    #[test]
    fn test_extended_gcd() {
        let (g, x, y) = extended_gcd(35, 15);
        assert_eq!(g, 5);
        assert_eq!(35 * x + 15 * y, 5);
    }
}
