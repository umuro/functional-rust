//! Averages/Mean time of day
//!
//! Computes the mean time of day from a list of times using circular/angular
//! averaging.  Because time wraps around midnight, a simple arithmetic mean
//! fails (e.g. the mean of 23:00 and 01:00 would wrongly be 12:00).  The
//! correct approach maps each time onto the unit circle, averages the angles,
//! and converts back.

use std::f64::consts::PI;

/// Seconds in a full day.
const DAY: f64 = 86_400.0; // 24 * 60 * 60

// ── Conversion helpers ────────────────────────────────────────────────────────

/// Map seconds-since-midnight to an angle in [0, 2π).
pub fn rad_of_time(t: f64) -> f64 {
    t * 2.0 * PI / DAY
}

/// Map an angle back to seconds-since-midnight.
pub fn time_of_rad(r: f64) -> f64 {
    r * DAY / (2.0 * PI)
}

// ── Core algorithm ────────────────────────────────────────────────────────────

/// Compute the circular mean of a slice of angles (radians).
///
/// Uses `atan2(Σsin θ, Σcos θ)`.  This is scale-invariant so we do not
/// divide by n — the direction of the resultant vector is what matters.
pub fn mean_angle(angles: &[f64]) -> f64 {
    let sum_sin: f64 = angles.iter().map(|a| a.sin()).sum();
    let sum_cos: f64 = angles.iter().map(|a| a.cos()).sum();
    // Rust: y.atan2(x)  ≡  OCaml: atan2 y x
    sum_sin.atan2(sum_cos)
}

/// Compute the circular mean of a slice of times (seconds since midnight).
///
/// Returns seconds in `[0, DAY)`.  Handles the midnight-wraparound problem
/// by working in angle space.
pub fn mean_time(times: &[f64]) -> f64 {
    let angles: Vec<f64> = times.iter().map(|&t| rad_of_time(t)).collect();
    let t = time_of_rad(mean_angle(&angles));
    // atan2 returns values in (-π, π]; a negative result means "just before
    // midnight", so we shift by one full day to land in [0, DAY).
    if t < 0.0 {
        t + DAY
    } else {
        t
    }
}

// ── Parse / Format ────────────────────────────────────────────────────────────

/// Parse `"HH:MM:SS"` into seconds since midnight.
///
/// Returns `None` if the string is not in the expected format.
pub fn parse_time(s: &str) -> Option<f64> {
    let mut parts = s.split(':');
    let h: i32 = parts.next()?.parse().ok()?;
    let m: i32 = parts.next()?.parse().ok()?;
    let sec: i32 = parts.next()?.parse().ok()?;
    // Reject strings with extra colon-separated components.
    if parts.next().is_some() {
        return None;
    }
    Some((h * 3600 + m * 60 + sec) as f64)
}

/// Format seconds since midnight as `"H:M:S"` (no zero-padding, matching
/// the OCaml reference output).
pub fn format_time(t: f64) -> String {
    // Round to the nearest second (mirrors OCaml's `floor (x +. 0.5)`).
    let t = (t + 0.5).floor() as u64;
    let h = t / 3600;
    let rem = t % 3600;
    let m = rem / 60;
    let s = rem % 60;
    format!("{h}:{m}:{s}")
}

// ── Idiomatic high-level entry point ─────────────────────────────────────────

/// Parse, average, and format in one iterator chain.
///
/// Returns `None` if any time string fails to parse.
pub fn mean_time_of_strings(times: &[&str]) -> Option<String> {
    let parsed: Vec<f64> = times
        .iter()
        .map(|t| parse_time(t))
        .collect::<Option<Vec<_>>>()?;
    Some(format_time(mean_time(&parsed)))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time() {
        assert_eq!(parse_time("00:00:00"), Some(0.0));
        assert_eq!(parse_time("01:00:00"), Some(3600.0));
        assert_eq!(parse_time("23:59:59"), Some(86399.0));
        assert_eq!(parse_time("invalid"), None);
        assert_eq!(parse_time("01:00:00:00"), None); // extra component
    }

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(0.0), "0:0:0");
        assert_eq!(format_time(3600.0), "1:0:0");
        assert_eq!(format_time(3661.0), "1:1:1");
        assert_eq!(format_time(86399.0), "23:59:59");
    }

    #[test]
    fn test_single_time_round_trips() {
        // The mean of a single time must equal that time (within rounding).
        let t = parse_time("12:30:00").unwrap();
        assert_eq!(format_time(mean_time(&[t])), "12:30:0");

        let t2 = parse_time("00:00:00").unwrap();
        assert_eq!(format_time(mean_time(&[t2])), "0:0:0");
    }

    #[test]
    fn test_symmetry_around_midnight() {
        // 23:00:00 and 01:00:00 are equidistant from midnight — mean = 00:00:00.
        let t1 = parse_time("23:00:00").unwrap();
        let t2 = parse_time("01:00:00").unwrap();
        assert_eq!(format_time(mean_time(&[t1, t2])), "0:0:0");
    }

    #[test]
    fn test_rosetta_code_example() {
        // Classic Rosetta Code test case — all times cluster near midnight.
        let result = mean_time_of_strings(&["23:00:17", "23:40:20", "00:12:45", "00:17:19"]);
        assert!(result.is_some());
        let s = result.unwrap();
        // Expected: somewhere in the 23:47:xx range.
        assert!(s.starts_with("23:47:"), "Expected mean ≈ 23:47:xx, got {s}");
    }

    #[test]
    fn test_invalid_time_strings() {
        assert!(mean_time_of_strings(&["12:00:00", "bad"]).is_none());
    }
}
