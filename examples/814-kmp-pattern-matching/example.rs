// KMP Pattern Matching — O(n+m)

fn build_prefix(pattern: &[u8]) -> Vec<usize> {
    let m  = pattern.len();
    let mut pi = vec![0usize; m];
    let mut k  = 0usize;
    for i in 1..m {
        while k > 0 && pattern[k] != pattern[i] { k = pi[k - 1]; }
        if pattern[k] == pattern[i] { k += 1; }
        pi[i] = k;
    }
    pi
}

fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    let (n, m) = (t.len(), p.len());
    if m == 0 { return vec![]; }

    let pi  = build_prefix(p);
    let mut matches = Vec::new();
    let mut q = 0usize;

    for i in 0..n {
        while q > 0 && p[q] != t[i] { q = pi[q - 1]; }
        if p[q] == t[i] { q += 1; }
        if q == m {
            matches.push(i + 1 - m);
            q = pi[m - 1];
        }
    }
    matches
}

fn main() {
    let text    = "aabaacaadaabaaba";
    let pattern = "aaba";
    println!("Text:    {text:?}");
    println!("Pattern: {pattern:?}");
    println!("Matches: {:?}", kmp_search(text, pattern));

    println!("\"abc\" in \"abcabcabc\": {:?}", kmp_search("abcabcabc", "abc"));
    println!("Empty pattern: {:?}", kmp_search("hello", ""));
}
