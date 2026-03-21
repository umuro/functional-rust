#![allow(dead_code)]
#![allow(clippy::all)]
// 1058: Longest Increasing Subsequence — O(n log n) Patience Sorting

// Approach 1: O(n^2) DP
fn lis_dp(arr: &[i32]) -> usize {
    if arr.is_empty() {
        return 0;
    }
    let n = arr.len();
    let mut dp = vec![1usize; n];
    for i in 1..n {
        for j in 0..i {
            if arr[j] < arr[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    *dp.iter().max().unwrap()
}

// Approach 2: O(n log n) patience sorting with binary search
fn lis_patience(arr: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::new();
    for &x in arr {
        match tails.binary_search(&x) {
            Ok(pos) => tails[pos] = x,
            Err(pos) => {
                if pos == tails.len() {
                    tails.push(x);
                } else {
                    tails[pos] = x;
                }
            }
        }
    }
    tails.len()
}

// Approach 3: Iterator-based with fold
fn lis_fold(arr: &[i32]) -> usize {
    arr.iter()
        .fold(Vec::new(), |mut tails: Vec<i32>, &x| {
            match tails.binary_search(&x) {
                Ok(pos) => tails[pos] = x,
                Err(pos) => {
                    if pos == tails.len() {
                        tails.push(x);
                    } else {
                        tails[pos] = x;
                    }
                }
            }
            tails
        })
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lis_dp() {
        assert_eq!(lis_dp(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(lis_dp(&[0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(lis_dp(&[7, 7, 7, 7]), 1);
    }

    #[test]
    fn test_lis_patience() {
        assert_eq!(lis_patience(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(lis_patience(&[0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(lis_patience(&[7, 7, 7, 7]), 1);
    }

    #[test]
    fn test_lis_fold() {
        assert_eq!(lis_fold(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(lis_fold(&[0, 1, 0, 3, 2, 3]), 4);
    }
}
