/// Chinese Remainder Theorem (CRT).
///
/// Solves: x ≡ aᵢ (mod mᵢ) for all i.
/// Works for non-coprime moduli; returns None when no solution exists.

/// Extended GCD: returns (g, x, y) where a*x + b*y = g.
fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

/// Combine two congruences: x ≡ a1 (mod m1) and x ≡ a2 (mod m2).
/// Returns Some((remainder, lcm)) or None if incompatible.
fn crt_combine(a1: i128, m1: i128, a2: i128, m2: i128) -> Option<(i128, i128)> {
    let (g, p, _) = extended_gcd(m1, m2);
    if (a2 - a1) % g != 0 {
        return None; // No solution
    }
    let lcm = m1 / g * m2;
    let m2g = m2 / g;
    let diff = ((a2 - a1) / g) % m2g;
    let x = (a1 + m1 * ((diff * p % m2g + m2g) % m2g)) % lcm;
    let x = (x + lcm) % lcm;
    Some((x, lcm))
}

/// Solve a system of congruences.
/// Input: slice of (remainder, modulus) pairs.
fn crt(congruences: &[(i128, i128)]) -> Option<(i128, i128)> {
    congruences.iter().try_fold((0i128, 1i128), |(r, m), &(a, mi)| {
        crt_combine(r, m, a, mi)
    })
}

fn main() {
    // Classic: x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7) → x = 23 mod 105
    let system = [(2, 3), (3, 5), (2, 7)];
    match crt(&system) {
        Some((x, m)) => println!("x ≡ 2(3), x ≡ 3(5), x ≡ 2(7): x = {x} (mod {m})"),
        None => println!("No solution"),
    }

    // Non-coprime moduli with solution
    match crt(&[(0, 4), (6, 10)]) {
        Some((x, m)) => println!("x ≡ 0(4), x ≡ 6(10): x = {x} (mod {m})"),
        None => println!("No solution"),
    }

    // Non-coprime moduli with no solution
    match crt(&[(1, 4), (6, 10)]) {
        Some((x, m)) => println!("x ≡ 1(4), x ≡ 6(10): x = {x} (mod {m})"),
        None => println!("x ≡ 1(4), x ≡ 6(10): No solution"),
    }

    // Two-modulus example
    match crt(&[(1, 5), (2, 7)]) {
        Some((x, m)) => println!("x ≡ 1(5), x ≡ 2(7): x = {x} (mod {m})"),
        None => println!("No solution"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_three_congruences() {
        // x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7)
        let (x, m) = crt(&[(2, 3), (3, 5), (2, 7)]).unwrap();
        assert_eq!(x, 23);
        assert_eq!(m, 105); // 3*5*7
        assert_eq!(x % 3, 2);
        assert_eq!(x % 5, 3);
        assert_eq!(x % 7, 2);
    }

    #[test]
    fn test_two_coprime() {
        let (x, m) = crt(&[(1, 5), (2, 7)]).unwrap();
        assert_eq!(m, 35);
        assert_eq!(x % 5, 1);
        assert_eq!(x % 7, 2);
    }

    #[test]
    fn test_non_coprime_has_solution() {
        let result = crt(&[(0, 4), (6, 10)]);
        assert!(result.is_some());
        let (x, _m) = result.unwrap();
        assert_eq!(x % 4, 0);
        assert_eq!(x % 10, 6);
    }

    #[test]
    fn test_non_coprime_no_solution() {
        // gcd(4,10)=2, but 6-1=5 is not divisible by 2
        assert!(crt(&[(1, 4), (6, 10)]).is_none());
    }

    #[test]
    fn test_single_congruence() {
        let (x, m) = crt(&[(3, 7)]).unwrap();
        assert_eq!(x, 3);
        assert_eq!(m, 7);
    }

    #[test]
    fn test_solution_uniqueness() {
        // Solution should be unique mod M
        let (x, m) = crt(&[(2, 3), (3, 5)]).unwrap();
        // Verify no other solution in [0, m)
        let others: Vec<i128> = (0..m).filter(|&t| t != x && t % 3 == 2 && t % 5 == 3).collect();
        assert!(others.is_empty(), "Multiple solutions found: {:?}", others);
    }

    #[test]
    fn test_consistency_verification() {
        let system = [(2, 3), (3, 5), (2, 7)];
        let (x, _m) = crt(&system).unwrap();
        for (a, mi) in system {
            assert_eq!(x % mi, a, "x={x} fails x ≡ {a} (mod {mi})");
        }
    }
}
