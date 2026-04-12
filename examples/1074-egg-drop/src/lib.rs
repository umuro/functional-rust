#![allow(dead_code)]
#![allow(clippy::all)]
// 1074: Egg Drop — DP + Binary Search

// Approach 1: Basic DP O(k*n^2)
fn egg_drop_basic(eggs: usize, floors: usize) -> usize {
    let mut dp = vec![vec![0usize; floors + 1]; eggs + 1];
    for i in 1..=eggs {
        for j in 1..=floors {
            if i == 1 {
                dp[i][j] = j;
            } else {
                dp[i][j] = usize::MAX;
                for x in 1..=j {
                    let worst = 1 + dp[i - 1][x - 1].max(dp[i][j - x]);
                    dp[i][j] = dp[i][j].min(worst);
                }
            }
        }
    }
    dp[eggs][floors]
}

// Approach 2: DP with binary search O(k*n*log(n))
fn egg_drop_bs(eggs: usize, floors: usize) -> usize {
    let mut dp = vec![vec![0usize; floors + 1]; eggs + 1];
    for i in 1..=eggs {
        for j in 1..=floors {
            if i == 1 {
                dp[i][j] = j;
            } else {
                let (mut lo, mut hi) = (1, j);
                while lo < hi {
                    let mid = (lo + hi) / 2;
                    if dp[i - 1][mid - 1] < dp[i][j - mid] {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                dp[i][j] = 1 + dp[i - 1][lo - 1].max(dp[i][j - lo]);
            }
        }
    }
    dp[eggs][floors]
}

// Approach 3: Optimal — how many floors can we check with t trials and k eggs?
fn egg_drop_optimal(eggs: usize, floors: usize) -> usize {
    // dp[t][k] = max floors checkable with t trials and k eggs
    let mut dp = vec![vec![0usize; eggs + 1]; floors + 1];
    for t in 1..=floors {
        for k in 1..=eggs {
            dp[t][k] = 1 + dp[t - 1][k - 1] + dp[t - 1][k];
            if dp[t][k] >= floors && k == eggs {
                return t;
            }
        }
    }
    floors // fallback (1 egg case)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(egg_drop_basic(1, 10), 10);
        assert_eq!(egg_drop_basic(2, 10), 4);
        assert_eq!(egg_drop_basic(2, 6), 3);
        assert_eq!(egg_drop_basic(3, 14), 4);
    }

    #[test]
    fn test_binary_search() {
        assert_eq!(egg_drop_bs(1, 10), 10);
        assert_eq!(egg_drop_bs(2, 10), 4);
        assert_eq!(egg_drop_bs(2, 6), 3);
    }

    #[test]
    fn test_optimal() {
        assert_eq!(egg_drop_optimal(1, 10), 10);
        assert_eq!(egg_drop_optimal(2, 10), 4);
        assert_eq!(egg_drop_optimal(2, 6), 3);
        assert_eq!(egg_drop_optimal(2, 100), 14);
    }
}
