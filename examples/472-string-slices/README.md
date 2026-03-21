📖 **[View on hightechmind.io →](https://hightechmind.io/rust/472-string-slices)**

---

# String Slices
**Difficulty:** ⭐  
**Category:** Functional Programming  



String slicing in Rust operates on UTF-8 byte boundaries, not character indices. Understanding the difference between byte offsets and character positions is essential for correct multibyte text handling.

## Problem Statement

In many languages, `str[2]` gives you the third character. In Rust, string slices are **byte ranges**. UTF-8 encodes non-ASCII characters in 2–4 bytes, so slicing at an arbitrary byte offset can split a multi-byte character and panic at runtime. The `str::get` method returns `Option<&str>` — `None` if the range falls outside a char boundary — while direct indexing panics. Correct Unicode handling requires iterating characters, not bytes.

## Learning Outcomes

- Understand that `"café".len() == 5` (bytes) but `"café".chars().count() == 4` (chars)
- Use `.get(range)` for safe slicing that returns `None` on boundary violations
- Use `char_indices()` to map character positions to byte offsets
- Distinguish ASCII-safe `[byte_range]` slicing from multi-byte safe `.chars()` iteration
- Recognise when byte-level slicing is acceptable (known ASCII or validated boundaries)

## Rust Application

ASCII strings can be sliced directly because every byte is a valid char boundary:

```rust
assert_eq!(&"hello"[0..3], "hel");
```

For untrusted or non-ASCII input, use `str::get`:

```rust
assert_eq!("hello".get(1..4), Some("ell"));
assert_eq!("hello".get(0..99), None);   // out of bounds → None, not panic
```

`"café".len()` returns 5 because `é` is encoded as two bytes (`\xc3\xa9`). `char_indices()` yields `(byte_offset, char)` pairs, allowing correct iteration when byte positions are needed alongside characters.

## OCaml Approach

OCaml's standard `string` is a byte string — `String.length "café"` returns 5, matching Rust's `.len()`. Character-level operations require the `Uutf` or `Camomile` library:

```ocaml
(* Byte-level slicing *)
let sub = String.sub "hello" 1 3  (* "ell" *)

(* Character count via Uutf *)
let char_count s =
  Uutf.String.fold_utf_8 (fun acc _ _ -> acc + 1) 0 s
```

OCaml 5 does not include Unicode-aware string operations in the standard library; correct Unicode handling always requires an external package.

## Key Differences

1. **Panic vs. None**: Rust's `&s[range]` panics on invalid UTF-8 boundaries; `s.get(range)` returns `Option`. OCaml's `String.sub` raises `Invalid_argument` on out-of-bounds.
2. **Byte vs. character length**: Both Rust and OCaml `len`/`length` count bytes; character counting requires `chars().count()` in Rust and a library in OCaml.
3. **`char_indices`**: Rust provides `char_indices()` as a standard iterator; OCaml requires `Uutf.String.fold_utf_8` or manual UTF-8 decoding.
4. **Safety by default**: Rust's type system distinguishes `char` (a Unicode scalar value, 4 bytes) from `u8` (a byte); OCaml's `char` is a single byte, silently wrong for non-ASCII.

## Exercises

1. **Safe nth char**: Implement `nth_char(s: &str, n: usize) -> Option<char>` using `chars().nth(n)` and benchmark it against a byte-indexed approach on ASCII-only input.
2. **Char boundary validator**: Write `is_char_boundary_range(s: &str, start: usize, end: usize) -> bool` without using `str::get` — check `s.is_char_boundary(start) && s.is_char_boundary(end)`.
3. **Grapheme clusters**: Use the `unicode-segmentation` crate to split `"e\u{0301}"` (e + combining accent) correctly and compare the grapheme count to `.chars().count()`.
