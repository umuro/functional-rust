# 815: Boyer-Moore-Horspool String Search

**Difficulty:** 4  **Level:** Advanced

Sublinear average-case string search: skip whole pattern-lengths at a time using a precomputed bad-character table.

## The Problem This Solves

KMP achieves O(n + m) by never backing up — but it still examines every character of the text at least once. Boyer-Moore goes further: it can skip entire sections of text without examining them, achieving O(n/m) average case. For long patterns on large texts (e.g., searching source code for a 20-character identifier), this is a dramatic practical speedup.

Boyer-Moore-Horspool is the simplified variant that keeps only the bad-character heuristic. It's easier to implement correctly and fast enough for the overwhelming majority of real use cases. The full Boyer-Moore adds the good-suffix rule for additional worst-case guarantees, but BMH is what you reach for in practice: file search tools like `grep` use Boyer-Moore variants internally.

The trade-off: BMH has O(n×m) worst case (repeated single-character patterns in repeated text), while KMP is always O(n + m). Choose BMH when patterns are long and the alphabet is large (English text, source code), choose KMP when patterns may be repetitive or worst-case guarantees matter.

## The Intuition

Compare right-to-left within the window. When a mismatch occurs at the rightmost position, look at the text character there and shift the pattern so that character aligns with its last occurrence in the pattern. The shift table `skip[c]` is built once in O(m): for each pattern character at position `i` (not the last), `skip[c] = m - 1 - i`. The default shift for characters not in the pattern is `m` — skip the entire pattern width. O(n/m) average because each shift advances by roughly `m/2` on random text.

OCaml builds the same table with `Array.make 256 m`; Rust uses `[usize; 256]` on the stack for cache-friendly constant-time access.

## How It Works in Rust

```rust
// Build the shift (bad-character) table in O(m)
fn build_shift(pattern: &[u8]) -> [usize; 256] {
    let m = pattern.len();
    let mut shift = [m; 256];  // Default: shift by full pattern length
    // All characters except the last get a shift based on their rightmost position
    for i in 0..m.saturating_sub(1) {
        shift[pattern[i] as usize] = m - 1 - i;
    }
    shift
}

fn bmh_search(text: &str, pattern: &str) -> Vec<usize> {
    let (t, p) = (text.as_bytes(), pattern.as_bytes());
    let (n, m) = (t.len(), p.len());
    if m == 0 || m > n { return vec![]; }

    let shift = build_shift(p);
    let mut matches = Vec::new();
    let mut pos = 0;

    while pos + m <= n {
        let mut j = m;
        // Compare right-to-left: first mismatch triggers the shift
        while j > 0 && p[j - 1] == t[pos + j - 1] { j -= 1; }
        if j == 0 { matches.push(pos); }
        // Always shift by the bad-character value of the rightmost text char
        pos += shift[t[pos + m - 1] as usize];
    }
    matches
}
```

The `[usize; 256]` stack array is a key Rust performance idiom: fixed-size, no heap allocation, fits in L1 cache.

## What This Unlocks

- **Text editors and IDEs**: Fast find-in-file; Visual Studio Code's search uses Boyer-Moore-family algorithms.
- **Network packet inspection**: Match signatures in high-throughput streams where O(n/m) matters; Snort IDS uses multi-pattern Boyer-Moore.
- **`grep` and file utilities**: The foundation of pattern-matching in CLI tools when the pattern is a literal string.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Shift table | `Array.make 256 m` | `[usize; 256]` — stack-allocated, no heap |
| Char to index | `Char.code c` | `c as usize` |
| Right-to-left compare | `for j = m-1 downto 0` | `while j > 0 { j -= 1; }` |
| Shift on mismatch | `shift.(Char.code text.[pos+m-1])` | `shift[t[pos + m - 1] as usize]` |
| Pattern not found | Returns empty list | Returns empty `Vec` |
