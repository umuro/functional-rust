/// Returns the signed angular difference from bearing `b1` to bearing `b2`,
/// normalized to the range (-180, 180].
///
/// Bearings may be given in any range; the result is always the shortest
/// angular path from `b1` to `b2`.
///
/// # Idiomatic Rust
/// Uses `f64::rem_euclid` to normalize into [0, 360), then shifts [180, 360)
/// down to (-180, 0] so the result is always the shortest signed arc.
pub fn bearing_diff(b1: f64, b2: f64) -> f64 {
    let r = (b2 - b1).rem_euclid(360.0);
    if r >= 180.0 { r - 360.0 } else { r }
}

/// Functional / explicit-modulo version — mirrors the OCaml source directly.
///
/// Uses `%` (which preserves sign in Rust, just like `mod_float` in OCaml)
/// and then corrects the two out-of-range cases exactly as OCaml does.
pub fn bearing_diff_explicit(b1: f64, b2: f64) -> f64 {
    let r = (b2 - b1) % 360.0;
    if r < -180.0 {
        r + 360.0
    } else if r >= 180.0 {
        r - 360.0
    } else {
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn test_basic_positive_diff() {
        // 45 - 20 = 25 degrees
        assert!(approx_eq(bearing_diff(20.0, 45.0), 25.0));
        assert!(approx_eq(bearing_diff_explicit(20.0, 45.0), 25.0));
    }

    #[test]
    fn test_negative_first_bearing() {
        // 45 - (-45) = 90
        assert!(approx_eq(bearing_diff(-45.0, 45.0), 90.0));
        assert!(approx_eq(bearing_diff_explicit(-45.0, 45.0), 90.0));
    }

    #[test]
    fn test_wraps_through_180() {
        // Shortest path from 170 → -170 is -20 (going backwards)
        assert!(approx_eq(bearing_diff(170.0, -170.0), -20.0));
        assert!(approx_eq(bearing_diff_explicit(170.0, -170.0), -20.0));
    }

    #[test]
    fn test_exactly_180_maps_to_negative() {
        // A 180-degree raw difference maps to -180.
        assert!(approx_eq(bearing_diff(0.0, 180.0), -180.0));
        assert!(approx_eq(bearing_diff_explicit(0.0, 180.0), -180.0));
    }

    #[test]
    fn test_same_bearing_zero_diff() {
        assert!(approx_eq(bearing_diff(90.0, 90.0), 0.0));
        assert!(approx_eq(bearing_diff_explicit(90.0, 90.0), 0.0));
    }

    #[test]
    fn test_fractional_bearings() {
        // From OCaml source: get_diff 29.4803 (-88.6381)
        // Expected ≈ -118.1184
        let got = bearing_diff(29.4803, -88.6381);
        assert!((got - (-118.118_4)).abs() < 1e-4, "got {got}");
    }

    #[test]
    fn test_wide_range_inputs() {
        // From OCaml source: get_diff (-70099.74233810938) 29840.67437876723
        // Expected ≈ -140.0183…  (verified against OCaml output)
        let got = bearing_diff(-70_099.742_338_109_38, 29_840.674_378_767_23);
        assert!((got - (-140.017_96)).abs() < 1e-3, "got {got}");
    }

    #[test]
    fn test_both_implementations_agree() {
        let cases: &[(f64, f64)] = &[
            (-85.0, 90.0),
            (-95.0, 90.0),
            (-45.0, 125.0),
            (-45.0, 145.0),
            (-78.3251, -159.036),
        ];
        for &(b1, b2) in cases {
            assert!(
                approx_eq(bearing_diff(b1, b2), bearing_diff_explicit(b1, b2)),
                "mismatch for ({b1}, {b2})"
            );
        }
    }
}
