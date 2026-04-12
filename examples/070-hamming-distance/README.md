📖 **[View on hightechmind.io →](https://hightechmind.io/rust/070-hamming-distance)**

---

# 070 — Hamming Distance
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Hamming distance counts the number of positions at which two equal-length strings differ. Named after Richard Hamming, who introduced it in his landmark 1950 paper on error-detecting and error-correcting codes, the concept was foundational for reliable digital communication. A code with minimum Hamming distance d can detect up to d-1 bit errors and correct up to floor((d-1)/2) errors — the foundation of Hamming codes and modern ECC memory.

Applications span many fields: DNA sequence comparison in bioinformatics measures how far two genetic strands have diverged through mutation; error-correcting codes in storage and networking (Hamming codes, Reed-Solomon) rely on Hamming distance to detect and repair corruption; nearest-neighbor search in machine learning uses it to find similar binary feature vectors in LSH (locality-sensitive hashing); and cryptographic protocols analyze password similarity to enforce minimum edit distance policies. The `zip + filter + count` pipeline it demonstrates is one of the most fundamental iterator composition patterns in Rust, applicable whenever you need to compare two sequences element-by-element.

## Learning Outcomes

- Use `.zip()` to pair corresponding elements from two iterators into a stream of tuples
- Use `.filter(|(a, b)| a != b)` to select only the mismatched pairs before counting
- Return `Result<usize, String>` when inputs must have equal length, signaling the precondition violation explicitly
- Implement the same logic using both `filter().count()` and `fold` to compare styles
- Use byte-level comparison (`as_bytes()`) for ASCII performance, bypassing UTF-8 decoding
- Understand the connection to error-correcting codes and binary distance metrics

## Rust Application

`hamming_distance(s1: &str, s2: &str) -> Result<usize, String>` validates equal lengths first, returning `Err` if they differ. The pipeline `s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count()` reads cleanly:
1. `zip` pairs up characters from both strings
2. `filter` keeps only the pairs that differ
3. `count` tallies the mismatches

`hamming_fold` replaces `filter().count()` with `fold(0, |acc, (a, b)| if a != b { acc + 1 } else { acc })`, showing the accumulator pattern explicitly. Both are O(n).

`hamming_bytes` accepts `&[u8]` slices for ASCII-only inputs (DNA sequences, binary strings), avoiding UTF-8 decoding overhead entirely.

## OCaml Approach

OCaml uses `String.fold_left2` (OCaml 4.14+) to visit corresponding characters:

```ocaml
let hamming s1 s2 =
  if String.length s1 <> String.length s2
  then Error "unequal lengths"
  else Ok (String.fold_left2
             (fun acc c1 c2 -> if c1 = c2 then acc else acc + 1)
             0 s1 s2)
```

Earlier versions convert to `List.of_seq` and use `List.fold_left2`. OCaml's `String` is byte-indexed so characters are always single bytes — no Unicode code-point complexity. The error result type is `(int, string) result`, matching Rust's `Result<usize, String>` structurally.

## Key Differences

1. **`fold_left2` vs `zip`**: OCaml's `List.fold_left2` processes two lists simultaneously. Rust's `zip` creates an iterator of pairs, then processes with `filter + count` or `fold`. Both are O(n).
2. **Error type**: Both return `Result` for the length mismatch. Rust: `Result<usize, String>`. OCaml: `(int, string) result`. The pattern is identical.
3. **Byte vs char**: Rust's `as_bytes()` comparison is O(n) without Unicode overhead. OCaml's `String.get` returns bytes too. Both should use byte comparison for ASCII input.
4. **`chars().zip(chars())`**: Creates two `Chars` iterators that advance in sync. Rust's `zip` is lazy and stops at the shorter iterator — the earlier length check ensures they are equal.

## Exercises

1. **Maximum in population**: Given a list of DNA strands, find the pair with the maximum Hamming distance. Use combinations from example 026 or nested loops.
2. **Error correction**: Given a received codeword and a set of valid codewords, find the closest valid codeword by minimum Hamming distance. This is the basis of hard-decision decoding.
3. **XOR shortcut**: For binary strings (only '0' and '1'), implement Hamming distance using XOR: `s1.bytes().zip(s2.bytes()).filter(|(a, b)| a ^ b != 0).count()`. How does this compare for performance?
