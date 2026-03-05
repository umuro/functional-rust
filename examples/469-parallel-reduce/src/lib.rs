//! # Parallel Reduce — Concurrent Aggregation
//!
//! Reduce collections in parallel for faster aggregation.

use std::thread;

/// Parallel reduce with custom operation
pub fn parallel_reduce<T>(data: &[T], identity: T, op: &(dyn Fn(T, T) -> T + Sync)) -> T
where
    T: Send + Sync + Clone,
{
    const THRESHOLD: usize = 100;

    if data.is_empty() {
        return identity;
    }

    if data.len() <= THRESHOLD {
        return data.iter().cloned().fold(identity, |a, b| op(a, b));
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);

    thread::scope(|s| {
        let id_left = identity.clone();
        let left_handle = s.spawn(|| parallel_reduce(left, id_left, op));
        let right_result = parallel_reduce(right, identity, op);
        op(left_handle.join().unwrap(), right_result)
    })
}

/// Parallel sum
pub fn parallel_sum<T>(data: &[T]) -> T
where
    T: Send + Sync + Clone + std::ops::Add<Output = T> + Default,
{
    parallel_reduce(data, T::default(), &|a, b| a + b)
}

/// Parallel product
pub fn parallel_product<T>(data: &[T]) -> T
where
    T: Send + Sync + Clone + std::ops::Mul<Output = T> + From<u8>,
{
    parallel_reduce(data, T::from(1), &|a, b| a * b)
}

/// Parallel min
pub fn parallel_min<T: Send + Sync + Clone + Ord>(data: &[T]) -> Option<T> {
    if data.is_empty() {
        return None;
    }
    Some(parallel_reduce(data, data[0].clone(), &|a, b| {
        if a < b { a } else { b }
    }))
}

/// Parallel max
pub fn parallel_max<T: Send + Sync + Clone + Ord>(data: &[T]) -> Option<T> {
    if data.is_empty() {
        return None;
    }
    Some(parallel_reduce(data, data[0].clone(), &|a, b| {
        if a > b { a } else { b }
    }))
}

/// Parallel all (conjunction)
pub fn parallel_all<T>(data: &[T], predicate: &(dyn Fn(&T) -> bool + Sync)) -> bool
where
    T: Sync,
{
    const THRESHOLD: usize = 100;

    if data.is_empty() {
        return true;
    }

    if data.len() <= THRESHOLD {
        return data.iter().all(predicate);
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);

    thread::scope(|s| {
        let left_handle = s.spawn(|| parallel_all(left, predicate));
        let right_result = parallel_all(right, predicate);
        left_handle.join().unwrap() && right_result
    })
}

/// Parallel any (disjunction)
pub fn parallel_any<T>(data: &[T], predicate: &(dyn Fn(&T) -> bool + Sync)) -> bool
where
    T: Sync,
{
    const THRESHOLD: usize = 100;

    if data.is_empty() {
        return false;
    }

    if data.len() <= THRESHOLD {
        return data.iter().any(predicate);
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);

    thread::scope(|s| {
        let left_handle = s.spawn(|| parallel_any(left, predicate));
        let right_result = parallel_any(right, predicate);
        left_handle.join().unwrap() || right_result
    })
}

/// Parallel count matching predicate
pub fn parallel_count<T>(data: &[T], predicate: &(dyn Fn(&T) -> bool + Sync)) -> usize
where
    T: Sync,
{
    const THRESHOLD: usize = 100;

    if data.len() <= THRESHOLD {
        return data.iter().filter(|x| predicate(x)).count();
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);

    thread::scope(|s| {
        let left_handle = s.spawn(|| parallel_count(left, predicate));
        let right_result = parallel_count(right, predicate);
        left_handle.join().unwrap() + right_result
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_reduce_sum() {
        let data: Vec<i64> = (1..=1000).collect();
        let sum = parallel_reduce(&data, 0, &|a, b| a + b);
        assert_eq!(sum, 500500);
    }

    #[test]
    fn test_parallel_sum() {
        let data: Vec<i32> = (1..=100).collect();
        assert_eq!(parallel_sum(&data), 5050);
    }

    #[test]
    fn test_parallel_product() {
        let data: Vec<i64> = (1..=10).collect();
        assert_eq!(parallel_product(&data), 3628800); // 10!
    }

    #[test]
    fn test_parallel_min() {
        let data = vec![5, 2, 8, 1, 9, 3];
        assert_eq!(parallel_min(&data), Some(1));
    }

    #[test]
    fn test_parallel_max() {
        let data = vec![5, 2, 8, 1, 9, 3];
        assert_eq!(parallel_max(&data), Some(9));
    }

    #[test]
    fn test_parallel_all() {
        let data: Vec<i32> = (1..=100).collect();
        assert!(parallel_all(&data, &|x| *x > 0));
        assert!(!parallel_all(&data, &|x| *x > 50));
    }

    #[test]
    fn test_parallel_any() {
        let data: Vec<i32> = (1..=100).collect();
        assert!(parallel_any(&data, &|x| *x == 50));
        assert!(!parallel_any(&data, &|x| *x > 100));
    }

    #[test]
    fn test_parallel_count() {
        let data: Vec<i32> = (1..=100).collect();
        assert_eq!(parallel_count(&data, &|x| x % 2 == 0), 50);
    }

    #[test]
    fn test_empty_cases() {
        let empty: Vec<i32> = vec![];
        assert_eq!(parallel_sum(&empty), 0);
        assert_eq!(parallel_min(&empty), None);
        assert_eq!(parallel_max(&empty), None);
        assert!(parallel_all(&empty, &|_: &i32| false));
        assert!(!parallel_any(&empty, &|_: &i32| true));
    }
}
