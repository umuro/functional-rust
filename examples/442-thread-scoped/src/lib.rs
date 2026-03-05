//! # Scoped Threads — Borrow Stack Data Across Threads
//!
//! Use `thread::scope` to spawn threads that borrow local data directly
//! — no `Arc`, no cloning, no heap allocation.

use std::thread;

/// Approach 1: Parallel sum using scoped threads
///
/// Splits data and processes halves in parallel, borrowing directly.
pub fn parallel_sum(data: &[i64]) -> i64 {
    if data.len() < 2 {
        return data.iter().sum();
    }

    let (left, right) = data.split_at(data.len() / 2);
    let mut ls = 0i64;
    let mut rs = 0i64;

    thread::scope(|s| {
        let t1 = s.spawn(|| left.iter().sum::<i64>());
        let t2 = s.spawn(|| right.iter().sum::<i64>());
        ls = t1.join().unwrap();
        rs = t2.join().unwrap();
    });

    ls + rs
}

/// Approach 2: Parallel map over chunks
///
/// Process data in parallel chunks, collecting results.
pub fn parallel_map<T, U, F>(data: &[T], chunk_size: usize, f: F) -> Vec<U>
where
    T: Sync,
    U: Send,
    F: Fn(&T) -> U + Sync,
{
    let mut results = Vec::with_capacity(data.len());

    thread::scope(|s| {
        let handles: Vec<_> = data
            .chunks(chunk_size)
            .map(|chunk| s.spawn(|| chunk.iter().map(&f).collect::<Vec<_>>()))
            .collect();

        for handle in handles {
            results.extend(handle.join().unwrap());
        }
    });

    results
}

/// Approach 3: Multiple readers of borrowed data
///
/// Multiple threads can borrow shared references simultaneously.
pub fn parallel_count_matches(data: &[i32], predicate: impl Fn(&i32) -> bool + Sync) -> usize {
    let num_threads = 4.min(data.len());
    if num_threads == 0 {
        return 0;
    }

    let chunk_size = (data.len() + num_threads - 1) / num_threads;
    let mut counts = vec![0usize; num_threads];

    thread::scope(|s| {
        let handles: Vec<_> = data
            .chunks(chunk_size)
            .enumerate()
            .map(|(i, chunk)| {
                let pred = &predicate;
                s.spawn(move || chunk.iter().filter(|x| pred(x)).count())
            })
            .collect();

        for (i, h) in handles.into_iter().enumerate() {
            if i < counts.len() {
                counts[i] = h.join().unwrap();
            }
        }
    });

    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_parallel_sum_basic() {
        let data: Vec<i64> = (1..=100).collect();
        assert_eq!(parallel_sum(&data), 5050);
    }

    #[test]
    fn test_parallel_sum_empty() {
        let data: Vec<i64> = vec![];
        assert_eq!(parallel_sum(&data), 0);
    }

    #[test]
    fn test_parallel_sum_single() {
        let data: Vec<i64> = vec![42];
        assert_eq!(parallel_sum(&data), 42);
    }

    #[test]
    fn test_borrow_string_in_scope() {
        let s = String::from("hello");
        thread::scope(|sc| {
            sc.spawn(|| assert_eq!(s.len(), 5));
        });
    }

    #[test]
    fn test_multiple_readers() {
        let message = String::from("shared");
        let mut results = Vec::new();

        thread::scope(|s| {
            let h1 = s.spawn(|| message.len());
            let h2 = s.spawn(|| message.chars().count());
            results.push(h1.join().unwrap());
            results.push(h2.join().unwrap());
        });

        assert_eq!(results, vec![6, 6]);
    }

    #[test]
    fn test_parallel_map() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let results = parallel_map(&data, 2, |x| x * x);
        assert_eq!(results, vec![1, 4, 9, 16, 25, 36, 49, 64]);
    }

    #[test]
    fn test_parallel_count_matches() {
        let data: Vec<i32> = (1..=100).collect();
        let count = parallel_count_matches(&data, |&x| x % 2 == 0);
        assert_eq!(count, 50);
    }

    #[test]
    fn test_mutable_split() {
        let mut data = vec![1, 2, 3, 4, 5, 6];
        let (left, right) = data.split_at_mut(3);

        thread::scope(|s| {
            s.spawn(|| {
                for x in left.iter_mut() {
                    *x *= 2;
                }
            });
            s.spawn(|| {
                for x in right.iter_mut() {
                    *x *= 3;
                }
            });
        });

        assert_eq!(data, vec![2, 4, 6, 12, 15, 18]);
    }
}
