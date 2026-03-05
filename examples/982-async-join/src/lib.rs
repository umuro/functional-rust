// 982: Join Parallel Async
// Rust: std::thread::spawn + join() — like OCaml's Lwt.both

use std::thread;

// --- Approach 1: Join two threads (Lwt.both analogue) ---
fn parallel_both<A, B, F1, F2>(f1: F1, f2: F2) -> (A, B)
where
    A: Send + 'static,
    B: Send + 'static,
    F1: FnOnce() -> A + Send + 'static,
    F2: FnOnce() -> B + Send + 'static,
{
    let h1 = thread::spawn(f1);
    let h2 = thread::spawn(f2);
    // Both run concurrently; join waits for both
    let a = h1.join().expect("thread 1 panicked");
    let b = h2.join().expect("thread 2 panicked");
    (a, b)
}

// --- Approach 2: Join N tasks and collect results ---
fn parallel_map<T, F>(tasks: Vec<F>) -> Vec<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let handles: Vec<_> = tasks.into_iter().map(thread::spawn).collect();
    handles.into_iter().map(|h| h.join().expect("task panicked")).collect()
}

// --- Approach 3: Parallel sum ---
fn parallel_sum(ns: Vec<i32>) -> i32 {
    let handles: Vec<_> = ns.into_iter()
        .map(|n| thread::spawn(move || n * n))
        .collect();
    handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_both() {
        let (a, b) = parallel_both(|| 6 * 7, || 10 + 20);
        assert_eq!(a, 42);
        assert_eq!(b, 30);
    }

    #[test]
    fn test_parallel_map() {
        let mut results = parallel_map(vec![
            Box::new(|| 2 + 2) as Box<dyn FnOnce() -> i32 + Send>,
            Box::new(|| 3 * 3),
            Box::new(|| 10 - 1),
        ]);
        results.sort(); // order may vary
        assert_eq!(results, vec![4, 9, 9]);
    }

    #[test]
    fn test_parallel_sum() {
        // 1+4+9+16 = 30
        assert_eq!(parallel_sum(vec![1, 2, 3, 4]), 30);
    }

    #[test]
    fn test_both_independent() {
        // Results don't depend on order
        let (x, y) = parallel_both(|| "hello", || 42u32);
        assert_eq!(x, "hello");
        assert_eq!(y, 42);
    }

    #[test]
    fn test_empty_parallel_map() {
        let results: Vec<i32> = parallel_map::<i32, fn() -> i32>(vec![]);
        assert!(results.is_empty());
    }
}
