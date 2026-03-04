# 819: Z-Algorithm

**Difficulty:** 3  **Level:** Intermediate

Compute, in linear time, how far each position in a string matches the string's own prefix — then use it for O(n + m) pattern matching with a single array.

## The Problem This Solves

The Z-algorithm is KMP's conceptual sibling: both achieve O(n + m) pattern matching, both precompute reuse information to avoid backtracking. The Z-array is often easier to reason about because it's a direct measurement — "the string starting here matches the first Z[i] characters of the whole string" — rather than the more abstract failure-function of KMP.

Beyond pattern matching, the Z-array itself is a building block: detect if a string is a rotation of another, find the shortest period of a string, check if a string has a prefix that is also a suffix. It's a clean primitive that appears in competitive programming problems where KMP would also work but Z is more straightforward to implement correctly under pressure.

For systems programmers: the Z-algorithm processes input in a single left-to-right pass, making it friendly for streaming and cache-efficient computation.

## The Intuition

Maintain a "Z-box" `[l, r)`: the rightmost interval where the string matches a prefix. For each new position `i`: if `i < r`, initialize `Z[i]` from the mirror position `Z[i-l]` (bounded by how far the box extends). Then expand by comparing characters. Update the Z-box if the new match extends further right. Each character is touched at most twice: once during expansion, once as a mirror lookup — giving O(n) total.

For pattern search, concatenate `pattern + '$' + text`. Any position in the text portion where `Z[i] == len(pattern)` is a match. The sentinel `'$'` prevents Z-values from crossing the boundary between pattern and text.

## How It Works in Rust

```rust
fn z_array(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0usize; n];
    z[0] = n;                    // Convention: z[0] = length of whole string
    let (mut l, mut r) = (0usize, 0usize);

    for i in 1..n {
        if i < r {
            // Mirror: reuse z[i-l], but don't go past the Z-box boundary
            z[i] = z[i - l].min(r - i);
        }
        // Expand: compare s[z[i]] against s[i + z[i]]
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        // Update Z-box if new match extends further right
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

fn z_search(pattern: &str, text: &str) -> Vec<usize> {
    let m = pattern.len();
    // Sentinel '$' must not appear in pattern or text
    let combined: Vec<u8> = pattern.bytes()
        .chain(std::iter::once(b'$'))
        .chain(text.bytes())
        .collect();
    let z = z_array(&combined);
    // Positions in text where z[i] == m are match starts
    z.iter()
        .enumerate()
        .skip(m + 1)               // Skip pattern + sentinel
        .filter_map(|(i, &zi)| if zi == m { Some(i - m - 1) } else { None })
        .collect()
}
```

The iterator chain for building the combined string is idiomatic Rust: `chain` composes without allocation until `collect()`.

## What This Unlocks

- **String period detection**: The shortest period of a string is the first position `i` where `i + Z[i] == n` — useful in run-length compression and DNA tandem repeat analysis.
- **Rotation detection**: `s` is a rotation of `t` iff `s` appears in `t + t` — find it with Z in O(n).
- **Competitive programming**: Z is often the cleaner choice over KMP for problems where you need the full prefix-match lengths, not just match positions.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Array mutation | `Array.set arr i v` | `z[i] = v` — direct indexing |
| String as bytes | `Bytes.of_string` or `String.to_seq` | `.bytes()` iterator — zero-copy |
| Sentinel concat | `pattern ^ "$" ^ text` | `.chain(once(b'$')).chain(text.bytes())` |
| Z-box update | `let l = ref 0; let r = ref 0` | `let (mut l, mut r) = (0, 0)` |
| Filter positions | `Array.to_seq |> Seq.filter_map` | `.enumerate().filter_map(...)` |
