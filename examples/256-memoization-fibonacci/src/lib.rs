// Example 256: Memoization — Fibonacci with Hashtable Cache
//
// OCaml uses a generic `memoize` HOF that wraps any function with a Hashtbl
// cache, then wires it to a recursive definition via `let rec … and`.
//
// Rust has no `let rec … and` for closures, so we show three equivalent
// patterns ranked from most idiomatic to closest to the OCaml source.

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

// ─────────────────────────────────────────────────────────────────────────────
// Solution 1: Idiomatic Rust — struct-based memoization
//
// A struct owns the HashMap; the recursive method takes `&mut self`.
// Mutable state is explicit in the type signature — no hidden globals.
// ─────────────────────────────────────────────────────────────────────────────

/// Fibonacci calculator that accumulates a memoization cache across calls.
pub struct FibMemo {
    cache: HashMap<u64, u64>,
}

impl Default for FibMemo {
    fn default() -> Self {
        Self::new()
    }
}

impl FibMemo {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Compute fib(n), caching every intermediate result.
    ///
    /// `&mut self` signals to callers that the cache is mutated — Rust makes
    /// the side-effect visible in the type, unlike OCaml's hidden Hashtbl.
    pub fn fib(&mut self, n: u64) -> u64 {
        if let Some(&v) = self.cache.get(&n) {
            return v;
        }
        let v = if n <= 1 {
            n
        } else {
            self.fib(n - 1) + self.fib(n - 2)
        };
        self.cache.insert(n, v);
        v
    }
}

/// Convenience wrapper — fresh cache per call (for one-off queries).
pub fn fib_struct(n: u64) -> u64 {
    FibMemo::new().fib(n)
}

// ─────────────────────────────────────────────────────────────────────────────
// Solution 2: Generic memoize HOF + explicit-cache recursion
//
// OCaml: let memoize f = let cache = Hashtbl.create 16 in fun x -> …
//
// The generic `memoize` works for any pure, non-recursive function.
// For *recursive* memoization the inner function must receive the cache
// explicitly — Rust's equivalent of OCaml's `let rec … and memo_fib = …`.
// ─────────────────────────────────────────────────────────────────────────────

/// Generic memoize wrapper for non-recursive functions.
///
/// Mirrors the OCaml `memoize` HOF exactly.  The returned `FnMut` owns
/// a `RefCell<HashMap>` so it can mutate the cache on each call.
pub fn memoize<A, R, F>(f: F) -> impl FnMut(A) -> R
where
    A: Eq + Hash + Clone,
    R: Clone,
    F: Fn(A) -> R,
{
    // RefCell gives us interior mutability inside the immutable closure capture.
    let cache = RefCell::new(HashMap::new());
    move |x: A| {
        if let Some(v) = cache.borrow().get(&x).cloned() {
            return v;
        }
        // Clone `x` so we can both call `f` and use `x` as the map key.
        let v = f(x.clone());
        cache.borrow_mut().insert(x, v.clone());
        v
    }
}

/// Fibonacci using an explicit-cache helper — the Rust analogue of OCaml's
/// `let rec fib' … and memo_fib = memoize fib'`.
///
/// A named inner function (not a closure) can call itself recursively while
/// also accepting a `&RefCell<HashMap>` parameter to share the cache.
pub fn fib_hof(n: u64) -> u64 {
    let cache = RefCell::new(HashMap::<u64, u64>::new());

    fn inner(n: u64, cache: &RefCell<HashMap<u64, u64>>) -> u64 {
        if let Some(&v) = cache.borrow().get(&n) {
            return v;
        }
        let v = if n <= 1 {
            n
        } else {
            inner(n - 1, cache) + inner(n - 2, cache)
        };
        cache.borrow_mut().insert(n, v);
        v
    }

    inner(n, &cache)
}

// ─────────────────────────────────────────────────────────────────────────────
// Solution 3: Thread-local transparent memoization
//
// The closest Rust equivalent to OCaml's "transparent" memoization: callers
// invoke `fib_tl(n)` with no cache argument.  The cache lives in a
// `thread_local!` — analogous to OCaml's module-level `let cache = Hashtbl…`.
//
// Trade-off: cache persists across all calls in the thread (same as OCaml),
// but is not shared across threads (each thread gets its own copy).
// ─────────────────────────────────────────────────────────────────────────────

thread_local! {
    static FIB_CACHE: RefCell<HashMap<u64, u64>> = RefCell::new(HashMap::new());
}

/// Fibonacci with a thread-local memoization cache.
///
/// Call signature is identical to a plain recursive function — the cache is
/// completely hidden, matching OCaml's transparent memoization style.
pub fn fib_tl(n: u64) -> u64 {
    // Borrow, check, and drop the guard *before* recursing to avoid a
    // `BorrowMutError` when the recursive call tries to insert into the cache.
    if let Some(v) = FIB_CACHE.with(|c| c.borrow().get(&n).copied()) {
        return v;
    }
    let v = if n <= 1 {
        n
    } else {
        fib_tl(n - 1) + fib_tl(n - 2)
    };
    FIB_CACHE.with(|c| c.borrow_mut().insert(n, v));
    v
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── struct-based ─────────────────────────────────────────────────────────

    #[test]
    fn test_struct_base_cases() {
        let mut m = FibMemo::new();
        assert_eq!(m.fib(0), 0);
        assert_eq!(m.fib(1), 1);
    }

    #[test]
    fn test_struct_small() {
        assert_eq!(fib_struct(10), 55);
    }

    #[test]
    fn test_struct_large() {
        assert_eq!(fib_struct(35), 9_227_465);
    }

    #[test]
    fn test_struct_cache_reuse() {
        let mut m = FibMemo::new();
        // Warm the cache up to fib(20); fib(10) is then a cache hit.
        let _ = m.fib(20);
        assert_eq!(m.fib(10), 55);
    }

    // ── generic memoize HOF ──────────────────────────────────────────────────

    #[test]
    fn test_memoize_non_recursive() {
        let mut sq = memoize(|x: u64| x * x);
        assert_eq!(sq(7), 49);
        assert_eq!(sq(7), 49); // second call is a cache hit
    }

    #[test]
    fn test_hof_base_cases() {
        assert_eq!(fib_hof(0), 0);
        assert_eq!(fib_hof(1), 1);
    }

    #[test]
    fn test_hof_small() {
        assert_eq!(fib_hof(10), 55);
    }

    #[test]
    fn test_hof_large() {
        assert_eq!(fib_hof(35), 9_227_465);
    }

    // ── thread-local ─────────────────────────────────────────────────────────

    #[test]
    fn test_tl_base_cases() {
        assert_eq!(fib_tl(0), 0);
        assert_eq!(fib_tl(1), 1);
    }

    #[test]
    fn test_tl_small() {
        assert_eq!(fib_tl(10), 55);
    }

    #[test]
    fn test_tl_large() {
        assert_eq!(fib_tl(35), 9_227_465);
    }

    // ── cross-implementation agreement ───────────────────────────────────────

    #[test]
    fn test_all_implementations_agree() {
        let mut m = FibMemo::new();
        for n in 0u64..=20 {
            let expected = m.fib(n);
            assert_eq!(fib_hof(n), expected, "fib_hof({n}) mismatch");
            assert_eq!(fib_tl(n), expected, "fib_tl({n}) mismatch");
        }
    }
}
