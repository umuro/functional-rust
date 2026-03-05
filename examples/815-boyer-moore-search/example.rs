// Boyer-Moore-Horspool string search — sublinear average case

fn build_shift(pattern: &[u8]) -> [usize; 256] {
    let m = pattern.len();
    let mut shift = [m; 256];
    for i in 0..m.saturating_sub(1) {
        shift[pattern[i] as usize] = m - 1 - i;
    }
    shift
}

fn bmh_search(text: &str, pattern: &str) -> Vec<usize> {
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    let (n, m) = (t.len(), p.len());
    if m == 0 || m > n { return vec![]; }

    let shift   = build_shift(p);
    let mut matches = Vec::new();
    let mut pos = 0;

    while pos + m <= n {
        // Compare right-to-left
        let mut j = m;
        while j > 0 && p[j - 1] == t[pos + j - 1] { j -= 1; }
        if j == 0 { matches.push(pos); }
        pos += shift[t[pos + m - 1] as usize];
    }
    matches
}

fn main() {
    let text = "ABAAABCDABABCABAB";
    let pat  = "ABAB";
    println!("Text:    {text:?}");
    println!("Pattern: {pat:?}");
    println!("Matches: {:?}", bmh_search(text, pat));

    println!("\n\"aaa\" in \"aaaaaaaaaa\": {:?}", bmh_search("aaaaaaaaaa", "aaa"));
    println!("No match: {:?}", bmh_search("abcdef", "xyz"));
}
