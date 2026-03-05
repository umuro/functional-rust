# 818: Suffix Array Construction O(n log n)

**Difficulty:** 5  **Level:** Master

Sort all suffixes of a string to enable O(m log n) pattern search and unlock a family of O(n) string problems via the LCP array.

## The Problem This Solves

For a single pattern search, KMP or BMH suffice. But when you need to search the *same text* for many different patterns — a search engine index, a genome browser, a compressed archive — you want a data structure built once that answers any query fast. A suffix array is that structure: after O(n log n) construction, any pattern of length m is found with binary search in O(m log n).

The LCP (Longest Common Prefix) array, built alongside the SA in O(n) via Kasai's algorithm, turns the SA into an even more powerful tool. With SA + LCP you can count distinct substrings, find the longest repeated substring, solve longest common substring for multiple strings, and compress repetitive data — all in O(n) or O(n log n). These problems are foundational in bioinformatics (genome assembly), data compression (BWT/FM-index), and search engine construction.

The simpler O(n log² n) prefix-doubling approach (implemented here) is almost always fast enough in practice and far easier to understand than the linear SA-IS algorithm. Knowing when "simpler and fast enough" beats "theoretically optimal" is itself a valuable engineering judgment.

## The Intuition

Prefix doubling: in round k, sort suffixes by their first 2^k characters. Represent each suffix as a pair `(rank[i], rank[i + 2^(k-1)])` — the rank of its first half and its second half. After ⌈log n⌉ rounds, the pairs uniquely identify every suffix. Each round is O(n log n) with comparison sort, giving O(n log² n) total. Binary search on the sorted array finds any pattern in O(m log n).

In OCaml, `Array.sort` with a comparison closure directly implements prefix doubling. Rust's `sort_unstable_by` does the same thing with better cache behavior.

## How It Works in Rust

```rust
fn build_sa(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();          // Initial: suffixes in order
    let mut rank: Vec<i64> = s.iter().map(|&c| c as i64).collect(); // Rank by first char
    let mut tmp  = vec![0i64; n];
    let mut gap  = 1usize;

    while gap < n {
        let g = gap;
        let rank_ref = &rank;
        // Sort by (rank[i], rank[i+gap]) — O(n log n) per round
        sa.sort_unstable_by(|&i, &j| {
            let ri = rank_ref[i];
            let rj = rank_ref[j];
            if ri != rj { return ri.cmp(&rj); }
            let ri2 = if i + g < n { rank_ref[i + g] } else { -1 };
            let rj2 = if j + g < n { rank_ref[j + g] } else { -1 };
            ri2.cmp(&rj2)
        });
        // Rebuild ranks: assign 0 to first, increment when adjacent pair differs
        tmp[sa[0]] = 0;
        for i in 1..n {
            let (pi, ci) = (sa[i - 1], sa[i]);
            let same = rank[pi] == rank[ci]
                && (pi + g < n) == (ci + g < n)
                && (pi + g >= n || rank[pi + g] == rank[ci + g]);
            tmp[ci] = tmp[pi] + if same { 0 } else { 1 };
        }
        rank.copy_from_slice(&tmp);
        gap *= 2;  // Double the comparison length each round
    }
    sa
}

// Binary search on SA: O(m log n) per query
fn sa_search(s: &[u8], sa: &[usize], pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    // partition_point is std binary search — finds [left, right) range of matches
    let left  = sa.partition_point(|&i| &s[i..s.len().min(i + m)] < pattern);
    let right = sa.partition_point(|&i| &s[i..s.len().min(i + m)] <= pattern);
    let mut positions = sa[left..right].to_vec();
    positions.sort_unstable();  // Return in text order
    positions
}
```

`partition_point` (stable since Rust 1.52) is the idiomatic binary search for range queries — cleaner than a pair of `binary_search` calls.

## What This Unlocks

- **Search engines and genome browsers**: Build once, query any pattern in O(m log n); with FM-index on top of SA, query time drops to O(m).
- **Data compression**: Burrows-Wheeler Transform uses the suffix array; BWT + move-to-front + Huffman = bzip2.
- **Longest repeated substring / longest common substring**: SA + LCP array solves these in O(n) after O(n log n) build — fundamental in plagiarism detection and DNA alignment.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Suffix sort | `Array.sort` with `compare` closure | `sort_unstable_by` — no stability needed |
| Rank as pair | Tuple `(rank[i], rank[i+gap])` | Compared inline in closure |
| Rank update | `Array.init n (fun i -> ...)` | In-place `copy_from_slice` of temp buffer |
| Binary search | Manual or `Array.blit` | `slice.partition_point` for range queries |
| LCP construction | Kasai's algorithm | Identical: rank inverse + linear scan |
