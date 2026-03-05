// 787. Edit Distance (Levenshtein) DP
// Table + traceback + space-optimised two-row

// ── Edit operations ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum EditOp {
    Match(char),       // characters are equal
    Substitute(char, char), // replace old with new
    Insert(char),      // insert character from s2
    Delete(char),      // delete character from s1
}

// ── DP table (full) ────────────────────────────────────────────────────────────

pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    let (n, m) = (c1.len(), c2.len());

    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 0..=n { dp[i][0] = i; }
    for j in 0..=m { dp[0][j] = j; }

    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if c1[i-1] == c2[j-1] {
                dp[i-1][j-1]
            } else {
                1 + dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1])
            };
        }
    }
    dp[n][m]
}

// ── Backtrack: reconstruct edit operations ────────────────────────────────────

pub fn edit_ops(s1: &str, s2: &str) -> Vec<EditOp> {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    let (n, m) = (c1.len(), c2.len());

    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 0..=n { dp[i][0] = i; }
    for j in 0..=m { dp[0][j] = j; }
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if c1[i-1] == c2[j-1] { dp[i-1][j-1] }
                       else { 1 + dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1]) };
        }
    }

    // Traceback
    let mut ops = Vec::new();
    let (mut i, mut j) = (n, m);
    while i > 0 || j > 0 {
        if i > 0 && j > 0 && c1[i-1] == c2[j-1] {
            ops.push(EditOp::Match(c1[i-1]));
            i -= 1; j -= 1;
        } else if i > 0 && j > 0 && dp[i][j] == dp[i-1][j-1] + 1 {
            ops.push(EditOp::Substitute(c1[i-1], c2[j-1]));
            i -= 1; j -= 1;
        } else if i > 0 && dp[i][j] == dp[i-1][j] + 1 {
            ops.push(EditOp::Delete(c1[i-1]));
            i -= 1;
        } else {
            ops.push(EditOp::Insert(c2[j-1]));
            j -= 1;
        }
    }
    ops.reverse();
    ops
}

// ── Space-optimised: two rows ─────────────────────────────────────────────────

pub fn edit_distance_opt(s1: &str, s2: &str) -> usize {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    let m = c2.len();

    let mut prev: Vec<usize> = (0..=m).collect();
    let mut curr = vec![0usize; m + 1];

    for i in 1..=c1.len() {
        curr[0] = i;
        for j in 1..=m {
            curr[j] = if c1[i-1] == c2[j-1] {
                prev[j-1]
            } else {
                1 + prev[j].min(curr[j-1]).min(prev[j-1])
            };
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[m]
}

fn print_ops(s1: &str, s2: &str) {
    let ops = edit_ops(s1, s2);
    print!("  ops: ");
    for op in &ops {
        match op {
            EditOp::Match(c)        => print!("{c}"),
            EditOp::Substitute(a,b) => print!("[{a}→{b}]"),
            EditOp::Insert(c)       => print!("[+{c}]"),
            EditOp::Delete(c)       => print!("[-{c}]"),
        }
    }
    println!();
}

fn main() {
    let pairs = [
        ("kitten",   "sitting"),   // 3
        ("saturday", "sunday"),    // 3
        ("",         "abc"),       // 3
        ("abc",      "abc"),       // 0
        ("intention","execution"), // 5
    ];

    for (s1, s2) in pairs {
        let d1 = edit_distance(s1, s2);
        let d2 = edit_distance_opt(s1, s2);
        assert_eq!(d1, d2);
        println!("edit({s1:?}, {s2:?}) = {d1}");
        print_ops(s1, s2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_distances() {
        assert_eq!(edit_distance("kitten", "sitting"),    3);
        assert_eq!(edit_distance("saturday", "sunday"),   3);
        assert_eq!(edit_distance("", "abc"),              3);
        assert_eq!(edit_distance("abc", ""),              3);
        assert_eq!(edit_distance("abc", "abc"),           0);
        assert_eq!(edit_distance("intention","execution"),5);
    }

    #[test]
    fn optimised_agrees() {
        for (s1, s2) in [("abc","xyz"), ("", "a"), ("aaa","aaaa")] {
            assert_eq!(edit_distance(s1,s2), edit_distance_opt(s1,s2));
        }
    }

    #[test]
    fn ops_count_matches_distance() {
        let s1 = "kitten"; let s2 = "sitting";
        let ops = edit_ops(s1, s2);
        let changes = ops.iter().filter(|o| !matches!(o, EditOp::Match(_))).count();
        assert_eq!(changes, edit_distance(s1, s2));
    }
}
