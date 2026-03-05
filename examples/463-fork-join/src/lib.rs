//! # Fork-Join Pattern — Divide and Conquer
//!
//! Split work into subtasks, process in parallel, and merge results.

use std::thread;

/// Fork-join sum: divide array, sum halves in parallel
pub fn parallel_sum(data: &[i64]) -> i64 {
    const THRESHOLD: usize = 1000;

    if data.len() <= THRESHOLD {
        return data.iter().sum();
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);

    thread::scope(|s| {
        let left_handle = s.spawn(|| parallel_sum(left));
        let right_sum = parallel_sum(right);
        left_handle.join().unwrap() + right_sum
    })
}

/// Fork-join map: apply function in parallel
pub fn parallel_map<T, U, F>(data: &[T], f: F) -> Vec<U>
where
    T: Sync,
    U: Send + Clone + Default,
    F: Fn(&T) -> U + Sync,
{
    const THRESHOLD: usize = 100;

    if data.len() <= THRESHOLD {
        return data.iter().map(&f).collect();
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);

    thread::scope(|s| {
        let left_handle = s.spawn(|| parallel_map(left, &f));
        let mut right_result = parallel_map(right, &f);

        let mut left_result = left_handle.join().unwrap();
        left_result.append(&mut right_result);
        left_result
    })
}

/// Fork-join reduce: combine with custom operation
pub fn parallel_reduce<T, F>(data: &[T], identity: T, op: F) -> T
where
    T: Send + Sync + Clone,
    F: Fn(T, T) -> T + Send + Sync,
{
    const THRESHOLD: usize = 100;

    if data.is_empty() {
        return identity;
    }

    if data.len() <= THRESHOLD {
        return data.iter().cloned().fold(identity, &op);
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);

    thread::scope(|s| {
        let left_handle = s.spawn(|| parallel_reduce(left, identity.clone(), &op));
        let right_result = parallel_reduce(right, identity, &op);
        op(left_handle.join().unwrap(), right_result)
    })
}

/// Fork-join quicksort
pub fn parallel_quicksort<T: Ord + Send + Clone>(data: &mut [T]) {
    const THRESHOLD: usize = 1000;

    if data.len() <= THRESHOLD {
        data.sort();
        return;
    }

    let pivot_idx = partition(data);

    let (left, right) = data.split_at_mut(pivot_idx);
    let right = &mut right[1..]; // Skip pivot

    thread::scope(|s| {
        s.spawn(|| parallel_quicksort(left));
        parallel_quicksort(right);
    });
}

fn partition<T: Ord + Clone>(data: &mut [T]) -> usize {
    let len = data.len();
    let pivot = data[len / 2].clone();
    let mut i = 0;
    let mut j = len - 1;

    loop {
        while data[i] < pivot {
            i += 1;
        }
        while data[j] > pivot {
            j -= 1;
        }
        if i >= j {
            return j;
        }
        data.swap(i, j);
        i += 1;
        j -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_parallel_sum_empty() {
        let data: Vec<i64> = vec![];
        assert_eq!(parallel_sum(&data), 0);
    }

    #[test]
    fn test_parallel_map() {
        let data: Vec<i32> = (1..=10).collect();
        let squares = parallel_map(&data, |x| x * x);
        assert_eq!(squares, vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100]);
    }

    #[test]
    fn test_parallel_reduce_sum() {
        let data: Vec<i64> = (1..=1000).collect();
        let sum = parallel_reduce(&data, 0, |a, b| a + b);
        assert_eq!(sum, 500500);
    }

    #[test]
    fn test_parallel_reduce_max() {
        let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let max = parallel_reduce(&data, i32::MIN, |a, b| a.max(b));
        assert_eq!(max, 9);
    }

    #[test]
    fn test_parallel_quicksort() {
        let mut data: Vec<i32> = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
        parallel_quicksort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_parallel_quicksort_large() {
        let mut data: Vec<i32> = (0..5000).rev().collect();
        parallel_quicksort(&mut data);
        let expected: Vec<i32> = (0..5000).collect();
        assert_eq!(data, expected);
    }
}
