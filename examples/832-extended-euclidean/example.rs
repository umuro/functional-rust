/// Extended Euclidean Algorithm.
///
/// Returns (g, x, y) where a*x + b*y = g = gcd(a, b).
/// These are Bézout coefficients — the key to modular inverse.

/// Recursive — mirrors the OCaml one-liner elegantly.
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        // Recurrence: b*x + (a%b)*y = g  →  a*y + b*(x - a/b*y) = g
        (g, y, x - (a / b) * y)
    }
}

/// Iterative version — avoids stack overflow for very large inputs.
fn extended_gcd_iter(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1i64, 0i64);
    let (mut old_t, mut t) = (0i64, 1i64);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }
    (old_r, old_s, old_t)
}

/// Modular inverse of a mod m. Returns None if gcd(a,m) ≠ 1.
fn mod_inv(a: i64, m: i64) -> Option<i64> {
    let a = ((a % m) + m) % m;
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 { return None; }
    Some(((x % m) + m) % m)
}

/// Solve linear Diophantine: ax + by = c.
/// Returns None if gcd(a,b) ∤ c, else Some((x0, y0, step_x, step_y)).
/// General solution: (x0 + k*step_x, y0 - k*step_y) for any integer k.
fn solve_diophantine(a: i64, b: i64, c: i64) -> Option<(i64, i64, i64, i64)> {
    let (g, x0, y0) = extended_gcd(a, b);
    if c % g != 0 { return None; }
    let scale = c / g;
    Some((x0 * scale, y0 * scale, b / g, a / g))
}

fn main() {
    // Show Bézout coefficients
    for (a, b) in [(35i64, 15), (48, 18), (101, 103)] {
        let (g, x, y) = extended_gcd(a, b);
        println!("gcd({a},{b}) = {g}: {a}×{x} + {b}×{y} = {g}  ✓ {}", a*x + b*y == g);
    }

    println!("\nModular inverses:");
    for (a, m) in [(3i64, 7), (10, 17), (2, 4)] {
        match mod_inv(a, m) {
            Some(inv) => println!("  inv({a}, {m}) = {inv}  (check: {a}×{inv} mod {m} = {})", (a * inv) % m),
            None => println!("  inv({a}, {m}): no inverse (gcd ≠ 1)"),
        }
    }

    println!("\nDiophantine 3x + 5y = 1:");
    if let Some((x, y, dx, dy)) = solve_diophantine(3, 5, 1) {
        println!("  Particular: ({x}, {y})  General: ({x}+{dx}k, {y}-{dy}k)");
        // Verify
        println!("  Check: 3×{x} + 5×{y} = {}", 3*x + 5*y);
    }

    // Compare recursive and iterative
    println!("\nRecursive vs iterative:");
    let (g1, x1, y1) = extended_gcd(120, 23);
    let (g2, x2, y2) = extended_gcd_iter(120, 23);
    println!("  recursive: gcd={}  {}*120 + {}*23 = {}", g1, x1, y1, 120*x1+23*y1);
    println!("  iterative: gcd={}  {}*120 + {}*23 = {}", g2, x2, y2, 120*x2+23*y2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bezout_35_15() {
        let (g, x, y) = extended_gcd(35, 15);
        assert_eq!(g, 5);
        assert_eq!(35 * x + 15 * y, 5);
    }

    #[test]
    fn test_bezout_48_18() {
        let (g, x, y) = extended_gcd(48, 18);
        assert_eq!(g, 6);
        assert_eq!(48 * x + 18 * y, 6);
    }

    #[test]
    fn test_coprime() {
        let (g, x, y) = extended_gcd(101, 103);
        assert_eq!(g, 1);
        assert_eq!(101 * x + 103 * y, 1);
    }

    #[test]
    fn test_iterative_matches_recursive() {
        for a in -20i64..=20 {
            for b in -20i64..=20 {
                if a == 0 && b == 0 { continue; }
                let (g1, x1, y1) = extended_gcd(a.abs(), b.abs());
                let (g2, x2, y2) = extended_gcd_iter(a.abs(), b.abs());
                assert_eq!(g1, g2, "gcd mismatch ({a},{b})");
                assert_eq!(a.abs() * x1 + b.abs() * y1, g1);
                assert_eq!(a.abs() * x2 + b.abs() * y2, g2);
            }
        }
    }

    #[test]
    fn test_mod_inv() {
        assert_eq!(mod_inv(3, 7), Some(5));   // 3*5=15≡1
        assert_eq!(mod_inv(10, 17), Some(12)); // 10*12=120≡1
        assert_eq!(mod_inv(2, 4), None);       // gcd(2,4)=2
    }

    #[test]
    fn test_diophantine_solution() {
        let (x, y, _, _) = solve_diophantine(3, 5, 1).unwrap();
        assert_eq!(3 * x + 5 * y, 1);
    }

    #[test]
    fn test_diophantine_no_solution() {
        assert!(solve_diophantine(2, 4, 3).is_none()); // gcd(2,4)=2, 2∤3
    }
}
