//! 714 — FFI Callbacks: Passing Rust Functions to C
//!
//! Two patterns for crossing the C ABI boundary with callables:
//!
//! 1. **Plain function pointer** — a bare `extern "C" fn(...)` coerces
//!    directly from a Rust function.  No captures, no `unsafe`, no overhead.
//!
//! 2. **Trampoline pattern** — a closure with captured state is split into:
//!    - a thin `extern "C"` wrapper function (a stable address C can call), and
//!    - a `*mut c_void` user-data pointer (carries the closure's state).
//!
//!    The wrapper reconstructs `&mut Closure` from the pointer and calls it.
//!
//! The trampoline appears in `pthread_create`, `qsort_r`, GTK signal handlers,
//! and every event-driven C API that accepts a `(callback, user_data)` pair.

use std::os::raw::c_void;

// ── Simulated C APIs ──────────────────────────────────────────────────────
// Real C functions would live in an `extern "C" { ... }` block and be
// linked from a compiled library.  We implement them here with the C calling
// convention so the example is fully self-contained and testable.

/// C-style for-each: calls `callback(elem)` for every element in `data`.
///
/// Equivalent C declaration:
/// `void sim_for_each(const int *data, size_t len, void (*callback)(int));`
pub fn sim_for_each(data: &[i32], callback: extern "C" fn(i32)) {
    for &v in data {
        callback(v);
    }
}

/// C-style left-fold: combines elements with `f`, starting from `init`.
///
/// `int sim_reduce(const int*, size_t, int (*f)(int, int), int init);`
pub fn sim_reduce(data: &[i32], init: i32, f: extern "C" fn(i32, i32) -> i32) -> i32 {
    data.iter().fold(init, |acc, &v| f(acc, v))
}

/// C-style for-each with user-data context (the "trampoline API").
///
/// `void sim_for_each_ctx(const int*, size_t, void (*)(void*, int), void*);`
pub fn sim_for_each_ctx(
    data: &[i32],
    callback: extern "C" fn(*mut c_void, i32),
    user_data: *mut c_void,
) {
    for &v in data {
        callback(user_data, v);
    }
}

/// C-style left-fold with user-data context.
pub fn sim_reduce_ctx(
    data: &[i32],
    init: i32,
    f: extern "C" fn(*mut c_void, i32, i32) -> i32,
    user_data: *mut c_void,
) -> i32 {
    data.iter().fold(init, |acc, &v| f(user_data, acc, v))
}

// ── Plain extern "C" functions (Pattern 1) ────────────────────────────────
// A Rust function declared `extern "C"` adopts the C calling convention and
// coerces to the matching `extern "C" fn(...)` type with no casting needed.

/// Sum accumulator — coerces to `extern "C" fn(i32, i32) -> i32`.
pub extern "C" fn add(acc: i32, v: i32) -> i32 {
    acc + v
}

/// Product accumulator — coerces to `extern "C" fn(i32, i32) -> i32`.
pub extern "C" fn mul(acc: i32, v: i32) -> i32 {
    acc * v
}

/// Max accumulator — coerces to `extern "C" fn(i32, i32) -> i32`.
pub extern "C" fn max_of(acc: i32, v: i32) -> i32 {
    acc.max(v)
}

// ── Trampoline pattern (Pattern 2) ────────────────────────────────────────
// A closure is a compiler-generated struct; it has no stable ABI and cannot
// be represented as a C function pointer.  The trampoline splits it into
// a plain function (an address) and a `*mut c_void` (the captured state).

/// Apply a Rust closure to every element, hiding the trampoline internals.
///
/// The closure `f` lives on the stack; its address is cast to `*mut c_void`
/// for the C side.  The inner `trampoline` fn recovers the reference and
/// calls the closure.  The pointer does not escape this function.
pub fn for_each_with_closure<F>(data: &[i32], mut f: F)
where
    F: FnMut(i32),
{
    // Trampoline: a plain `extern "C"` fn that reconstructs `&mut F` from
    // the opaque user-data pointer and invokes the closure.
    extern "C" fn trampoline<F: FnMut(i32)>(user_data: *mut c_void, v: i32) {
        // SAFETY: `user_data` is `&mut f` from the enclosing stack frame,
        // cast to `*mut c_void`.  `f` is alive for the duration of
        // `for_each_with_closure`, which does not return until
        // `sim_for_each_ctx` finishes — so the reference is valid and no
        // aliasing occurs (only one call at a time).
        let closure = unsafe { &mut *user_data.cast::<F>() };
        closure(v);
    }

    // SAFETY: casting `&mut f` to `*mut c_void` is valid; the pointer is
    // immediately consumed by `sim_for_each_ctx` and not stored elsewhere.
    let user_data = (&raw mut f).cast::<c_void>();
    sim_for_each_ctx(data, trampoline::<F>, user_data);
}

/// Fold `data` with a Rust closure, hiding the trampoline internals.
pub fn reduce_with_closure<F>(data: &[i32], init: i32, mut f: F) -> i32
where
    F: FnMut(i32, i32) -> i32,
{
    extern "C" fn trampoline<F: FnMut(i32, i32) -> i32>(
        user_data: *mut c_void,
        acc: i32,
        v: i32,
    ) -> i32 {
        // SAFETY: same as `for_each_with_closure` — `user_data` is `&mut f`,
        // valid and uniquely borrowed for the call.
        let closure = unsafe { &mut *user_data.cast::<F>() };
        closure(acc, v)
    }

    let user_data = (&raw mut f).cast::<c_void>();
    sim_reduce_ctx(data, init, trampoline::<F>, user_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Pattern 1: plain function pointer coercion ────────────────────────

    #[test]
    fn test_plain_fn_reduce_sum() {
        // `add` is `extern "C" fn(i32, i32) -> i32` — coerces with no casting.
        assert_eq!(sim_reduce(&[1, 2, 3, 4, 5], 0, add), 15);
        assert_eq!(sim_reduce(&[], 0, add), 0);
        assert_eq!(sim_reduce(&[7], 0, add), 7);
    }

    #[test]
    fn test_plain_fn_reduce_product() {
        assert_eq!(sim_reduce(&[1, 2, 3, 4, 5], 1, mul), 120);
        assert_eq!(sim_reduce(&[], 1, mul), 1);
    }

    #[test]
    fn test_plain_fn_reduce_max() {
        assert_eq!(sim_reduce(&[3, 1, 4, 1, 5, 9, 2, 6], i32::MIN, max_of), 9);
        assert_eq!(sim_reduce(&[-5, -3, -10], i32::MIN, max_of), -3);
    }

    #[test]
    fn test_plain_fn_reduce_single_element() {
        assert_eq!(sim_reduce(&[42], 0, add), 42);
        assert_eq!(sim_reduce(&[42], 1, mul), 42);
    }

    // ── Pattern 2: trampoline / closure with captures ─────────────────────

    #[test]
    fn test_closure_for_each_collects_into_vec() {
        // The closure captures `&mut collected` — impossible with a plain fn.
        let mut collected: Vec<i32> = Vec::new();
        for_each_with_closure(&[10, 20, 30], |v| collected.push(v));
        assert_eq!(collected, [10, 20, 30]);
    }

    #[test]
    fn test_closure_for_each_empty_slice() {
        let mut count = 0i32;
        for_each_with_closure(&[], |_| count += 1);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_closure_for_each_counts_matching() {
        // Closure captures `threshold` from the enclosing scope.
        let threshold = 3;
        let mut above = 0u32;
        for_each_with_closure(&[1, 2, 3, 4, 5], |v| {
            if v > threshold {
                above += 1;
            }
        });
        assert_eq!(above, 2); // 4 and 5
    }

    #[test]
    fn test_closure_reduce_sum_with_captured_offset() {
        // Each step adds `offset` on top of the element — impossible without captures.
        let offset = 10;
        // fold: 0 + (1+10)=11, 11 + (2+10)=23, 23 + (3+10)=36
        let result = reduce_with_closure(&[1, 2, 3], 0, |acc, v| acc + v + offset);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_closure_reduce_product() {
        let result = reduce_with_closure(&[2, 3, 4], 1, |acc, v| acc * v);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_closure_reduce_empty_returns_init() {
        let result = reduce_with_closure(&[], 99, |acc, v| acc + v);
        assert_eq!(result, 99);
    }
}
