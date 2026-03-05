// 1066: All Subsets (Power Set) — Backtracking vs Bitmasking

// Approach 1: Backtracking
fn subsets_backtrack(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let mut current = Vec::new();

    fn build(start: usize, nums: &[i32], current: &mut Vec<i32>, results: &mut Vec<Vec<i32>>) {
        results.push(current.clone());
        for i in start..nums.len() {
            current.push(nums[i]);
            build(i + 1, nums, current, results);
            current.pop();
        }
    }

    build(0, nums, &mut current, &mut results);
    results
}

// Approach 2: Bitmasking
fn subsets_bitmask(nums: &[i32]) -> Vec<Vec<i32>> {
    let n = nums.len();
    let total = 1 << n;
    (0..total)
        .map(|mask| {
            (0..n)
                .filter(|&i| mask & (1 << i) != 0)
                .map(|i| nums[i])
                .collect()
        })
        .collect()
}

// Approach 3: Iterative doubling (fold)
fn subsets_fold(nums: &[i32]) -> Vec<Vec<i32>> {
    nums.iter().fold(vec![vec![]], |acc, &x| {
        let mut result = acc.clone();
        for mut subset in acc {
            subset.push(x);
            result.push(subset);
        }
        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtrack() {
        let s = subsets_backtrack(&[1, 2, 3]);
        assert_eq!(s.len(), 8);
        assert!(s.contains(&vec![]));
        assert!(s.contains(&vec![1, 2, 3]));
    }

    #[test]
    fn test_bitmask() {
        let s = subsets_bitmask(&[1, 2, 3]);
        assert_eq!(s.len(), 8);
    }

    #[test]
    fn test_fold() {
        let s = subsets_fold(&[1, 2, 3]);
        assert_eq!(s.len(), 8);
    }

    #[test]
    fn test_empty() {
        assert_eq!(subsets_backtrack(&[]).len(), 1);
    }
}
