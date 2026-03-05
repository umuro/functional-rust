// Palindrome Partitioning — minimum cuts DP O(n²)
// Precompute is_pal table, then solve cuts[i] bottom-up

fn palindrome_partition(s: &str) -> (usize, Vec<String>) {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return (0, vec![]);
    }

    // is_pal[i][j] = true if b[i..=j] is a palindrome
    let mut is_pal = vec![vec![false; n]; n];
    for i in 0..n { is_pal[i][i] = true; }
    for i in 0..n - 1 { is_pal[i][i + 1] = b[i] == b[i + 1]; }
    for len in 3..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            is_pal[i][j] = b[i] == b[j] && is_pal[i + 1][j - 1];
        }
    }

    // cuts[i] = min cuts for b[0..=i]
    let mut cuts = vec![usize::MAX; n];
    let mut prev = vec![0usize; n]; // start of last partition

    for i in 0..n {
        if is_pal[0][i] {
            cuts[i] = 0;
            prev[i] = 0;
        } else {
            for j in 1..=i {
                if is_pal[j][i] {
                    let c = cuts[j - 1].saturating_add(1);
                    if c < cuts[i] {
                        cuts[i] = c;
                        prev[i] = j;
                    }
                }
            }
        }
    }

    // Reconstruct
    let mut parts = Vec::new();
    let mut j = n as isize - 1;
    while j >= 0 {
        let start = prev[j as usize];
        parts.push(s[start..=(j as usize)].to_string());
        j = start as isize - 1;
    }
    parts.reverse();
    (cuts[n - 1], parts)
}

fn main() {
    for s in &["aab", "a", "ab", "aabb", "racecaranana"] {
        let (cuts, parts) = palindrome_partition(s);
        println!("{:?} -> cuts={}, parts={:?}", s, cuts, parts);
    }
}
