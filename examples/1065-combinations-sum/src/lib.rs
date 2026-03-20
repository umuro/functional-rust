#![allow(clippy::all)]
// 1065: Combination Sum — Find Combos Summing to Target

// Approach 1: Backtracking with reuse allowed
fn combination_sum(candidates: &mut Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort();
    let mut results = Vec::new();
    let mut current = Vec::new();

    fn backtrack(
        start: usize,
        remaining: i32,
        candidates: &[i32],
        current: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>,
    ) {
        if remaining == 0 {
            results.push(current.clone());
            return;
        }
        for i in start..candidates.len() {
            if candidates[i] > remaining {
                break;
            } // sorted, so prune
            current.push(candidates[i]);
            backtrack(i, remaining - candidates[i], candidates, current, results);
            current.pop();
        }
    }

    backtrack(0, target, candidates, &mut current, &mut results);
    results
}

// Approach 2: Functional with iterators
fn combination_sum_func(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut sorted = candidates.to_vec();
    sorted.sort();

    fn solve(start: usize, remaining: i32, sorted: &[i32]) -> Vec<Vec<i32>> {
        if remaining == 0 {
            return vec![vec![]];
        }
        if remaining < 0 {
            return vec![];
        }
        let mut results = Vec::new();
        for i in start..sorted.len() {
            if sorted[i] > remaining {
                break;
            }
            for mut combo in solve(i, remaining - sorted[i], sorted) {
                combo.insert(0, sorted[i]);
                results.push(combo);
            }
        }
        results
    }

    solve(0, target, &sorted)
}

// Approach 3: Combination sum II (each number used once)
fn combination_sum_unique(candidates: &mut Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort();
    let mut results = Vec::new();
    let mut current = Vec::new();

    fn backtrack(
        start: usize,
        remaining: i32,
        candidates: &[i32],
        current: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>,
    ) {
        if remaining == 0 {
            results.push(current.clone());
            return;
        }
        for i in start..candidates.len() {
            if candidates[i] > remaining {
                break;
            }
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            } // skip dupes
            current.push(candidates[i]);
            backtrack(
                i + 1,
                remaining - candidates[i],
                candidates,
                current,
                results,
            );
            current.pop();
        }
    }

    backtrack(0, target, candidates, &mut current, &mut results);
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum() {
        let mut cands = vec![2, 3, 6, 7];
        let r = combination_sum(&mut cands, 7);
        assert_eq!(r.len(), 2);
        assert!(r.contains(&vec![2, 2, 3]));
        assert!(r.contains(&vec![7]));
    }

    #[test]
    fn test_combination_sum_func() {
        let r = combination_sum_func(&[2, 3, 5], 8);
        assert_eq!(r.len(), 3);
    }

    #[test]
    fn test_combination_sum_unique() {
        let mut cands = vec![10, 1, 2, 7, 6, 1, 5];
        let r = combination_sum_unique(&mut cands, 8);
        assert!(r.contains(&vec![1, 1, 6]));
        assert!(r.contains(&vec![1, 2, 5]));
        assert!(r.contains(&vec![1, 7]));
        assert!(r.contains(&vec![2, 6]));
    }
}
