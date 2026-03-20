#![allow(clippy::all)]
// ── black_box usage ───────────────────────────────────────────────────────────

/// Without black_box, the compiler may constant-fold this entire call.
#[inline(never)] // ensures the function appears in profiler output
fn sum_squares(n: u64) -> u64 {
    (0..n).map(|i| i * i).sum()
}

// ── Hot / Cold path annotation ────────────────────────────────────────────────

/// Mark rare error-handling code as cold to keep it out of the hot path.
#[cold]
#[inline(never)]
fn handle_overflow(a: u64, b: u64) -> u64 {
    eprintln!("overflow: {} + {}", a, b);
    u64::MAX
}

fn checked_add_hot(a: u64, b: u64) -> u64 {
    // Compiler infers that the success branch is hot
    a.checked_add(b).unwrap_or_else(|| handle_overflow(a, b))
}

// ── Struct-of-Arrays (SoA) vs Array-of-Structs (AoS) ─────────────────────────

/// AoS: poor cache use when accessing only one field
#[allow(dead_code)]
struct PointAoS {
    x: f32,
    y: f32,
    z: f32,
}

/// SoA: excellent cache use — each array is contiguous
struct PointsSoA {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
}

impl PointsSoA {
    fn new(n: usize) -> Self {
        PointsSoA {
            x: (0..n).map(|i| i as f32).collect(),
            y: (0..n).map(|i| i as f32 * 2.0).collect(),
            z: (0..n).map(|i| i as f32 * 3.0).collect(),
        }
    }

    /// Only touches the `x` array — minimal cache footprint.
    fn sum_x(&self) -> f32 {
        self.x.iter().sum()
    }
}

fn aos_sum_x(points: &[PointAoS]) -> f32 {
    // Loads all 3 floats even though we only want x → cache waste
    points.iter().map(|p| p.x).sum()
}

// ── Measurement discipline ────────────────────────────────────────────────────

fn measure_ns<F: FnOnce() -> R, R>(f: F) -> (R, u128) {
    let t0 = std::time::Instant::now();
    let result = f();
    let elapsed = t0.elapsed().as_nanos();
    (result, elapsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_squares_correct() {
        // 0² + 1² + 2² + 3² = 14
        assert_eq!(sum_squares(4), 14);
        assert_eq!(sum_squares(0), 0);
    }

    #[test]
    fn checked_add_no_overflow() {
        assert_eq!(checked_add_hot(3, 4), 7);
    }

    #[test]
    fn checked_add_overflow_returns_max() {
        assert_eq!(checked_add_hot(u64::MAX, 1), u64::MAX);
    }

    #[test]
    fn soa_sum_x_correct() {
        let soa = PointsSoA::new(5); // x = [0,1,2,3,4]
        assert_eq!(soa.sum_x(), 10.0);
    }

    #[test]
    fn aos_sum_x_correct() {
        let aos: Vec<PointAoS> = (0..5u32)
            .map(|i| PointAoS {
                x: i as f32,
                y: 0.0,
                z: 0.0,
            })
            .collect();
        assert_eq!(aos_sum_x(&aos), 10.0);
    }
}
