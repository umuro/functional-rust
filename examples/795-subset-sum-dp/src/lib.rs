//! # Subset Sum

pub fn subset_sum(nums: &[i32], target: i32) -> bool {
    if target < 0 {
        return false;
    }
    let t = target as usize;
    let mut dp = vec![false; t + 1];
    dp[0] = true;
    for &n in nums {
        if n < 0 {
            continue;
        }
        let n = n as usize;
        for i in (n..=t).rev() {
            dp[i] = dp[i] || dp[i - n];
        }
    }
    dp[t]
}

pub fn count_subsets(nums: &[usize], target: usize) -> usize {
    let mut dp = vec![0usize; target + 1];
    dp[0] = 1;
    for &n in nums {
        for i in (n..=target).rev() {
            dp[i] += dp[i - n];
        }
    }
    dp[target]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_subset() {
        assert!(subset_sum(&[3, 34, 4, 12, 5, 2], 9));
    }
    #[test]
    fn test_count() {
        assert_eq!(count_subsets(&[1, 2, 3], 4), 1);
    }
}
