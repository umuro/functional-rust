// 1064: Generate All Permutations via Backtracking

// Approach 1: Swap-based backtracking
fn permutations_swap(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    fn permute(start: usize, nums: &mut Vec<i32>, results: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            results.push(nums.clone());
            return;
        }
        for i in start..nums.len() {
            nums.swap(start, i);
            permute(start + 1, nums, results);
            nums.swap(start, i);
        }
    }
    permute(0, nums, &mut results);
    results
}

// Approach 2: Used-flags approach
fn permutations_flags(nums: &[i32]) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut results = Vec::new();
    let mut used = vec![false; n];
    let mut current = Vec::with_capacity(n);

    fn build(
        nums: &[i32],
        used: &mut Vec<bool>,
        current: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>,
    ) {
        if current.len() == nums.len() {
            results.push(current.clone());
            return;
        }
        for i in 0..nums.len() {
            if !used[i] {
                used[i] = true;
                current.push(nums[i]);
                build(nums, used, current, results);
                current.pop();
                used[i] = false;
            }
        }
    }

    build(nums, &mut used, &mut current, &mut results);
    results
}

// Approach 3: Iterator-based using Heap's algorithm
fn permutations_heap(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut arr = nums.to_vec();
    let n = arr.len();
    let mut results = vec![arr.clone()];
    let mut c = vec![0usize; n];
    let mut i = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                arr.swap(0, i);
            } else {
                arr.swap(c[i], i);
            }
            results.push(arr.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let mut nums = vec![1, 2, 3];
        let perms = permutations_swap(&mut nums);
        assert_eq!(perms.len(), 6);
        assert!(perms.contains(&vec![1, 2, 3]));
        assert!(perms.contains(&vec![3, 2, 1]));
    }

    #[test]
    fn test_flags() {
        let perms = permutations_flags(&[1, 2, 3]);
        assert_eq!(perms.len(), 6);
    }

    #[test]
    fn test_heap() {
        let perms = permutations_heap(&[1, 2, 3]);
        assert_eq!(perms.len(), 6);
    }

    #[test]
    fn test_single() {
        assert_eq!(permutations_flags(&[42]).len(), 1);
    }
}
