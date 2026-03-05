// Example 120: Fn, FnMut, FnOnce
//
// Rust closures implement one or more of three traits based on how they
// use captured variables:
//   Fn      — borrows immutably; callable any number of times
//   FnMut   — borrows mutably;  callable any number of times
//   FnOnce  — moves out of captures; callable exactly once
//
// Every Fn is also FnMut and FnOnce.
// Every FnMut is also FnOnce.
// FnOnce is the most general bound; Fn is the most restrictive.

// ---------------------------------------------------------------------------
// Approach 1: Fn — immutable capture, callable repeatedly
// ---------------------------------------------------------------------------

/// Returns a closure that prepends `prefix` to any name.
/// The closure only *reads* `prefix`, so it implements `Fn`.
pub fn make_greeter(prefix: String) -> impl Fn(&str) -> String {
    move |name| format!("{prefix}, {name}!")
}

/// Accepts any `Fn` — caller knows it may be invoked multiple times
/// with shared (non-mutable) access to captures.
pub fn apply_twice<F: Fn() -> String>(f: F) -> (String, String) {
    (f(), f())
}

// ---------------------------------------------------------------------------
// Approach 2: FnMut — mutable capture, callable repeatedly
// ---------------------------------------------------------------------------

/// Returns a counter closure.  It mutates its captured `count`, so the
/// compiler infers `FnMut` (not plain `Fn`).
pub fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0u32;
    move || {
        count += 1;
        count
    }
}

/// Calls an `FnMut` closure `n` times and collects the results.
pub fn call_n_times<F: FnMut() -> u32>(mut f: F, n: usize) -> Vec<u32> {
    (0..n).map(|_| f()).collect()
}

// ---------------------------------------------------------------------------
// Approach 3: FnOnce — moves captured value out, callable exactly once
// ---------------------------------------------------------------------------

/// Returns a closure that *consumes* `message` when called.
/// Moving a value out of the closure body forces `FnOnce`.
pub fn make_one_shot(message: String) -> impl FnOnce() -> String {
    move || message // `message` is moved out on the single call
}

/// Accepts an `FnOnce` — the type system enforces single invocation.
pub fn consume_once<F: FnOnce() -> String>(f: F) -> String {
    f()
}

// ---------------------------------------------------------------------------
// Approach 4: Higher-order functions showing the trait hierarchy
// ---------------------------------------------------------------------------

/// Accepts the *most restrictive* bound: the closure must be freely
/// shareable (Fn).  This is appropriate for functions like `Iterator::map`.
pub fn transform_all<F: Fn(i32) -> i32>(data: &[i32], f: F) -> Vec<i32> {
    data.iter().map(|&x| f(x)).collect()
}

/// Accepts `FnMut` — the closure may carry mutable state across calls.
/// Useful for functions like `Iterator::for_each` that fold side effects.
pub fn accumulate<F: FnMut(i32) -> i32>(data: &[i32], mut f: F) -> Vec<i32> {
    data.iter().map(|&x| f(x)).collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Fn ---

    #[test]
    fn test_fn_greeter_callable_multiple_times() {
        let greet = make_greeter("Hello".into());
        assert_eq!(greet("Alice"), "Hello, Alice!");
        assert_eq!(greet("Bob"), "Hello, Bob!");
        // A third call proves the closure is not consumed
        assert_eq!(greet("Charlie"), "Hello, Charlie!");
    }

    #[test]
    fn test_fn_apply_twice() {
        let label = "ping";
        // Pure Fn closure: only reads captured `label`
        let (a, b) = apply_twice(|| label.to_string());
        assert_eq!(a, "ping");
        assert_eq!(b, "ping");
    }

    #[test]
    fn test_fn_transform_all() {
        let doubled = transform_all(&[1, 2, 3, 4], |x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_fn_transform_all_with_offset() {
        let offset = 10;
        // Closure captures `offset` by reference; still implements Fn
        let result = transform_all(&[1, 2, 3], |x| x + offset);
        assert_eq!(result, vec![11, 12, 13]);
    }

    // --- FnMut ---

    #[test]
    fn test_fnmut_counter_increments() {
        let mut counter = make_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_fnmut_call_n_times() {
        let counter = make_counter();
        let results = call_n_times(counter, 5);
        assert_eq!(results, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_fnmut_accumulate_running_sum() {
        let mut running = 0i32;
        // Closure mutates `running` across calls — FnMut
        let result = accumulate(&[1, 2, 3, 4], |x| {
            running += x;
            running
        });
        assert_eq!(result, vec![1, 3, 6, 10]);
    }

    // --- FnOnce ---

    #[test]
    fn test_fnonce_consume_once() {
        let shot = make_one_shot("boom".into());
        let result = consume_once(shot);
        assert_eq!(result, "boom");
        // `shot` is moved into `consume_once`; calling it again would be a compile error
    }

    #[test]
    fn test_fnonce_inline_closure() {
        let owned = String::from("consumed");
        // Moving `owned` out inside the body makes this FnOnce
        let f: Box<dyn FnOnce() -> usize> = Box::new(move || owned.len());
        assert_eq!(f(), 8);
    }

    // --- Trait hierarchy ---

    #[test]
    fn test_fn_satisfies_fnmut_and_fnonce_bounds() {
        // A pure Fn closure can be passed where FnMut or FnOnce is required
        let addend = 5i32;
        let pure_fn = |x: i32| x + addend; // implements Fn

        // Works as FnMut
        let as_fnmut: Vec<i32> = accumulate(&[1, 2, 3], pure_fn);
        assert_eq!(as_fnmut, vec![6, 7, 8]);
    }
}
