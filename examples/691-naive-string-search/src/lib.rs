//! # Naive String Search
//! Simple O(n*m) pattern matching

pub fn search(text: &str, pattern: &str) -> Vec<usize> {
    let (t, p) = (text.as_bytes(), pattern.as_bytes());
    if p.is_empty() || p.len() > t.len() { return vec![]; }
    (0..=t.len() - p.len()).filter(|&i| t[i..i + p.len()] == *p).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search() {
        assert_eq!(search("abcabc", "abc"), vec![0, 3]);
        assert_eq!(search("aaaa", "aa"), vec![0, 1, 2]);
    }
}
