// Word Break — dictionary DP O(n²)
use std::collections::HashSet;

fn word_break(s: &str, dict: &[&str]) -> bool {
    let dict_set: HashSet<&str> = dict.iter().copied().collect();
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..=n {
        for j in 0..i {
            if dp[j] && dict_set.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }
    dp[n]
}

fn word_break_all(s: &str, dict: &[&str]) -> Vec<String> {
    let dict_set: HashSet<&str> = dict.iter().copied().collect();
    let n = s.len();
    let mut dp   = vec![false; n + 1];
    let mut prev: Vec<Vec<usize>> = vec![vec![]; n + 1];
    dp[0] = true;
    for i in 1..=n {
        for j in 0..i {
            if dp[j] && dict_set.contains(&s[j..i]) {
                dp[i] = true;
                prev[i].push(j);
            }
        }
    }

    fn collect(s: &str, prev: &Vec<Vec<usize>>, i: usize) -> Vec<String> {
        if i == 0 { return vec![String::new()]; }
        let mut results = Vec::new();
        for &j in &prev[i] {
            let word = &s[j..i];
            for base in collect(s, prev, j) {
                if base.is_empty() {
                    results.push(word.to_string());
                } else {
                    results.push(format!("{base} {word}"));
                }
            }
        }
        results
    }

    if dp[n] { collect(s, &prev, n) } else { vec![] }
}

fn main() {
    let dict = vec!["leet", "code", "cats", "and", "sand", "dog", "cat"];
    println!("'leetcode':    {}", word_break("leetcode", &dict));
    println!("'catsanddog': {}", word_break("catsanddog", &dict));
    println!("'catsanddogx':{}", word_break("catsanddogx", &dict));

    let sentences = word_break_all("catsanddog", &["cat", "cats", "and", "sand", "dog"]);
    println!("All parses of 'catsanddog': {:?}", sentences);
}
