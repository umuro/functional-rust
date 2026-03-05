/// Polynomial Rolling Hash for strings.
///
/// hash(s) = s[0]*B^(n-1) + s[1]*B^(n-2) + ... + s[n-1] (mod P)
/// Prefix array lets any substring hash be queried in O(1).

const BASE: u64 = 31;
const MOD: u64 = 1_000_000_007;

struct RollingHash {
    prefix: Vec<u64>, // prefix[i] = hash(s[0..i])
    powers: Vec<u64>, // powers[i] = BASE^i mod MOD
}

impl RollingHash {
    /// Build from a string of lowercase ASCII.
    fn new(s: &str) -> Self {
        let n = s.len();
        let mut prefix = vec![0u64; n + 1];
        let mut powers = vec![1u64; n + 1];
        for (i, b) in s.bytes().enumerate() {
            let c = (b - b'a' + 1) as u64;
            prefix[i + 1] = (prefix[i] * BASE + c) % MOD;
            powers[i + 1] = powers[i] * BASE % MOD;
        }
        Self { prefix, powers }
    }

    /// Hash of s[l..r] (0-indexed, exclusive r). O(1).
    fn query(&self, l: usize, r: usize) -> u64 {
        let len = r - l;
        (self.prefix[r] + MOD * MOD - self.prefix[l] * self.powers[len] % MOD) % MOD
    }
}

/// Rabin-Karp pattern search. Returns 0-based positions.
fn rabin_karp(pattern: &str, text: &str) -> Vec<usize> {
    let m = pattern.len();
    let n = text.len();
    if m > n {
        return vec![];
    }
    let ph = RollingHash::new(pattern);
    let th = RollingHash::new(text);
    let pat_hash = ph.query(0, m);
    let pattern_bytes = pattern.as_bytes();
    let text_bytes = text.as_bytes();

    (0..=n - m)
        .filter(|&i| {
            th.query(i, i + m) == pat_hash
                && &text_bytes[i..i + m] == pattern_bytes // verify on hash match
        })
        .collect()
}

/// Check if two substrings are equal in O(1) using hashes.
fn substring_eq(h1: &RollingHash, l1: usize, r1: usize,
                h2: &RollingHash, l2: usize, r2: usize) -> bool {
    r1 - l1 == r2 - l2 && h1.query(l1, r1) == h2.query(l2, r2)
}

fn main() {
    // Rabin-Karp demo
    let text = "abcabcabc";
    let pattern = "abc";
    let positions = rabin_karp(pattern, text);
    println!("rabin_karp({pattern:?} in {text:?}): {positions:?}");

    // Substring equality
    let h = RollingHash::new("abcabc");
    println!("hash('abc'@0) == hash('abc'@3): {}", substring_eq(&h, 0, 3, &h, 3, 6));
    println!("hash('abc'@0) == hash('bca'@1): {}", substring_eq(&h, 0, 3, &h, 1, 4));

    // Longest repeated substring hint via hashing
    let s = "banana";
    let hs = RollingHash::new(s);
    let n = s.len();
    println!("\nSubstring hashes in 'banana':");
    for len in 1..=3 {
        for i in 0..=n - len {
            print!("  [{i}..{}] {:?} hash={}", i + len, &s[i..i + len], hs.query(i, i + len));
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rabin_karp_multiple() {
        assert_eq!(rabin_karp("abc", "abcabcabc"), vec![0, 3, 6]);
    }

    #[test]
    fn test_rabin_karp_not_found() {
        assert_eq!(rabin_karp("xyz", "abcdef"), vec![]);
    }

    #[test]
    fn test_rabin_karp_single() {
        assert_eq!(rabin_karp("abc", "abc"), vec![0]);
    }

    #[test]
    fn test_rabin_karp_overlapping() {
        assert_eq!(rabin_karp("aa", "aaaa"), vec![0, 1, 2]);
    }

    #[test]
    fn test_substring_eq_same() {
        let h = RollingHash::new("abcabc");
        assert!(substring_eq(&h, 0, 3, &h, 3, 6));
    }

    #[test]
    fn test_substring_neq() {
        let h = RollingHash::new("abcabc");
        assert!(!substring_eq(&h, 0, 3, &h, 1, 4));
    }

    #[test]
    fn test_full_string_hash_consistent() {
        let h = RollingHash::new("hello");
        // Same content → same hash
        let h2 = RollingHash::new("hello");
        assert_eq!(h.query(0, 5), h2.query(0, 5));
    }

    #[test]
    fn test_different_strings_different_hash() {
        let h1 = RollingHash::new("abc");
        let h2 = RollingHash::new("abd");
        assert_ne!(h1.query(0, 3), h2.query(0, 3));
    }
}
