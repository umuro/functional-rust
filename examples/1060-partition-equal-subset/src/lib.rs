#![allow(clippy::all)]
// 1060: Partition Equal Subset Sum — Boolean DP

use std::collections::{HashMap, HashSet};

// Approach 1: Bottom-up boolean DP
fn can_partition(nums: &[i32]) -> bool {
    let total: i32 = nums.iter().sum();
    if total % 2 != 0 {
        return false;
    }
    let target = total as usize / 2;
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for &num in nums {
        let num = num as usize;
        for j in (num..=target).rev() {
            if dp[j - num] {
                dp[j] = true;
            }
        }
    }
    dp[target]
}

// Approach 2: Using HashSet for reachable sums
fn can_partition_set(nums: &[i32]) -> bool {
    let total: i32 = nums.iter().sum();
    if total % 2 != 0 {
        return false;
    }
    let target = total / 2;
    let mut reachable = HashSet::new();
    reachable.insert(0i32);
    for &num in nums {
        let new_sums: Vec<i32> = reachable
            .iter()
            .map(|&s| s + num)
            .filter(|&s| s <= target)
            .collect();
        for s in new_sums {
            reachable.insert(s);
        }
        if reachable.contains(&target) {
            return true;
        }
    }
    reachable.contains(&target)
}

// Approach 3: Recursive with memoization
fn can_partition_memo(nums: &[i32]) -> bool {
    let total: i32 = nums.iter().sum();
    if total % 2 != 0 {
        return false;
    }
    let target = total / 2;
    fn solve(
        i: usize,
        remaining: i32,
        nums: &[i32],
        cache: &mut HashMap<(usize, i32), bool>,
    ) -> bool {
        if remaining == 0 {
            return true;
        }
        if i >= nums.len() || remaining < 0 {
            return false;
        }
        if let Some(&v) = cache.get(&(i, remaining)) {
            return v;
        }
        let v =
            solve(i + 1, remaining - nums[i], nums, cache) || solve(i + 1, remaining, nums, cache);
        cache.insert((i, remaining), v);
        v
    }
    let mut cache = HashMap::new();
    solve(0, target, nums, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_partition() {
        assert!(can_partition(&[1, 5, 11, 5]));
        assert!(!can_partition(&[1, 2, 3, 5]));
        assert!(can_partition(&[1, 1]));
    }

    #[test]
    fn test_can_partition_set() {
        assert!(can_partition_set(&[1, 5, 11, 5]));
        assert!(!can_partition_set(&[1, 2, 3, 5]));
    }

    #[test]
    fn test_can_partition_memo() {
        assert!(can_partition_memo(&[1, 5, 11, 5]));
        assert!(!can_partition_memo(&[1, 2, 3, 5]));
    }
}
