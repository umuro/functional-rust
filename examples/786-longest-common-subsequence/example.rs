// 786. Longest Common Subsequence — DP table + backtrack

// ── Length only (DP table) ─────────────────────────────────────────────────────

pub fn lcs_length(s1: &[char], s2: &[char]) -> usize {
    let (n, m) = (s1.len(), s2.len());
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if s1[i-1] == s2[j-1] {
                dp[i-1][j-1] + 1
            } else {
                dp[i-1][j].max(dp[i][j-1])
            };
        }
    }
    dp[n][m]
}

// ── Full backtrack: return the actual LCS string ───────────────────────────────

pub fn lcs(s1: &str, s2: &str) -> String {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    let (n, m) = (c1.len(), c2.len());

    // Build DP table
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if c1[i-1] == c2[j-1] {
                dp[i-1][j-1] + 1
            } else {
                dp[i-1][j].max(dp[i][j-1])
            };
        }
    }

    // Backtrack
    let mut result = Vec::new();
    let (mut i, mut j) = (n, m);
    while i > 0 && j > 0 {
        if c1[i-1] == c2[j-1] {
            result.push(c1[i-1]);
            i -= 1; j -= 1;
        } else if dp[i-1][j] > dp[i][j-1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    result.iter().rev().collect()
}

// ── Space-optimised: two rows only ────────────────────────────────────────────

pub fn lcs_length_opt(s1: &[char], s2: &[char]) -> usize {
    let m = s2.len();
    let mut prev = vec![0usize; m + 1];
    let mut curr = vec![0usize; m + 1];
    for i in 1..=s1.len() {
        for j in 1..=m {
            curr[j] = if s1[i-1] == s2[j-1] {
                prev[j-1] + 1
            } else {
                prev[j].max(curr[j-1])
            };
        }
        std::mem::swap(&mut prev, &mut curr);
        curr.fill(0);
    }
    prev[m]
}

// ── Diff-style output ─────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum DiffOp { Keep(char), Insert(char), Delete(char) }

pub fn diff(s1: &str, s2: &str) -> Vec<DiffOp> {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    let (n, m) = (c1.len(), c2.len());
    let mut dp = vec![vec![0i32; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if c1[i-1] == c2[j-1] { dp[i-1][j-1] + 1 }
                       else { dp[i-1][j].max(dp[i][j-1]) };
        }
    }
    let mut ops = Vec::new();
    let (mut i, mut j) = (n, m);
    while i > 0 || j > 0 {
        if i > 0 && j > 0 && c1[i-1] == c2[j-1] {
            ops.push(DiffOp::Keep(c1[i-1]));
            i -= 1; j -= 1;
        } else if j > 0 && (i == 0 || dp[i][j-1] >= dp[i-1][j]) {
            ops.push(DiffOp::Insert(c2[j-1]));
            j -= 1;
        } else {
            ops.push(DiffOp::Delete(c1[i-1]));
            i -= 1;
        }
    }
    ops.reverse();
    ops
}

fn main() {
    let pairs = [
        ("ABCBDAB", "BDCAB"),   // LCS = "BCAB" (length 4)
        ("AGGTAB",  "GXTXAYB"), // LCS = "GTAB" (length 4)
        ("ABCDEF",  "ACE"),     // LCS = "ACE"  (length 3)
    ];

    for (s1, s2) in pairs {
        let l = lcs(s1, s2);
        println!("lcs({s1:?}, {s2:?}) = {l:?} (len {})", l.len());
    }

    // Diff demo
    println!("\nDiff(\"kitten\", \"sitting\"):");
    for op in diff("kitten", "sitting") {
        match op {
            DiffOp::Keep(c)   => print!(" {c}"),
            DiffOp::Insert(c) => print!("[+{c}]"),
            DiffOp::Delete(c) => print!("[-{c}]"),
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcs_known_cases() {
        assert_eq!(lcs("ABCBDAB", "BDCAB").len(), 4);
        assert_eq!(lcs("AGGTAB", "GXTXAYB").len(), 4);
        assert_eq!(lcs("", "ABC"), "");
        assert_eq!(lcs("ABC", ""), "");
        assert_eq!(lcs("ABC", "ABC"), "ABC");
    }

    #[test]
    fn length_methods_agree() {
        let s1: Vec<char> = "ABCBDAB".chars().collect();
        let s2: Vec<char> = "BDCAB".chars().collect();
        assert_eq!(lcs_length(&s1, &s2), lcs_length_opt(&s1, &s2));
    }

    #[test]
    fn single_char() {
        assert_eq!(lcs("A", "A"), "A");
        assert_eq!(lcs("A", "B"), "");
    }
}
