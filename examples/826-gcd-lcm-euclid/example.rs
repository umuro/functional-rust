/// GCD, LCM, and the Euclidean Algorithm.
///
/// gcd(a, b) = gcd(b, a % b) until b = 0.
/// Converges in O(log min(a, b)) steps.

/// Recursive GCD — mirrors OCaml's one-liner elegantly.
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Iterative GCD — no stack concerns for large inputs.
fn gcd_iter(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// LCM: divide first to prevent overflow.
fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { return 0; }
    a / gcd(a, b) * b
}

/// GCD of a slice — fold with neutral element 0.
fn gcd_slice(xs: &[u64]) -> u64 {
    xs.iter().fold(0u64, |acc, &x| gcd(acc, x))
}

/// LCM of a slice — fold with neutral element 1.
fn lcm_slice(xs: &[u64]) -> u64 {
    xs.iter().fold(1u64, |acc, &x| lcm(acc, x))
}

/// Binary GCD (Stein's algorithm) — replaces modulo with bit shifts.
fn binary_gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b { std::mem::swap(&mut a, &mut b); }
        b -= a;
        if b == 0 { break; }
    }
    a << shift
}

/// Trace the Euclidean algorithm steps.
fn gcd_trace(mut a: u64, mut b: u64) -> u64 {
    println!("Tracing gcd({a}, {b}):");
    while b != 0 {
        let r = a % b;
        println!("  gcd({a}, {b}) → remainder {r}");
        a = b;
        b = r;
    }
    println!("  = {a}");
    a
}

fn main() {
    gcd_trace(48, 18);

    println!("\ngcd(48, 18) = {} (expected 6)", gcd(48, 18));
    println!("lcm(4, 6)   = {} (expected 12)", lcm(4, 6));
    println!("lcm(12, 18) = {} (expected 36)", lcm(12, 18));
    println!("gcd_slice([12,18,24]) = {} (expected 6)", gcd_slice(&[12, 18, 24]));
    println!("lcm_slice([2,3,4])    = {} (expected 12)", lcm_slice(&[2, 3, 4]));
    println!("binary_gcd(48, 18) = {} (expected 6)", binary_gcd(48, 18));
    println!("\nLCM of 1..10 = {}", lcm_slice(&[1,2,3,4,5,6,7,8,9,10])); // 2520
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 48), 6); // Commutative
    }

    #[test]
    fn test_gcd_coprime() {
        assert_eq!(gcd(13, 7), 1);
    }

    #[test]
    fn test_gcd_same() {
        assert_eq!(gcd(12, 12), 12);
    }

    #[test]
    fn test_gcd_zero() {
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
    }

    #[test]
    fn test_lcm_basic() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(12, 18), 36);
    }

    #[test]
    fn test_lcm_coprime() {
        assert_eq!(lcm(3, 7), 21);
    }

    #[test]
    fn test_gcd_iter_matches() {
        for a in 0..50u64 {
            for b in 0..50u64 {
                assert_eq!(gcd(a, b), gcd_iter(a, b));
            }
        }
    }

    #[test]
    fn test_binary_gcd_matches() {
        for a in 0..50u64 {
            for b in 0..50u64 {
                assert_eq!(gcd(a, b), binary_gcd(a, b),
                    "binary_gcd({a}, {b}) mismatch");
            }
        }
    }

    #[test]
    fn test_gcd_slice() {
        assert_eq!(gcd_slice(&[12, 18, 24]), 6);
        assert_eq!(gcd_slice(&[7, 14, 21]), 7);
    }

    #[test]
    fn test_lcm_slice() {
        assert_eq!(lcm_slice(&[2, 3, 4]), 12);
        assert_eq!(lcm_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 2520);
    }
}
