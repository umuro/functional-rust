//! # 525. Tap Pattern for Side Effects
//! Inspect values in a pipeline without disrupting the data flow.

/// Tap: run a side effect, then return the value unchanged
fn tap<T, F: FnOnce(&T)>(value: T, f: F) -> T {
    f(&value);
    value
}

/// Tap with a mutable reference (for accumulating taps)
fn tap_mut<T, F: FnOnce(&mut T)>(mut value: T, f: F) -> T {
    f(&mut value);
    value
}

/// Extension trait to enable chained .tap() calls
trait Tap: Sized {
    fn tap(self, f: impl FnOnce(&Self)) -> Self {
        f(&self);
        self
    }

    fn tap_mut(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self);
        self
    }

    fn tap_if(self, condition: bool, f: impl FnOnce(&Self)) -> Self {
        if condition { f(&self); }
        self
    }
}

// Implement Tap for all types
impl<T> Tap for T {}

fn main() {
    // Basic tap in a transformation chain
    let result = 5i32
        .tap(|&x| println!("initial: {}", x))
        .tap(|x| println!("before double: {}", x));
    let result = result * 2;
    let result = result
        .tap(|x| println!("after double: {}", x));
    let result = result + 1;
    let result = result
        .tap(|x| println!("after inc: {}", x));
    let result = result * result;
    println!("final: {}", result);

    // Tap in iterator pipeline (using map as tap)
    println!("\nIterator pipeline with taps:");
    let sum: i32 = (1..=5)
        .map(|x| tap(x, |v| print!("in:{} ", v)))
        .filter(|&x| x % 2 != 0)
        .map(|x| tap(x * x, |v| print!("sq:{} ", v)))
        .sum();
    println!("\nsum of odd squares: {}", sum);

    // Tap with Vec — inspect intermediate state
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let result: Vec<i32> = data.into_iter()
        .tap(|_| println!("\nProcessing..."))  // wait, tap is on T not Iterator
        .collect::<Vec<_>>()
        .tap(|v| println!("collected {} items", v.len()))
        .tap_mut(|v| v.sort())
        .tap(|v| println!("sorted: {:?}", v))
        .tap_mut(|v| v.dedup())
        .tap(|v| println!("deduped: {:?}", v));

    println!("final: {:?}", result);

    // Conditional tap (only in debug mode)
    let debug = true;
    let x = 42i32
        .tap_if(debug, |v| println!("debug value: {}", v));
    println!("x = {}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tap_returns_value() {
        let result = tap(42, |_| {});
        assert_eq!(result, 42);
    }

    #[test]
    fn test_tap_runs_side_effect() {
        let mut called = false;
        let _ = tap(10, |_| { called = true; });
        assert!(called);
    }

    #[test]
    fn test_tap_trait() {
        let result = 5i32
            .tap(|_| {})
            .tap_mut(|x| *x *= 2);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_tap_if() {
        let mut log = Vec::new();
        let _ = 7i32.tap_if(true, |_| log.push("logged"));
        assert_eq!(log, ["logged"]);
        let _ = 7i32.tap_if(false, |_| log.push("should not log"));
        assert_eq!(log.len(), 1);
    }
}
