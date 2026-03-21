📖 **[View on hightechmind.io →](https://hightechmind.io/rust/482-string-unicode)**

---

# String Unicode
**Difficulty:** ⭐  
**Category:** Functional Programming  



Rust's standard library exposes Unicode properties through `char` methods (`is_alphabetic`, `is_uppercase`, `to_lowercase`) and string methods (`is_ascii`, `eq_ignore_ascii_case`), while full Unicode normalisation requires the `unicode-normalization` crate.

## Problem Statement

Unicode defines multiple ways to represent the same visual character: `é` can be a single precomposed codepoint (U+00E9) or a base letter `e` (U+0065) followed by a combining accent (U+0301). These two sequences look identical but compare unequal as byte strings. Web forms, databases, and search engines must normalise Unicode before comparison. Emoji occupy 4 bytes in UTF-8 (U+1F600 = `\u{1F600}`) — naive `len()` returns 4, not 1. Correct Unicode handling requires understanding: NFC/NFD normalisation, grapheme clusters, and the difference between bytes, codepoints, and user-perceived characters.

## Learning Outcomes

- Understand that NFC and NFD representations of the same character compare unequal
- Use `eq_ignore_ascii_case` for case-insensitive ASCII comparison without allocation
- Check ASCII-only strings with `str::is_ascii()`
- Understand emoji encoding: 4 UTF-8 bytes, 1 char, 1 grapheme cluster
- Recognise when the `unicode-normalization` crate is needed for correct comparison

## Rust Application

Two representations of `é` are not byte-equal:

```rust
assert_ne!("caf\u{00E9}", "caf\u{0065}\u{0301}");  // NFC vs NFD
```

Case-insensitive ASCII comparison without allocation:

```rust
"hello".eq_ignore_ascii_case("HELLO")  // true
```

An emoji is 1 char, 4 bytes:

```rust
"\u{1F600}".len()            // 4 bytes
"\u{1F600}".chars().count()  // 1 char
```

For full Unicode normalisation:

```rust
use unicode_normalization::UnicodeNormalization;
let nfc: String = "caf\u{0065}\u{0301}".nfc().collect();
// nfc == "caf\u{00E9}"
```

## OCaml Approach

OCaml's standard library has no Unicode normalisation. `String.equal` is byte equality. Case-insensitive comparison requires `String.lowercase_ascii` (ASCII-only) or `Uucp.Case.fold` (full Unicode):

```ocaml
String.equal
  (String.lowercase_ascii "Hello")
  (String.lowercase_ascii "HELLO")  (* true *)

(* Unicode normalisation via uunf *)
let nfc s =
  let buf = Buffer.create (String.length s) in
  let norm = Uunf.create `NFC in
  (* feed codepoints from Uutf, flush from Uunf into buf *)
  Buffer.contents buf
```

## Key Differences

1. **Standard Unicode properties**: Rust's `char::is_alphabetic()` uses the Unicode `Alphabetic` property; OCaml's `Char.is_alpha` is ASCII-only (via `is_alpha` from `Char`).
2. **NFC/NFD in stdlib**: Rust delegates normalisation to `unicode-normalization` crate; OCaml delegates to `uunf` — neither includes it in the standard library.
3. **`eq_ignore_ascii_case`**: Rust has this in the standard library; OCaml needs `String.lowercase_ascii` + compare.
4. **Emoji byte count**: Both languages store emoji as 4-byte UTF-8 sequences; both `.chars().count()` / `Uutf` yield 1 codepoint; both require `unicode-segmentation` / `Uuseg` for grapheme cluster counting.

## Exercises

1. **NFC normalise and compare**: Write `unicode_eq(a: &str, b: &str) -> bool` that normalises both strings to NFC (using `unicode-normalization`) before comparing.
2. **Emoji counter**: Write `count_emoji(s: &str) -> usize` that counts characters with Unicode category `So` (Other Symbol) using `char::is_ascii()` inversion and the `unicode-properties` crate.
3. **Case folding**: Use the `caseless` crate to implement `case_fold_eq(a: &str, b: &str) -> bool` that handles the Turkish dotless-i and other Unicode case-folding edge cases.
