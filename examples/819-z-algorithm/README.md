📖 **[View on hightechmind.io →](https://hightechmind.io/rust/819-z-algorithm)**

---

# Z-Algorithm String Matching
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

The Z-algorithm computes, for each position in a string, the length of the longest substring starting at that position that is also a prefix of the whole string. This Z-array answers pattern matching questions directly: concatenate `pattern + "$" + text`, compute the Z-array, and any position where Z[i] equals the pattern length is a match. The algorithm runs in O(n) time and O(n) space, making it a linear alternative to KMP with a simpler conceptual model. It powers prefix-related string queries in competitive programming, bioinformatics repeat analysis, and text compression preprocessing.

## Learning Outcomes

- Understand the Z-box (l, r) maintenance: a window tracking the rightmost known matching prefix
- Implement O(1) amortized extension using the existing Z-box to skip recomputation
- Recognize the pattern matching reduction: `P + $ + T` → find positions where `Z[i] == |P|`
- Learn the difference from KMP failure function: Z measures prefix matches at each position directly
- Apply Z-array to: string periods, pattern counting, lexicographically smallest rotation

## Rust Application

```rust
pub fn z_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0usize; n];
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i < r { z[i] = z[i - l].min(r - i); }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] { z[i] += 1; }
        if i + z[i] > r { l = i; r = i + z[i]; }
    }
    z
}
```

The implementation operates on `&[u8]` for byte-level efficiency. The Z-box `(l, r)` stores the leftmost start and one-past-end of the rightmost-extending known match. For position `i` inside the box, `z[i - l]` gives a safe lower bound without recomputation. Rust's array indexing with bounds checking in debug mode catches off-by-one errors during development. The separator `b'$'` in the concatenated slice must not appear in pattern or text for correctness — typically a byte value of 0 or a sentinel outside the alphabet.

## OCaml Approach

OCaml implements the Z-function with mutable `int ref` for `l` and `r`, or threads them through a tail-recursive loop. `Bytes.get` provides O(1) character access. The `Array.make n 0` creates the Z-array. OCaml's pattern matching on the Z-box condition `i < r` maps cleanly to `if i < !r then`. For string matching, `Bytes.concat Bytes.empty [pattern; sep; text]` creates the concatenated input. The result scan `Array.to_seq |> Seq.filter_mapi` finds positions where Z equals pattern length.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Input type | `&[u8]` — byte slice | `bytes` or `string` |
| Z-box state | Two `usize` variables | `int ref` or threaded recursion |
| Concatenation | `[pattern, b'$', text].concat()` | `Bytes.concat` |
| Bound checks | Debug-mode auto-checks | `Bytes.get` raises exception |
| Result filter | `enumerate().filter()` | `Array.to_seq \|> Seq.filter_mapi` |
| Separator byte | Must not appear in input | Same constraint |

## Exercises

1. Use the Z-array to find the shortest period of a string (smallest k such that the string is made of repeating k-length prefix).
2. Find all occurrences of a pattern in text and return start/end pairs using the Z-function approach.
3. Implement lexicographically smallest rotation: find i that minimizes `s[i..] + s[..i]`.
4. Compare Z-algorithm and KMP on identical inputs and verify they produce consistent results.
5. Use the Z-array to count distinct substrings that are prefixes of the string.
