//! # Run-Length Encoding — String Compression
//!
//! Encode consecutive repeated characters as count+char.
//! OCaml uses `Buffer` for incremental string building; Rust uses `String` directly.

// ---------------------------------------------------------------------------
// Approach A: Imperative — iterate with tracking
// ---------------------------------------------------------------------------

pub fn encode(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut count = 1;

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
        } else {
            if count > 1 {
                result.push_str(&count.to_string());
            }
            result.push(chars[i - 1]);
            count = 1;
        }
    }
    if count > 1 {
        result.push_str(&count.to_string());
    }
    result.push(*chars.last().unwrap());
    result
}

// ---------------------------------------------------------------------------
// Approach B: Iterator — chunk_by (nightly) or manual grouping
// ---------------------------------------------------------------------------

pub fn encode_functional(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let chars: Vec<char> = s.chars().collect();
    let mut groups: Vec<(char, usize)> = vec![];
    for &c in &chars {
        match groups.last_mut() {
            Some((last, count)) if *last == c => *count += 1,
            _ => groups.push((c, 1)),
        }
    }
    groups
        .iter()
        .map(|&(c, n)| {
            if n > 1 {
                format!("{}{}", n, c)
            } else {
                c.to_string()
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Approach C: Fold-based grouping
// ---------------------------------------------------------------------------

pub fn encode_fold(s: &str) -> String {
    s.chars()
        .fold(Vec::<(char, usize)>::new(), |mut acc, c| {
            match acc.last_mut() {
                Some((last, count)) if *last == c => *count += 1,
                _ => acc.push((c, 1)),
            }
            acc
        })
        .iter()
        .map(|&(c, n)| {
            if n > 1 {
                format!("{}{}", n, c)
            } else {
                c.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(encode("AABCCCDEEEE"), "2AB3CD4E");
    }

    #[test]
    fn test_no_repeats() {
        assert_eq!(encode("ABCDE"), "ABCDE");
    }

    #[test]
    fn test_all_same() {
        assert_eq!(encode("AAAA"), "4A");
    }

    #[test]
    fn test_empty() {
        assert_eq!(encode(""), "");
    }

    #[test]
    fn test_single() {
        assert_eq!(encode("A"), "A");
    }

    #[test]
    fn test_functional() {
        assert_eq!(encode_functional("AABCCCDEEEE"), "2AB3CD4E");
    }

    #[test]
    fn test_fold() {
        assert_eq!(encode_fold("AABCCCDEEEE"), "2AB3CD4E");
    }
}
