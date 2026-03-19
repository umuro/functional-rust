#![allow(clippy::all)]
//! # Structured Concurrency
//! Spawn tasks that are guaranteed to complete before the scope exits.

use std::thread;

pub fn scoped_work<F, R>(work: F) -> R
where
    F: FnOnce() -> R + Send,
    R: Send,
{
    thread::scope(|s| {
        let handle = s.spawn(work);
        handle.join().unwrap()
    })
}

pub fn parallel_sum(nums: &[i32]) -> i32 {
    if nums.len() < 100 {
        return nums.iter().sum();
    }
    let mid = nums.len() / 2;
    let (left, right) = nums.split_at(mid);
    thread::scope(|s| {
        let l = s.spawn(|| parallel_sum(left));
        let r = s.spawn(|| parallel_sum(right));
        l.join().unwrap() + r.join().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scoped_returns_result() {
        assert_eq!(scoped_work(|| 42), 42);
    }
    #[test]
    fn parallel_sum_correct() {
        let nums: Vec<i32> = (1..=1000).collect();
        assert_eq!(parallel_sum(&nums), 500500);
    }
}
