/// Manacher's Algorithm — O(n) longest palindromic substring.
///
/// Key idea: transform "abc" → "#a#b#c#" so every palindrome is odd-length,
/// then maintain a (centre, right) Z-box to reuse mirror symmetry.

/// Transform original string: insert '#' sentinels.
/// "racecar" → "#r#a#c#e#c#a#r#"
fn transform(s: &str) -> Vec<u8> {
    let mut t = Vec::with_capacity(2 * s.len() + 1);
    t.push(b'#');
    for b in s.bytes() {
        t.push(b);
        t.push(b'#');
    }
    t
}

/// Compute the palindrome radius array P where P[i] = radius in transformed string.
fn manacher(t: &[u8]) -> Vec<usize> {
    let n = t.len();
    let mut p = vec![0usize; n];
    let (mut c, mut r) = (0usize, 0usize);

    for i in 0..n {
        // Mirror of i with respect to centre c
        if i < r {
            let mirror = 2 * c - i;
            p[i] = p[mirror].min(r - i);
        }
        // Expand around i
        let mut a = p[i] + 1;
        while i >= a && i + a < n && t[i - a] == t[i + a] {
            a += 1;
        }
        p[i] = a - 1;
        // Update rightmost palindrome
        if i + p[i] > r {
            c = i;
            r = i + p[i];
        }
    }
    p
}

/// Return the longest palindromic substring.
fn longest_palindrome(s: &str) -> &str {
    if s.is_empty() {
        return s;
    }
    let t = transform(s);
    let p = manacher(&t);

    // Find centre with maximum radius
    let (best_c, best_r) = p
        .iter()
        .enumerate()
        .max_by_key(|&(_, &r)| r)
        .map(|(i, &r)| (i, r))
        .unwrap();

    // Map transformed centre back to original string index
    // t[i] corresponds to original[(i-1)/2] if i is odd
    let start = (best_c - best_r) / 2;
    &s[start..start + best_r]
}

fn main() {
    let cases = [
        "babad",
        "cbbd",
        "racecar",
        "abacaba",
        "a",
        "aabbaa",
        "aaaa",
        "abcde",
    ];
    for s in &cases {
        println!("longest_palindrome({s:?}) = {:?}", longest_palindrome(s));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // Both "bab" and "aba" are valid for "babad"
        let result = longest_palindrome("babad");
        assert!(result == "bab" || result == "aba");
    }

    #[test]
    fn test_even_palindrome() {
        assert_eq!(longest_palindrome("cbbd"), "bb");
    }

    #[test]
    fn test_full_palindrome() {
        assert_eq!(longest_palindrome("racecar"), "racecar");
    }

    #[test]
    fn test_odd_palindrome() {
        assert_eq!(longest_palindrome("abacaba"), "abacaba");
    }

    #[test]
    fn test_single_char() {
        assert_eq!(longest_palindrome("a"), "a");
    }

    #[test]
    fn test_even_full() {
        assert_eq!(longest_palindrome("aabbaa"), "aabbaa");
    }

    #[test]
    fn test_all_same() {
        assert_eq!(longest_palindrome("aaaa"), "aaaa");
    }

    #[test]
    fn test_no_palindrome_longer_than_1() {
        let result = longest_palindrome("abcde");
        assert_eq!(result.len(), 1); // Any single char is a palindrome
    }

    #[test]
    fn test_empty() {
        assert_eq!(longest_palindrome(""), "");
    }

    #[test]
    fn test_transform() {
        assert_eq!(transform("abc"), b"#a#b#c#");
    }
}
