//! # Rabin-Karp Algorithm
//! Rolling hash string matching

const BASE: u64 = 256;
const MOD: u64 = 1_000_000_007;

pub fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    let (t, p) = (text.as_bytes(), pattern.as_bytes());
    if p.is_empty() || p.len() > t.len() { return vec![]; }
    
    let m = p.len();
    let mut pattern_hash = 0u64;
    let mut text_hash = 0u64;
    let mut h = 1u64;
    
    for _ in 0..m - 1 { h = (h * BASE) % MOD; }
    for i in 0..m {
        pattern_hash = (pattern_hash * BASE + p[i] as u64) % MOD;
        text_hash = (text_hash * BASE + t[i] as u64) % MOD;
    }
    
    let mut result = vec![];
    for i in 0..=t.len() - m {
        if pattern_hash == text_hash && &t[i..i + m] == p { result.push(i); }
        if i < t.len() - m {
            text_hash = ((text_hash + MOD - (t[i] as u64 * h) % MOD) * BASE + t[i + m] as u64) % MOD;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rk() { assert_eq!(rabin_karp("abcabc", "abc"), vec![0, 3]); }
}
