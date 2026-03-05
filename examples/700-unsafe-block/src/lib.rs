//! 700 — Unsafe Blocks
//! Keep unsafe footprint minimal: only what truly needs it.

static mut GLOBAL_COUNTER: u64 = 0;

/// Increment the global counter — smallest possible unsafe block.
fn increment() {
    unsafe {
        // SAFETY: Single-threaded; no concurrent access to GLOBAL_COUNTER.
        // In multi-threaded code, use AtomicU64 instead.
        GLOBAL_COUNTER += 1;
    }
    // ← Safe code (logging, side-effects) lives OUTSIDE the unsafe block.
}

fn get() -> u64 {
    unsafe {
        // SAFETY: Same single-threaded guarantee.
        GLOBAL_COUNTER
    }
}

fn reset() {
    unsafe {
        // SAFETY: Same single-threaded guarantee.
        GLOBAL_COUNTER = 0;
    }
    // Safe operations after the minimal unsafe block
    println!("Counter reset to 0.");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_lifecycle() {
        reset();
        assert_eq!(get(), 0);
        increment();
        increment();
        assert_eq!(get(), 2);
        reset();
        assert_eq!(get(), 0);
    }

    #[test]
    fn test_safe_code_outside_unsafe() {
        // Demonstrate safe code compiles and works without unsafe
        let v = vec![1u32, 2, 3];
        assert_eq!(v.iter().sum::<u32>(), 6);
    }
}
