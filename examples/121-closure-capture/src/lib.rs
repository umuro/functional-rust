#![allow(clippy::all)]
//! Example 121: Closure Capture Modes
//!
//! Rust closures capture variables from their enclosing scope in three ways:
//! 1. By shared reference (`&T`) — default when only reading
//! 2. By mutable reference (`&mut T`) — when the closure mutates the variable
//! 3. By move (`T`) — with the `move` keyword; the closure owns its captures
//!
//! OCaml closures always capture by reference (GC-managed environment).
//! Rust requires you to be explicit when ownership is needed.

// ── Approach 1: Capture by shared reference (immutable borrow) ───────────────
//
// The compiler infers `&x` because the closure only reads `x`.
// After the closure is created, `x` is still fully accessible.
pub fn add_one_to(x: i32) -> impl Fn() -> i32 {
    // `move` is required here so the closure can outlive this stack frame.
    // For `Copy` types like `i32`, moving is equivalent to copying — `x`
    // would still be usable after the closure is created if we were in the
    // same scope. Here we need `move` to return the closure.
    move || x + 1
}

// ── Approach 2: Capture by mutable reference ─────────────────────────────────
//
// The closure takes `&mut total` because it writes to `total`.
// Only one `&mut` borrow can exist at a time, so we scope it tightly.
pub fn sum_with_closure(values: &[i32]) -> i32 {
    let mut total = 0;
    {
        // `add` borrows `total` mutably; the borrow ends at the end of this block.
        let mut add = |x: i32| total += x;
        for &v in values {
            add(v);
        }
    } // mutable borrow released here — `total` is readable again
    total
}

// ── Approach 3: move closure — closure owns its captures ─────────────────────
//
// `make_multiplier` returns a closure. The closure must own `factor` because
// the stack frame that created `factor` is gone by the time the closure runs.
// `move` gives the closure ownership; for `Copy` types this is just a copy.
pub fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

// ── Approach 4: move closure capturing a non-Copy value ──────────────────────
//
// When the captured value is not `Copy` (e.g. `String`), `move` transfers
// ownership into the closure. The original binding is no longer accessible.
pub fn make_greeter(name: String) -> impl Fn() -> String {
    move || format!("Hello, {}!", name)
}

// ── Approach 5: functional accumulator returning a new closure ────────────────
//
// Demonstrates that closures are first-class values: we build a pipeline of
// closures, each moving its own state.
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_borrow_via_move_copy() {
        // For Copy types, `move` copies the value; original is unaffected in
        // the same scope. Here we verify the returned closure computes correctly.
        let f = add_one_to(42);
        assert_eq!(f(), 43);
        // Calling multiple times — closure is Fn, not FnOnce.
        assert_eq!(f(), 43);
    }

    #[test]
    fn test_mutable_capture_accumulates() {
        let result = sum_with_closure(&[10, 20, 30]);
        assert_eq!(result, 60);

        let empty = sum_with_closure(&[]);
        assert_eq!(empty, 0);
    }

    #[test]
    fn test_move_closure_make_multiplier() {
        let double = make_multiplier(2);
        let triple = make_multiplier(3);
        assert_eq!(double(5), 10);
        assert_eq!(triple(5), 15);
        // Both closures are independent — they own separate copies of `factor`.
        assert_eq!(double(0), 0);
    }

    #[test]
    fn test_move_closure_non_copy_string() {
        let greet = make_greeter("Alice".to_string());
        assert_eq!(greet(), "Hello, Alice!");
        // Closure is `Fn` — can be called multiple times.
        assert_eq!(greet(), "Hello, Alice!");
    }

    #[test]
    fn test_adder_closure_pipeline() {
        let add5 = make_adder(5);
        let add10 = make_adder(10);
        // Each closure owns its own `n`.
        assert_eq!(add5(1), 6);
        assert_eq!(add10(1), 11);
        // Compose by chaining calls.
        assert_eq!(add10(add5(0)), 15);
    }

    #[test]
    fn test_closure_captures_independently() {
        // Two closures from the same factory do not share state.
        let a = make_multiplier(7);
        let b = make_multiplier(7);
        assert_eq!(a(3), b(3));
    }
}
