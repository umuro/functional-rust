#![allow(clippy::all)]
// Example 110: Cell<T> — Interior Mutability for Copy Types
//
// Cell<T> allows mutation through a shared reference (&T).
// It works only with Copy types and avoids runtime borrow-check overhead
// by never handing out references to the interior — values are only moved
// in and out with `set` / `get`.

use std::cell::Cell;

// ── Approach 1: Simple counter ────────────────────────────────────────────────
//
// Mirrors OCaml `let counter = ref 0`.  Here the binding is immutable (`let`),
// yet `Cell::set` can still update the interior value.  The key insight:
// `Cell` wraps the mutation, so the *binding* never needs `mut`.

pub fn counter_demo() -> u32 {
    let counter = Cell::new(0u32);
    counter.set(counter.get() + 1);
    counter.set(counter.get() + 1);
    counter.get()
}

// ── Approach 2: Mutable field inside an otherwise-immutable struct ────────────
//
// `Config` can be shared via `&Config` (multiple callers, no `mut` required),
// yet `call_count` tracks how many times it has been used.
// This is the classic "shared-but-selectively-mutable" pattern in Rust.

pub struct Config {
    pub name: String,
    pub call_count: Cell<u32>,
}

impl Config {
    pub fn new(name: &str) -> Self {
        Config {
            name: name.to_string(),
            call_count: Cell::new(0),
        }
    }

    // Takes `&self` (shared reference) yet increments the counter.
    // Without Cell we would need `&mut self`, preventing sharing.
    pub fn use_it(&self) {
        self.call_count.set(self.call_count.get() + 1);
    }

    pub fn count(&self) -> u32 {
        self.call_count.get()
    }
}

// ── Approach 3: Lazy / cached computation ────────────────────────────────────
//
// Mirrors the OCaml pattern of storing `None` initially and replacing with
// `Some(computed)` on first access.  `Cell<Option<T>>` works here because
// `Option<T>` is `Copy` when `T: Copy`.

pub struct CachedSquare {
    input: i32,
    cache: Cell<Option<i32>>,
}

impl CachedSquare {
    pub fn new(input: i32) -> Self {
        CachedSquare {
            input,
            cache: Cell::new(None),
        }
    }

    // Expensive computation (simulated).  Result is stored on first call.
    pub fn get(&self) -> i32 {
        match self.cache.get() {
            Some(v) => v,
            None => {
                let v = self.input * self.input;
                self.cache.set(Some(v));
                v
            }
        }
    }
}

// ── Approach 4: Cell as a flag (bool) ─────────────────────────────────────────
//
// `bool` is Copy, so `Cell<bool>` is a lightweight, non-atomic toggle.
// Useful for visited flags in traversal, or once-only guards, without
// the overhead of a Mutex.

pub fn toggle_demo() -> (bool, bool) {
    let flag = Cell::new(false);
    let before = flag.get();
    flag.set(!flag.get());
    let after = flag.get();
    (before, after)
}

// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_increments_via_shared_ref() {
        // Cell::set works through &Cell<T> — no `mut` binding needed.
        let c = Cell::new(0u32);
        let r = &c; // shared reference
        r.set(r.get() + 10);
        r.set(r.get() + 5);
        assert_eq!(c.get(), 15);
    }

    #[test]
    fn test_counter_demo_returns_two() {
        assert_eq!(counter_demo(), 2);
    }

    #[test]
    fn test_config_call_count_through_shared_ref() {
        let cfg = Config::new("test");
        let r1 = &cfg;
        let r2 = &cfg; // two shared refs at the same time — allowed!
        r1.use_it();
        r2.use_it();
        r1.use_it();
        assert_eq!(cfg.count(), 3);
    }

    #[test]
    fn test_config_starts_at_zero() {
        let cfg = Config::new("fresh");
        assert_eq!(cfg.count(), 0);
    }

    #[test]
    fn test_cached_square_computed_once() {
        let cs = CachedSquare::new(7);
        // First call computes, subsequent calls return cached value.
        assert_eq!(cs.get(), 49);
        assert_eq!(cs.get(), 49); // from cache
                                  // Confirm the cache cell is now Some.
        assert_eq!(cs.cache.get(), Some(49));
    }

    #[test]
    fn test_cached_square_negative_input() {
        let cs = CachedSquare::new(-4);
        assert_eq!(cs.get(), 16);
    }

    #[test]
    fn test_toggle_demo() {
        let (before, after) = toggle_demo();
        assert!(!before);
        assert!(after);
    }

    #[test]
    fn test_cell_replace() {
        // Cell::replace returns the old value — handy for swap patterns.
        let c = Cell::new(42i32);
        let old = c.replace(99);
        assert_eq!(old, 42);
        assert_eq!(c.get(), 99);
    }
}
