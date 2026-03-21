📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1125-reverse-string)**

---

# 1125-reverse-string — Reverse a String
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Reversing a string is a fundamental exercise that reveals important differences between languages in how they handle Unicode text. Reversing the bytes of a UTF-8 string produces garbage — multi-byte characters become invalid. Reversing by characters (Unicode scalar values) is correct for most purposes. Reversing by grapheme clusters (the human-perceived characters, e.g., emoji with skin tone modifiers) requires the `unicode-segmentation` crate.

This simple operation exposes Rust's explicit Unicode handling, contrasting with OCaml's byte-string model.

## Learning Outcomes

- Reverse a string correctly using `chars().rev().collect()`
- Understand why byte-level reversal is incorrect for UTF-8
- Know the difference between bytes, chars (Unicode scalar values), and grapheme clusters
- Use `String::chars()` for Unicode-correct character iteration
- Connect to the `unicode-segmentation` crate for grapheme-level operations

## Rust Application

```rust
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn reverse_words(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}
```

The `chars()` iterator yields Unicode scalar values (code points), making the reversal Unicode-correct for most strings. Emoji with zero-width joiners (e.g., family emoji) require grapheme cluster reversal via the `unicode-segmentation` crate.

## OCaml Approach

```ocaml
(* OCaml strings are byte sequences — correct only for ASCII *)
let reverse s =
  let n = String.length s in
  String.init n (fun i -> s.[n - 1 - i])

(* For Unicode, use the uutf library *)
let reverse_unicode s =
  let buf = Buffer.create (String.length s) in
  Uutf.String.fold_utf_8 (fun acc _ _ -> acc) () s;
  (* ... complex with uutf *)
```

OCaml strings are byte arrays — `String.length` returns byte count, not character count. For Unicode reversal, the `uutf` library provides UTF-8 decoding.

## Key Differences

1. **Unicode by default**: Rust's `String` is guaranteed UTF-8; `chars()` iterates Unicode scalar values. OCaml strings are byte arrays with no Unicode guarantee.
2. **Character counting**: Rust's `s.chars().count()` counts Unicode scalar values; `s.len()` counts bytes. OCaml's `String.length` counts bytes always.
3. **Grapheme clusters**: Neither language handles grapheme clusters in the standard library; Rust's `unicode-segmentation` crate and OCaml's `uucp` provide this.
4. **Performance**: Rust's `chars().rev().collect()` is O(n); OCaml's byte-level reversal is the same complexity but incorrect for non-ASCII.

## Exercises

1. Write `reverse_grapheme_clusters(s: &str) -> String` using the `unicode-segmentation` crate's `graphemes(true)`.
2. Implement `is_palindrome(s: &str) -> bool` that correctly handles Unicode by comparing the char sequence forward and backward.
3. Write `reverse_words_preserve_spaces(s: &str) -> String` that reverses word order while preserving the original whitespace pattern.
