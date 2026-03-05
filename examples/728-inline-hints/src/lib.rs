// 728. #[inline], #[cold], #[target_feature] hints
//
// Demonstrates how compiler hints guide LLVM without changing semantics.

use std::time::Instant;

// ── #[inline] family ──────────────────────────────────────────────────────────

/// `#[inline]` — suggest inlining across crate boundaries.
/// The compiler may still ignore this hint.
#[inline]
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// `#[inline(always)]` — force inlining. Increases binary size but enables
/// constant folding and eliminates call overhead. Use for tiny, hot functions.
#[inline(always)]
pub fn fast_abs(x: i64) -> i64 {
    // LLVM turns this into a branchless CMOV or bit-trick.
    if x < 0 { -x } else { x }
}

/// `#[inline(never)]` — prevent inlining. Useful when:
///  - You want a stable call frame for profiling
///  - The function is too large and would bloat callers
///  - You're debugging inlining decisions
#[inline(never)]
pub fn heavy_computation(data: &[i64]) -> i64 {
    data.iter().map(|&x| x * x).sum()
}

// ── #[cold] — cold path annotation ────────────────────────────────────────────

/// Mark rarely-executed code paths. LLVM will:
///  1. Place this code far from hot code (cache friendliness)
///  2. Bias branch prediction to expect the fast path
///  3. Prefer registers for the calling context, not this function
#[cold]
#[inline(never)]
fn handle_error(msg: &str) -> ! {
    // Panic/error handlers are ideal candidates for #[cold].
    panic!("Error: {msg}");
}

#[cold]
#[inline(never)]
pub fn allocation_failed(size: usize) -> ! {
    handle_error(&format!("failed to allocate {size} bytes"))
}

/// A parse function where the error branch is cold.
/// The hot path (success) gets priority register allocation.
pub fn parse_u64(s: &str) -> u64 {
    match s.parse::<u64>() {
        Ok(v)  => v,
        Err(_) => {
            // #[cold] on the closure isn't possible, but we can call a cold fn.
            // In practice, marking this inline fn cold steers LLVM.
            #[cold]
            fn fail(s: &str) -> u64 {
                eprintln!("Failed to parse {s:?} as u64, defaulting to 0");
                0
            }
            fail(s)
        }
    }
}

// ── #[target_feature] — per-function CPU features ─────────────────────────────

/// Enable SSE4.2 for this function only.
/// Other functions in the same binary are not affected.
///
/// # Safety
/// Calling this function on a CPU that doesn't support SSE4.2 will cause
/// an illegal-instruction fault (SIGILL). Always guard with runtime detection.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse4.2")]
unsafe fn sum_sse42(data: &[i32]) -> i64 {
    // SAFETY: Caller guarantees SSE4.2 is available on the current CPU.
    // We use only standard arithmetic here; in real code you'd use
    // `std::arch::x86_64::_mm_...` intrinsics.
    data.iter().map(|&x| x as i64).sum()
}

/// Safe runtime dispatch: use SSE4.2 if available, else scalar fallback.
pub fn sum_dispatch(data: &[i32]) -> i64 {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("sse4.2") {
            // SAFETY: We just checked that SSE4.2 is available on this CPU.
            return unsafe { sum_sse42(data) };
        }
    }
    // Scalar fallback for other architectures or older CPUs.
    data.iter().map(|&x| x as i64).sum()
}

// ── hint::black_box — prevent optimiser from removing benchmarked code ─────────

use std::hint::black_box;

/// Prevent the compiler from optimising away a computation during benchmarking.
/// Without `black_box`, LLVM might constant-fold the entire loop.
pub fn bench_sum(data: &[i64]) -> i64 {
    let data = black_box(data);
    data.iter().copied().sum()
}

// ── hint::spin_loop — efficient busy-wait ─────────────────────────────────────

use std::sync::atomic::{AtomicBool, Ordering};

/// Spin-wait until `flag` is set. Uses `spin_loop()` to emit
/// the PAUSE instruction on x86 (reduces power, avoids pipeline stall).
pub fn spin_until(flag: &AtomicBool) {
    while !flag.load(Ordering::Acquire) {
        std::hint::spin_loop(); // emits `PAUSE` on x86, `YIELD` on ARM
    }
}

// ── Attribute showcase: likely/unlikely via branch weights ────────────────────

/// Simulate `likely`/`unlikely` using explicit cold annotation.
/// In Rust 1.65+ you can use `#[expect]` / `core::intrinsics::likely` on nightly.
/// On stable, structure code so the cold path calls a `#[cold]` function.
pub fn classify(x: i32) -> &'static str {
    if x == 0 {
        cold_zero()   // cold path — x == 0 is rare
    } else if x > 0 {
        "positive"    // hot path — most common
    } else {
        "negative"    // warm path
    }
}

#[cold]
fn cold_zero() -> &'static str {
    "zero" // rare — cold annotation keeps this away from hot code
}

// ── main ──────────────────────────────────────────────────────────────────────


// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_add() {
        assert_eq!(add(3, 4), 7);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn inline_abs() {
        assert_eq!(fast_abs(-42), 42);
        assert_eq!(fast_abs(0), 0);
        assert_eq!(fast_abs(7), 7);
    }

    #[test]
    fn heavy_computation_sum_of_squares() {
        let data = vec![1i64, 2, 3, 4];
        assert_eq!(heavy_computation(&data), 1 + 4 + 9 + 16);
    }

    #[test]
    fn parse_u64_valid() {
        assert_eq!(parse_u64("100"), 100);
        assert_eq!(parse_u64("0"), 0);
    }

    #[test]
    fn parse_u64_invalid_returns_zero() {
        assert_eq!(parse_u64("not_a_number"), 0);
    }

    #[test]
    fn classify_values() {
        assert_eq!(classify(5), "positive");
        assert_eq!(classify(-5), "negative");
        assert_eq!(classify(0), "zero");
    }

    #[test]
    fn sum_dispatch_correct() {
        let v: Vec<i32> = (1..=10).collect();
        assert_eq!(sum_dispatch(&v), 55);
    }

    #[test]
    fn spin_loop_unblocks() {
        let flag = AtomicBool::new(false);
        std::thread::scope(|s| {
            s.spawn(|| {
                std::thread::sleep(std::time::Duration::from_millis(5));
                flag.store(true, std::sync::atomic::Ordering::Release);
            });
            spin_until(&flag);
        });
        assert!(flag.load(std::sync::atomic::Ordering::Relaxed));
    }
}
