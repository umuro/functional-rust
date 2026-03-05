// Rabin-Karp Rolling Hash Search — O(n+m) expected

const BASE:  u64 = 256;
const PRIME: u64 = 1_000_000_007;

fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    let (n, m) = (t.len(), p.len());
    if m == 0 || m > n { return vec![]; }

    // Compute base^(m-1) mod prime
    let mut pow = 1u64;
    for _ in 0..m - 1 { pow = pow * BASE % PRIME; }

    // Initial hashes
    let mut hash_p = 0u64;
    let mut hash_t = 0u64;
    for i in 0..m {
        hash_p = (hash_p * BASE + p[i] as u64) % PRIME;
        hash_t = (hash_t * BASE + t[i] as u64) % PRIME;
    }

    let mut matches = Vec::new();
    for i in 0..=n - m {
        if hash_t == hash_p && &t[i..i + m] == p {
            matches.push(i);
        }
        if i < n - m {
            hash_t = (hash_t + PRIME - t[i] as u64 * pow % PRIME) % PRIME;
            hash_t = (hash_t * BASE + t[i + m] as u64) % PRIME;
        }
    }
    matches
}

fn main() {
    let text = "ABCDABABCDABCDAB";
    let pat  = "ABCD";
    println!("Text:    {text:?}");
    println!("Pattern: {pat:?}");
    println!("Matches: {:?}", rabin_karp(text, pat));

    println!("\"aab\" in \"aaabaaabaa\": {:?}", rabin_karp("aaabaaabaa", "aab"));
    println!("No match: {:?}", rabin_karp("abcdef", "xyz"));
}
