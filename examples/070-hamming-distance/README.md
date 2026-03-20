📖 **[View on hightechmind.io →](https://hightechmind.io/rust/070-hamming-distance)**

---

# 070 — Hamming Distance

## Problem Statement

Hamming distance counts the number of positions at which two equal-length strings differ. Named after Richard Hamming (1950), it was originally developed for error detection in digital communication — a Hamming distance of 1 means strings differ at exactly one position, allowing single-bit error correction.

Applications: DNA sequence comparison (bioinformatics), error-correcting codes (Hamming codes, Reed-Solomon), nearest-neighbor search in machine learning, and password similarity analysis. The `zip + filter + count` pattern it demonstrates is one of the most common iterator patterns in Rust.

## Learning Outcomes

- Use `.zip()` to pair corresponding elements from two iterators
- Use `.filter(|(a, b)| a != b)` to count mismatches
- Return `Result` when inputs must have equal length
- Implement using both `filter().count()` and `fold` for comparison
- Use byte-level comparison (`as_bytes()`) for ASCII performance

## Rust Application

`hamming_distance(s1, s2)` checks lengths first (returning `Err` if unequal), then `s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count()`. `hamming_fold` uses `fold(0, |acc, (a, b)| if a != b { acc + 1 } else { acc })` — equivalent but explicit about accumulation. `hamming_bytes` compares `&[u8]` directly — faster for ASCII strings since `chars()` overhead is avoided.

## OCaml Approach

OCaml's version: `let hamming s1 s2 = if String.length s1 <> String.length s2 then Error "unequal lengths" else Ok (String.fold_left2 (fun acc c1 c2 -> if c1 = c2 then acc else acc + 1) 0 s1 s2)`. `String.fold_left2` (OCaml 4.14+) folds over two strings simultaneously. Earlier: `List.fold_left2 acc (String.to_seq_of_bytes s1 |> List.of_seq) (...)`.

## Key Differences

1. **`fold_left2` vs `zip`**: OCaml's `List.fold_left2` processes two lists simultaneously. Rust's `zip` creates an iterator of pairs, then processes with `filter + count` or `fold`. Both are O(n).
2. **Error type**: Both return `Result` for the length mismatch. Rust: `Result<usize, String>`. OCaml: `(int, string) result`. The pattern is identical.
3. **Byte vs char**: Rust's `as_bytes()` comparison is O(n) without Unicode overhead. OCaml's `String.get` returns bytes too. Both should use byte comparison for ASCII input.
4. **`chars().zip(chars())`**: Creates two `Chars` iterators that advance in sync. Rust's `zip` is lazy and stops at the shorter iterator — the earlier length check ensures they are equal.

## Exercises

1. **Maximum in population**: Given a list of DNA strands, find the pair with the maximum Hamming distance. Use combinations from example 026 or nested loops.
2. **Error correction**: Given a received codeword and a set of valid codewords, find the closest valid codeword by minimum Hamming distance. This is the basis of hard-decision decoding.
3. **XOR shortcut**: For binary strings (only '0' and '1'), implement Hamming distance using XOR: `s1.bytes().zip(s2.bytes()).filter(|(a, b)| a ^ b != 0).count()`. How does this compare for performance?
