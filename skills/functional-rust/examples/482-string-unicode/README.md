# 482: Unicode Normalization and Graphemes

**Difficulty:** 2  **Level:** Intermediate

Why `"café" != "café"` can be true — and how to handle Unicode correctly in Rust.

## The Problem This Solves

Unicode is harder than it looks. The character `é` can be encoded two ways: as a single codepoint `U+00E9` (NFC — "composed"), or as the letter `e` followed by a combining accent `U+0301` (NFD — "decomposed"). Both look identical on screen. Both are valid UTF-8. But they are different byte sequences — and Rust's `==` operator compares bytes, so `"café" == "café"` can return `false` if one was composed and the other decomposed.

This matters in practice: text copied from macOS tends to be NFD. Text from Windows tends to be NFC. If you compare usernames, file paths, or search terms without normalizing, you get false negatives. The `unicode-normalization` crate handles this, but the std library doesn't — a deliberate decision to keep the core small.

Even within std, there are surprises: the flag emoji `🇳🇱` is two Unicode scalar values (regional indicator N + regional indicator L) but one user-visible character. `.chars().count()` returns 2 for the flag, not 1. For true grapheme cluster counting, you need the `unicode-segmentation` crate.

## The Intuition

Think of Unicode as having three levels:
1. **Bytes** — raw UTF-8 storage (what `s.len()` measures)
2. **Scalar values** — what `s.chars()` iterates (Rust's `char` type, U+0000 to U+10FFFF)
3. **Grapheme clusters** — what users *see* as characters (requires external crate)

NFC vs NFD is about level 2: the same user-visible character, different scalar value sequences. Rust's `==` operates at level 1 (bytes), so it distinguishes NFC from NFD. For case-insensitive ASCII comparison, std has `.eq_ignore_ascii_case()`. For full Unicode case folding, you need the `unicode-casefold` crate.

The practical rule: for most applications, receive text, assume it's NFC or normalize it on input, and compare bytes. Only reach for normalization crates when you're building search engines, identifier systems, or any user-facing comparison.

## How It Works in Rust

```rust
// Two ways to write "café" — look the same, different bytes
let nfc = "caf\u{00E9}";          // U+00E9: single codepoint é
let nfd = "caf\u{0065}\u{0301}"; // U+0065 + U+0301: e + combining accent

println!("NFC: {} bytes, {} chars", nfc.len(), nfc.chars().count()); // 5, 4
println!("NFD: {} bytes, {} chars", nfd.len(), nfd.chars().count()); // 6, 5

// Byte comparison — these are NOT equal!
println!("{}", nfc == nfd);  // false

// Case-insensitive comparison (ASCII only, in std)
"Hello".eq_ignore_ascii_case("HELLO")  // true
"café".eq_ignore_ascii_case("CAFÉ")    // false — only works for ASCII letters

// Check if all ASCII (no multi-byte chars)
"hello".is_ascii()  // true
"café".is_ascii()   // false

// Emoji: chars().count() counts scalar values, not graphemes
let flag = "\u{1F1F3}\u{1F1F1}";  // 🇳🇱 — two scalar values, one grapheme
println!("{} bytes, {} chars", flag.len(), flag.chars().count()); // 8, 2

// Unicode category predicates on char
for c in "Hello 42 !".chars() {
    if c.is_alphabetic() { /* letter */ }
    if c.is_numeric()    { /* digit */ }
    if c.is_whitespace() { /* space */ }
    if c.is_uppercase()  { /* A-Z + Unicode uppercase */ }
}

// For normalization: use unicode-normalization crate
// use unicode_normalization::UnicodeNormalization;
// let normalized = nfd.nfc().collect::<String>();
```

## What This Unlocks

- **Robust text comparison** — understand why two visually identical strings can be `!=` and how to prevent it.
- **Correct character counting** — know the difference between bytes, scalar values, and grapheme clusters.
- **Internationalization** — handle text from multiple platforms and locales without silent bugs.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String equality | Byte comparison | Byte comparison (NFC ≠ NFD) |
| Character count | Manual UTF-8 decode loop | `.chars().count()` (scalar values) |
| Grapheme count | Uutf crate | `unicode-segmentation` crate |
| Case-insensitive eq | Manual | `.eq_ignore_ascii_case()` (ASCII only) |
| Normalization | Uunf crate | `unicode-normalization` crate |
| Is ASCII? | Manual check | `s.is_ascii()` |
| Char predicates | `Char.code c < 128` etc. | `c.is_alphabetic()`, `.is_numeric()`, etc. |
