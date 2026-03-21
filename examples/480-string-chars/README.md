📖 **[View on hightechmind.io →](https://hightechmind.io/rust/480-string-chars)**

---

# String Chars
**Difficulty:** ⭐  
**Category:** Functional Programming  



Rust's `.chars()` iterator provides character-level traversal of UTF-8 strings, yielding Unicode scalar values (`char`, 4 bytes each) rather than raw bytes — the correct unit for most text processing.

## Problem Statement

Strings in memory are byte sequences, but humans think in characters. For ASCII, bytes and characters coincide; for Unicode text, they diverge. Iterating bytes and assuming each is a character corrupts emoji, accented letters, CJK characters, and anything outside ASCII. Rust's `char` type is a Unicode scalar value (U+0000 to U+10FFFF, excluding surrogates), and `.chars()` decodes UTF-8 on the fly, yielding the correct unit for character counting, filtering, reversal, and indexing.

## Learning Outcomes

- Iterate characters with `.chars()` vs. iterating bytes with `.bytes()`
- Count characters correctly for non-ASCII text with `.chars().count()`
- Filter characters by predicate and collect back to `String`
- Reverse a string character-by-character with `.chars().rev().collect()`
- Access the Nth character with `.chars().nth(n)` (O(N), not O(1))

## Rust Application

`.chars()` decodes UTF-8 and yields `char` values:

```rust
"café".chars().count()  // 4 (characters)
"café".len()            // 5 (bytes — 'é' is 2 bytes)
```

Filtering digits from a string:

```rust
let digits: String = "Hello123".chars().filter(|c| c.is_ascii_digit()).collect();
// "123"
```

Reversing requires `.chars().rev()` because reversing bytes would corrupt multi-byte sequences:

```rust
let rev: String = "abcde".chars().rev().collect();  // "edcba"
```

`.chars().nth(n)` is O(N) — it scans from the start each time; if random character access is needed frequently, consider converting to `Vec<char>` first.

## OCaml Approach

OCaml 4.07+ provides `String.to_seq` which yields `char` values (single bytes — not Unicode scalars):

```ocaml
String.to_seq "hello" |> Seq.filter (fun c -> c >= '0' && c <= '9')
                       |> String.of_seq  (* standard lib 4.07+ *)
```

For true Unicode character iteration, the `Uutf` library is required:

```ocaml
Uutf.String.fold_utf_8 (fun acc _ d ->
  match d with `Uchar u -> u :: acc | _ -> acc) [] "café"
```

OCaml's `char` is a single byte; `Uchar.t` (from `uchar` package) is the Unicode scalar equivalent.

## Key Differences

1. **`char` semantics**: Rust's `char` is a 4-byte Unicode scalar value; OCaml's `char` is a 1-byte value (0–255). True Unicode characters in OCaml require `Uchar.t`.
2. **Standard Unicode support**: Rust handles multibyte UTF-8 correctly via `.chars()` without any external crate; OCaml requires `Uutf` or similar.
3. **Collect from chars**: Rust's `FromIterator<char> for String` enables `.chars().filter(...).collect::<String>()`; OCaml requires `String.of_seq` (4.07+) which works on bytes, not Unicode scalars.
4. **Reversal safety**: `chars().rev().collect()` correctly reverses character by character; reversing bytes with OCaml's `Bytes` can corrupt multi-byte sequences.

## Exercises

1. **Palindrome check**: Write `is_palindrome(s: &str) -> bool` that compares the string to its character-reversed form, handling Unicode correctly.
2. **Char frequency map**: Build a `HashMap<char, usize>` counting character occurrences in a `&str` using `.chars()` and `.entry().and_modify().or_insert()`.
3. **Grapheme-aware reverse**: Use the `unicode-segmentation` crate's `graphemes` iterator to correctly reverse `"e\u{0301}nde"` (e + combining accent + nde) and compare the result to `.chars().rev().collect()`.
