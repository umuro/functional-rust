//! # Async Blocks and Lazy Evaluation
//!
//! Demonstrates lazy evaluation with closures as a synchronous analogy
//! for async blocks. Work is described but not executed until invoked.

/// Creates a lazy computation that prints a message when created and another when executed.
/// Analogous to `async { }` blocks which describe work without running it.
pub fn lazy_comp<'a, F, T>(label: &'a str, f: F) -> impl FnOnce() -> T + 'a
where
    F: FnOnce() -> T + 'a,
{
    println!("Creating: {}", label);
    move || {
        println!("Executing: {}", label);
        f()
    }
}

/// Conditionally run a lazy computation.
/// Analogous to: `if cond { fut.await } else { None }`
pub fn run_if<F, T>(cond: bool, thunk: F) -> Option<T>
where
    F: FnOnce() -> T,
{
    if cond {
        Some(thunk())
    } else {
        None
    }
}

/// Create multiple lazy tasks that capture a value by move.
/// Analogous to `async move { }` blocks.
pub fn create_tasks_with_capture(multiplier: i32, count: usize) -> Vec<Box<dyn FnOnce() -> i32>> {
    (1..=count as i32)
        .map(|x| -> Box<dyn FnOnce() -> i32> { Box::new(move || x * multiplier) })
        .collect()
}

/// A more idiomatic approach using iterators and Option.
pub fn lazy_filter_map<T, U, F>(items: impl IntoIterator<Item = T>, pred: F) -> Vec<U>
where
    F: Fn(&T) -> Option<U>,
{
    items.into_iter().filter_map(|x| pred(&x)).collect()
}

/// Chain multiple lazy computations.
pub fn chain_lazy<A, B, C, F, G>(first: F, second: G) -> impl FnOnce() -> C
where
    F: FnOnce() -> A,
    G: FnOnce(A) -> B,
    B: Into<C>,
{
    move || second(first()).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn test_lazy_not_called_until_invoked() {
        let called = Cell::new(false);
        let thunk = || {
            called.set(true);
            42
        };

        assert!(!called.get(), "should not be called yet");
        let result = thunk();
        assert!(called.get(), "should be called now");
        assert_eq!(result, 42);
    }

    #[test]
    fn test_run_if_skips_when_false() {
        let called = Cell::new(false);
        let result = run_if(false, || {
            called.set(true);
            panic!("should not reach here")
        });
        assert!(!called.get());
        assert!(result.is_none());
    }

    #[test]
    fn test_run_if_executes_when_true() {
        let result = run_if(true, || 42);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_create_tasks_with_capture() {
        let tasks = create_tasks_with_capture(7, 5);
        assert_eq!(tasks.len(), 5);

        let results: Vec<i32> = tasks.into_iter().map(|t| t()).collect();
        assert_eq!(results, vec![7, 14, 21, 28, 35]);
    }

    #[test]
    fn test_lazy_filter_map() {
        let items = vec![1, 2, 3, 4, 5, 6];
        let evens_doubled =
            lazy_filter_map(items, |&x| if x % 2 == 0 { Some(x * 2) } else { None });
        assert_eq!(evens_doubled, vec![4, 8, 12]);
    }

    #[test]
    fn test_chain_lazy() {
        let computation = chain_lazy::<i32, i32, i32, _, _>(|| 5, |x| x * 2);
        assert_eq!(computation(), 10);
    }
}
