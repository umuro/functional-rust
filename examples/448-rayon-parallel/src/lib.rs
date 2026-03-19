#![allow(clippy::all)]
//! # Rayon Parallel Iterators — Data Parallelism Made Easy
//!
//! Demonstrates the parallel iterator pattern that Rayon provides,
//! implemented here with scoped threads to show the concept.

use std::thread;

/// Approach 1: Parallel map over slices
///
/// Maps a function over slice elements in parallel.
pub fn parallel_map<T, U, F>(data: &[T], f: F) -> Vec<U>
where
    T: Sync,
    U: Send + Default + Clone,
    F: Fn(&T) -> U + Sync,
{
    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let chunk_size = (data.len() / num_threads).max(1);

    let mut results = vec![U::default(); data.len()];

    thread::scope(|s| {
        for (chunk_in, chunk_out) in data.chunks(chunk_size).zip(results.chunks_mut(chunk_size)) {
            s.spawn(|| {
                for (input, output) in chunk_in.iter().zip(chunk_out.iter_mut()) {
                    *output = f(input);
                }
            });
        }
    });

    results
}

/// Approach 2: Parallel sum/reduce
///
/// Sums elements in parallel by dividing into chunks.
pub fn parallel_sum(data: &[f64]) -> f64 {
    let num_threads = 4;
    let chunk_size = (data.len() / num_threads).max(1);

    let partial_sums: Vec<f64> = thread::scope(|s| {
        data.chunks(chunk_size)
            .map(|chunk| s.spawn(move || chunk.iter().sum::<f64>()))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect()
    });

    partial_sums.iter().sum()
}

/// Approach 3: Parallel filter-map
///
/// Filters and maps in parallel.
pub fn parallel_filter_map<T, U, F, P>(data: &[T], predicate: P, mapper: F) -> Vec<U>
where
    T: Sync,
    U: Send,
    F: Fn(&T) -> U + Sync,
    P: Fn(&T) -> bool + Sync,
{
    let num_threads = 4;
    let chunk_size = (data.len() / num_threads).max(1);

    let partial_results: Vec<Vec<U>> = thread::scope(|s| {
        data.chunks(chunk_size)
            .map(|chunk| {
                s.spawn(|| {
                    chunk
                        .iter()
                        .filter(|x| predicate(x))
                        .map(|x| mapper(x))
                        .collect::<Vec<_>>()
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect()
    });

    partial_results.into_iter().flatten().collect()
}

/// Approach 4: Parallel reduce with custom operation
pub fn parallel_reduce<T, F>(data: &[T], identity: T, op: F) -> T
where
    T: Send + Sync + Clone,
    F: Fn(T, T) -> T + Sync,
{
    if data.is_empty() {
        return identity;
    }

    let num_threads = 4;
    let chunk_size = (data.len() / num_threads).max(1);

    let partial: Vec<T> = thread::scope(|s| {
        data.chunks(chunk_size)
            .map(|chunk| {
                let op = &op;
                let id = identity.clone();
                s.spawn(move || chunk.iter().cloned().fold(id, |acc, x| op(acc, x)))
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect()
    });

    partial.into_iter().fold(identity, |acc, x| op(acc, x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_map_squares() {
        let data: Vec<f64> = (1..=10).map(|x| x as f64).collect();
        let squares = parallel_map(&data, |x| x * x);
        let expected: Vec<f64> = (1..=10).map(|x| (x * x) as f64).collect();
        assert_eq!(squares, expected);
    }

    #[test]
    fn test_parallel_map_strings() {
        let data = vec!["hello", "world", "rust"];
        let lengths = parallel_map(&data, |s| s.len());
        assert_eq!(lengths, vec![5, 5, 4]);
    }

    #[test]
    fn test_parallel_sum() {
        let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let sum = parallel_sum(&data);
        assert!((sum - 5050.0).abs() < 1e-9);
    }

    #[test]
    fn test_parallel_sum_empty() {
        let data: Vec<f64> = vec![];
        let sum = parallel_sum(&data);
        assert!((sum - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_parallel_filter_map() {
        let data: Vec<i32> = (1..=20).collect();
        // Get squares of even numbers
        let result = parallel_filter_map(&data, |x| x % 2 == 0, |x| x * x);
        let expected: Vec<i32> = vec![4, 16, 36, 64, 100, 144, 196, 256, 324, 400];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parallel_reduce_sum() {
        let data: Vec<i32> = (1..=10).collect();
        let sum = parallel_reduce(&data, 0, |a, b| a + b);
        assert_eq!(sum, 55);
    }

    #[test]
    fn test_parallel_reduce_max() {
        let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let max = parallel_reduce(&data, i32::MIN, |a, b| a.max(b));
        assert_eq!(max, 9);
    }

    #[test]
    fn test_parallel_reduce_product() {
        let data: Vec<i64> = (1..=5).collect();
        let product = parallel_reduce(&data, 1, |a, b| a * b);
        assert_eq!(product, 120);
    }
}
