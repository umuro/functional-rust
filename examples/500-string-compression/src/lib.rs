#![allow(clippy::all)]
//! # 500. String Compression — Run-Length Encoding
//! Encode/decode strings using run-length encoding via iterator fold.

/// Encode a string into (count, char) pairs.
/// "aabbbcc" -> vec![(2,'a'), (3,'b'), (2,'c')]
fn encode(s: &str) -> Vec<(usize, char)> {
    let mut chars = s.chars();
    let first = match chars.next() {
        None => return vec![],
        Some(c) => c,
    };
    let (cur, count, mut acc) =
        chars.fold((first, 1usize, Vec::new()), |(cur, count, mut acc), c| {
            if c == cur {
                (cur, count + 1, acc)
            } else {
                acc.push((count, cur));
                (c, 1, acc)
            }
        });
    acc.push((count, cur));
    acc
}

/// Decode (count, char) pairs back into a string.
fn decode(pairs: &[(usize, char)]) -> String {
    pairs.iter().fold(String::new(), |mut s, &(n, c)| {
        for _ in 0..n {
            s.push(c);
        }
        s
    })
}

/// Format encoded pairs as human-readable "2a3b2c"
fn show_encoded(pairs: &[(usize, char)]) -> String {
    pairs.iter().map(|(n, c)| format!("{}{}", n, c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_basic() {
        assert_eq!(encode("aabbbcc"), vec![(2, 'a'), (3, 'b'), (2, 'c')]);
    }

    #[test]
    fn test_encode_all_same() {
        assert_eq!(encode("aaaa"), vec![(4, 'a')]);
    }

    #[test]
    fn test_encode_no_repeats() {
        assert_eq!(
            encode("abcde"),
            vec![(1, 'a'), (1, 'b'), (1, 'c'), (1, 'd'), (1, 'e')]
        );
    }

    #[test]
    fn test_encode_empty() {
        assert_eq!(encode(""), vec![]);
    }

    #[test]
    fn test_roundtrip() {
        let inputs = ["aabbbcc", "aaaa", "abcde", "aabbcc", "z"];
        for s in &inputs {
            assert_eq!(decode(&encode(s)), *s, "roundtrip failed for {:?}", s);
        }
    }
}
