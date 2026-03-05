// Rod Cutting — bottom-up DP O(n²)

fn rod_cut(prices: &[u64]) -> (u64, Vec<usize>) {
    let n = prices.len();
    let mut dp   = vec![0u64; n + 1];
    let mut cuts = vec![0usize; n + 1];

    for i in 1..=n {
        for j in 1..=i {
            let v = prices[j - 1] + dp[i - j];
            if v > dp[i] {
                dp[i]   = v;
                cuts[i] = j;
            }
        }
    }

    // Reconstruct
    let mut pieces = Vec::new();
    let mut len = n;
    while len > 0 {
        pieces.push(cuts[len]);
        len -= cuts[len];
    }
    (dp[n], pieces)
}

fn main() {
    let prices = vec![1u64, 5, 8, 9, 10, 17, 17, 20];
    let n = prices.len();
    let (revenue, pieces) = rod_cut(&prices);
    println!("Rod length:  {n}");
    println!("Max revenue: {revenue}");
    println!("Cut into:    {:?}", pieces);
}
