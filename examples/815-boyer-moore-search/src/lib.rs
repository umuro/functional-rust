//! # Boyer-Moore String Search
pub fn boyer_moore(text: &str, pattern: &str) -> Vec<usize> {
    let t: Vec<char> = text.chars().collect();
    let p: Vec<char> = pattern.chars().collect();
    let (n, m) = (t.len(), p.len());
    if m == 0 || m > n {
        return vec![];
    }

    let mut bad_char = std::collections::HashMap::new();
    for (i, &c) in p.iter().enumerate() {
        bad_char.insert(c, i);
    }

    let mut results = vec![];
    let mut s = 0;
    while s <= n - m {
        let mut j = m - 1;
        while j < m && p[j] == t[s + j] {
            if j == 0 {
                break;
            }
            j -= 1;
        }
        if j == 0 && p[0] == t[s] {
            results.push(s);
            s += 1;
        } else {
            s += (j.saturating_sub(*bad_char.get(&t[s + j]).unwrap_or(&0))).max(1);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bm() {
        assert!(!boyer_moore("ABAAABCD", "ABC").is_empty());
    }
}
