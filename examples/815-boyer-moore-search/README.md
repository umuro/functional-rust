📖 **[View on hightechmind.io →](https://hightechmind.io/rust/815-boyer-moore-search)**

---

# Boyer-Moore String Search
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Naive string search checks every position in the text, yielding O(n*m) worst-case time. For long texts with long patterns this is prohibitively slow. Real-world applications — log scanning, virus signature detection, text editors, DNA sequence analysis — require sublinear average-case search. Boyer-Moore achieves this by using two heuristics: the bad-character rule skips large chunks of text when a mismatch occurs at the wrong character position, and the good-suffix rule leverages known pattern structure to skip even further. The result is often O(n/m) average-case performance, making Boyer-Moore the algorithm behind `grep` and many production text search tools.

## Learning Outcomes

- Understand the bad-character heuristic and how it allows right-to-left scanning with large skips
- Implement a preprocessing phase that builds a shift table from the pattern
- Recognize why Boyer-Moore scans the pattern right-to-left even though it advances left-to-right through text
- Learn how the algorithm achieves sublinear average-case complexity
- Compare Boyer-Moore with KMP: Boyer-Moore is faster in practice for large alphabets, KMP has better worst-case

## Rust Application

```rust
pub fn boyer_moore(text: &str, pattern: &str) -> Vec<usize> {
    let t: Vec<char> = text.chars().collect();
    let p: Vec<char> = pattern.chars().collect();
    // bad_char table: for each character, last occurrence in pattern
    // skip = pattern_len - 1 - bad_char[text[i]]
}
```

The implementation collects chars upfront to handle Unicode correctly. The bad-character table maps each character to its rightmost occurrence in the pattern. When a mismatch occurs at position `i` in the text, we can skip by `pattern_len - 1 - bad_char[mismatch_char]` positions, advancing the pattern past the mismatch. Rust's `HashMap<char, usize>` stores this shift table naturally without alphabet-size assumptions, making it work for any Unicode input without a fixed alphabet array.

## OCaml Approach

In OCaml, Boyer-Moore is implemented using `Bytes` or `String` with `Char.code` indexing for O(1) character access. The bad-character table is often an `int array` of size 256 for ASCII. OCaml's tail recursion enables a clean recursive formulation of the scan loop. The `Buffer` module avoids string concatenation in preprocessing. Functors can parameterize over alphabet type, and the `Map` module serves the role of Rust's `HashMap`. OCaml's immutable-first style encourages returning a list of match positions rather than mutating a results vector.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Shift table | `HashMap<char, usize>` for Unicode | `int array` (256) for ASCII or `Map` |
| String indexing | `chars().collect()` to Vec | `String.get`/`Bytes.get` by index |
| Alphabet size | Unbounded (any Unicode) | Usually 256 for byte-level search |
| Result collection | `Vec<usize>` pushed in loop | Recursive list accumulation |
| Pattern scanning | Right-to-left inner loop | Same, via index arithmetic |
| Worst case | O(n*m) with bad-character only | Same without good-suffix rule |

## Exercises

1. Add the good-suffix heuristic and measure the improvement on repetitive text like `"aaaa...a"` searching for `"aaab"`.
2. Implement a case-insensitive version by normalizing to lowercase during the bad-character table build.
3. Benchmark Boyer-Moore against KMP on: short patterns in long text, long patterns in long text, and DNA sequences.
4. Extend to return `(usize, usize)` pairs of `(start, end)` positions instead of just start indices.
5. Handle the edge case where the pattern is longer than the text gracefully without panicking.
