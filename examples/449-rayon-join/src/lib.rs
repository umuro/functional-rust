//! # Rayon Join — Fork-Join Parallelism
//!
//! Execute two closures in parallel and wait for both results.
//! This is the core primitive for divide-and-conquer parallelism.

use std::thread;

/// Approach 1: Simple join - run two tasks in parallel
///
/// One task runs in a spawned thread, the other in the current thread.
pub fn join<A, B, FA, FB>(f: FA, g: FB) -> (A, B)
where
    A: Send + 'static,
    B: Send + 'static,
    FA: FnOnce() -> A + Send + 'static,
    FB: FnOnce() -> B + Send + 'static,
{
    let handle = thread::spawn(f);
    let b = g();
    let a = handle.join().unwrap();
    (a, b)
}

/// Approach 2: Scoped join for borrowed data
pub fn scoped_join<'a, A, B, FA, FB>(f: FA, g: FB) -> (A, B)
where
    A: Send,
    B: Send,
    FA: FnOnce() -> A + Send + 'a,
    FB: FnOnce() -> B + Send + 'a,
{
    thread::scope(|s| {
        let handle = s.spawn(f);
        let b = g();
        let a = handle.join().unwrap();
        (a, b)
    })
}

/// Merge two sorted vectors
fn merge(a: Vec<i64>, b: Vec<i64>) -> Vec<i64> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            out.push(a[i]);
            i += 1;
        } else {
            out.push(b[j]);
            j += 1;
        }
    }

    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

/// Approach 3: Parallel merge sort using join
pub fn parallel_sort(mut v: Vec<i64>) -> Vec<i64> {
    // Base case: small arrays use sequential sort
    if v.len() <= 512 {
        v.sort();
        return v;
    }

    // Split and sort in parallel
    let right = v.split_off(v.len() / 2);
    let left = v;

    let (sorted_left, sorted_right) =
        join(move || parallel_sort(left), move || parallel_sort(right));

    merge(sorted_left, sorted_right)
}

/// Approach 4: Parallel sum using join
pub fn parallel_sum(data: &[i64]) -> i64 {
    const THRESHOLD: usize = 1000;

    if data.len() <= THRESHOLD {
        return data.iter().sum();
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);

    let (sum_left, sum_right) = scoped_join(|| parallel_sum(left), || parallel_sum(right));

    sum_left + sum_right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_basic() {
        let (a, b) = join(|| 6 * 7, || "hello".len());
        assert_eq!(a, 42);
        assert_eq!(b, 5);
    }

    #[test]
    fn test_join_independent_computations() {
        let (sum1, sum2) = join(
            || (1u64..=5000).sum::<u64>(),
            || (5001u64..=10000).sum::<u64>(),
        );
        assert_eq!(sum1 + sum2, 50005000);
    }

    #[test]
    fn test_scoped_join() {
        let data = vec![1, 2, 3, 4, 5];
        let (sum, len) = scoped_join(|| data.iter().sum::<i32>(), || data.len());
        assert_eq!(sum, 15);
        assert_eq!(len, 5);
    }

    #[test]
    fn test_parallel_sort_small() {
        let data = vec![5i64, 3, 8, 1, 9, 2, 7, 4, 6];
        let sorted = parallel_sort(data);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_parallel_sort_large() {
        let data: Vec<i64> = (0..2000).rev().collect();
        let sorted = parallel_sort(data);
        let expected: Vec<i64> = (0..2000).collect();
        assert_eq!(sorted, expected);
    }

    #[test]
    fn test_parallel_sum_small() {
        let data: Vec<i64> = (1..=100).collect();
        assert_eq!(parallel_sum(&data), 5050);
    }

    #[test]
    fn test_parallel_sum_large() {
        let data: Vec<i64> = (1..=10000).collect();
        assert_eq!(parallel_sum(&data), 50005000);
    }

    #[test]
    fn test_merge() {
        let a = vec![1, 3, 5, 7];
        let b = vec![2, 4, 6, 8];
        assert_eq!(merge(a, b), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
