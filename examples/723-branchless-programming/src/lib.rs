#![allow(clippy::all)]
//! # Branchless Programming
//!
//! Demonstrates replacing conditional branches with arithmetic operations to
//! avoid branch misprediction penalties on modern out-of-order CPUs.
//!
//! The core technique: arithmetic right-shift on signed integers sign-extends
//! the MSB to all bits, producing a bitmask of 0 (if positive) or -1 (if negative).
//! This mask selects between two precomputed values without a branch.

// ── Branchless integer min/max ────────────────────────────────────────────────

/// Branchless min using arithmetic right-shift bitmask.
///
/// `(a - b) >> 63` produces 0x0000...0000 when a >= b,
/// or 0xFFFF...FFFF when a < b. ANDing `diff` with this mask
/// gives either 0 or `diff`, which we add to `b`.
///
/// **Note:** inputs must not differ by more than `i64::MAX` (i.e., both values
/// should lie in the same signed half-range) to avoid wrapping in the final add.
/// For general use, prefer `i64::min` which LLVM compiles to a CMOV.
#[inline(always)]
pub fn min_branchless(a: i64, b: i64) -> i64 {
    let diff = a.wrapping_sub(b);
    let mask = diff >> 63; // arithmetic shift: 0 or -1 (all bits set)
    b.wrapping_add(diff & mask)
}

/// Branchless max using arithmetic right-shift bitmask.
///
/// Same overflow caveat as `min_branchless`.
#[inline(always)]
pub fn max_branchless(a: i64, b: i64) -> i64 {
    let diff = a.wrapping_sub(b);
    let mask = diff >> 63;
    a.wrapping_sub(diff & mask)
}

/// Idiomatic Rust min — LLVM compiles this to a CMOV (conditional move)
/// instruction on x86-64, which is already branchless at the hardware level.
#[inline(always)]
pub fn min_idiomatic(a: i64, b: i64) -> i64 {
    a.min(b)
}

/// Idiomatic Rust max — same CMOV story as min_idiomatic.
#[inline(always)]
pub fn max_idiomatic(a: i64, b: i64) -> i64 {
    a.max(b)
}

// ── Clamp ─────────────────────────────────────────────────────────────────────

/// Branchless clamp: compose min/max without any conditional jumps.
#[inline(always)]
pub fn clamp_branchless(lo: i64, hi: i64, x: i64) -> i64 {
    min_branchless(hi, max_branchless(lo, x))
}

/// Idiomatic Rust clamp — also CMOV-friendly via LLVM.
#[inline(always)]
pub fn clamp_idiomatic(lo: i64, hi: i64, x: i64) -> i64 {
    x.clamp(lo, hi)
}

// ── Absolute value ────────────────────────────────────────────────────────────

/// Branchless absolute value using the sign-mask technique.
///
/// `mask = x >> 63` is 0 for non-negative, -1 for negative.
/// `(x + mask) ^ mask` negates x when negative, leaves it alone otherwise.
#[inline(always)]
pub fn abs_branchless(x: i64) -> i64 {
    let mask = x >> 63;
    (x + mask) ^ mask
}

/// Idiomatic Rust abs — wrapping_abs avoids UB on i64::MIN.
#[inline(always)]
pub fn abs_idiomatic(x: i64) -> i64 {
    x.wrapping_abs()
}

// ── Select without branch ─────────────────────────────────────────────────────

/// Return `a` if `cond` is true, `b` otherwise — no branch.
///
/// `cond as i64` is 0 or 1; negating gives 0 or -1 (the mask).
#[inline(always)]
pub fn select_branchless(cond: bool, a: i64, b: i64) -> i64 {
    let mask = -(cond as i64); // 0 or -1
    (a & mask) | (b & !mask)
}

// ── Sign function ─────────────────────────────────────────────────────────────

/// Returns -1, 0, or 1 depending on the sign of x, branchlessly.
#[inline(always)]
pub fn sign_branchless(x: i64) -> i64 {
    // positive: (x > 0) as i64 = 1, -(x < 0) as i64 = 0  → 1
    // negative: (x > 0) as i64 = 0, -(x < 0) as i64 = -1 → -1
    // zero:     both 0 → 0
    (x > 0) as i64 - (x < 0) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── min ───────────────────────────────────────────────────────────────────

    #[test]
    fn test_min_branchless_basic() {
        assert_eq!(min_branchless(3, 7), 3);
        assert_eq!(min_branchless(7, 3), 3);
        assert_eq!(min_branchless(5, 5), 5);
    }

    #[test]
    fn test_min_branchless_negatives() {
        assert_eq!(min_branchless(-10, -3), -10);
        assert_eq!(min_branchless(-3, -10), -10);
        assert_eq!(min_branchless(-1, 1), -1);
    }

    #[test]
    fn test_min_matches_idiomatic() {
        // Pairs must not differ by more than i64::MAX (documented limitation of
        // the bitmask technique). For arbitrary-range inputs, use min_idiomatic.
        let pairs = [
            (0_i64, 0),
            (42, -42),
            (-100, 100),
            (i64::MAX, i64::MAX - 1),
            (i64::MIN + 1, i64::MIN),
        ];
        for (a, b) in pairs {
            assert_eq!(min_branchless(a, b), min_idiomatic(a, b), "min({a}, {b})");
        }
    }

    // ── max ───────────────────────────────────────────────────────────────────

    #[test]
    fn test_max_branchless_basic() {
        assert_eq!(max_branchless(3, 7), 7);
        assert_eq!(max_branchless(7, 3), 7);
        assert_eq!(max_branchless(5, 5), 5);
    }

    #[test]
    fn test_max_matches_idiomatic() {
        // Same range restriction as min_branchless.
        let pairs = [
            (0_i64, 0),
            (42, -42),
            (i64::MAX, i64::MAX - 1),
            (i64::MIN + 1, i64::MIN),
        ];
        for (a, b) in pairs {
            assert_eq!(max_branchless(a, b), max_idiomatic(a, b), "max({a}, {b})");
        }
    }

    // ── clamp ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_clamp_within_range() {
        assert_eq!(clamp_branchless(0, 10, 5), 5);
    }

    #[test]
    fn test_clamp_below_lo() {
        assert_eq!(clamp_branchless(0, 10, -5), 0);
    }

    #[test]
    fn test_clamp_above_hi() {
        assert_eq!(clamp_branchless(0, 10, 15), 10);
    }

    #[test]
    fn test_clamp_matches_idiomatic() {
        let cases = [(-5_i64, 5, -10), (-5, 5, 0), (-5, 5, 10), (0, 0, 0)];
        for (lo, hi, x) in cases {
            assert_eq!(
                clamp_branchless(lo, hi, x),
                clamp_idiomatic(lo, hi, x),
                "clamp({lo}, {hi}, {x})"
            );
        }
    }

    // ── abs ───────────────────────────────────────────────────────────────────

    #[test]
    fn test_abs_branchless_positive() {
        assert_eq!(abs_branchless(42), 42);
    }

    #[test]
    fn test_abs_branchless_negative() {
        assert_eq!(abs_branchless(-42), 42);
    }

    #[test]
    fn test_abs_branchless_zero() {
        assert_eq!(abs_branchless(0), 0);
    }

    #[test]
    fn test_abs_matches_idiomatic() {
        for x in [-1000_i64, -1, 0, 1, 1000, i64::MAX] {
            assert_eq!(abs_branchless(x), abs_idiomatic(x), "abs({x})");
        }
    }

    // ── select ────────────────────────────────────────────────────────────────

    #[test]
    fn test_select_true() {
        assert_eq!(select_branchless(true, 100, 200), 100);
    }

    #[test]
    fn test_select_false() {
        assert_eq!(select_branchless(false, 100, 200), 200);
    }

    // ── sign ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_sign_positive() {
        assert_eq!(sign_branchless(42), 1);
    }

    #[test]
    fn test_sign_negative() {
        assert_eq!(sign_branchless(-42), -1);
    }

    #[test]
    fn test_sign_zero() {
        assert_eq!(sign_branchless(0), 0);
    }
}
