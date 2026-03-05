// Subset Sum — DP with bitset optimisation
// Bitset: bit i set means sum i is reachable. `bits |= bits << x` per element.

fn subset_sum_dp(nums: &[usize], target: usize) -> bool {
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for &x in nums {
        for s in (x..=target).rev() {
            if dp[s - x] { dp[s] = true; }
        }
    }
    dp[target]
}

/// Bitset optimisation using Vec<u64> — each bit represents one reachable sum
fn subset_sum_bitset(nums: &[usize], target: usize) -> bool {
    let words = (target / 64) + 1;
    let mut bits = vec![0u64; words];
    bits[0] = 1; // sum 0 is reachable
    for &x in nums {
        // bits |= bits << x
        // Shift left by x bits across word boundaries
        let word_shift = x / 64;
        let bit_shift  = x % 64;
        for i in (0..words).rev() {
            let from = if i >= word_shift { i - word_shift } else { continue };
            let mut shifted = bits[from] << bit_shift;
            if bit_shift > 0 && from > 0 {
                shifted |= bits[from - 1] >> (64 - bit_shift);
            }
            bits[i] |= shifted;
        }
    }
    let word = target / 64;
    let bit  = target % 64;
    (bits[word] >> bit) & 1 == 1
}

fn subset_find(nums: &[usize], target: usize) -> Option<Vec<usize>> {
    let n = nums.len();
    let mut dp = vec![vec![false; target + 1]; n + 1];
    dp[0][0] = true;
    for i in 1..=n {
        let x = nums[i - 1];
        for s in 0..=target {
            dp[i][s] = dp[i - 1][s] || (s >= x && dp[i - 1][s - x]);
        }
    }
    if !dp[n][target] { return None; }
    let mut subset = Vec::new();
    let mut s = target;
    for i in (1..=n).rev() {
        if !dp[i - 1][s] {
            subset.push(nums[i - 1]);
            s -= nums[i - 1];
        }
    }
    subset.reverse();
    Some(subset)
}

fn main() {
    let nums   = vec![3usize, 34, 4, 12, 5, 2];
    let target = 9;
    println!("nums={nums:?} target={target}");
    println!("DP boolean:  {}", subset_sum_dp(&nums, target));
    println!("Bitset:      {}", subset_sum_bitset(&nums, target));
    println!("Subset:      {:?}", subset_find(&nums, target));
    println!("Target=30:   {}", subset_sum_dp(&nums, 30));
    println!("Bitset=30:   {}", subset_sum_bitset(&nums, 30));
}
