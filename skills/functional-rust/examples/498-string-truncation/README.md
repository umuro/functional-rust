# 498: Safe Unicode Truncation

**Difficulty:** 1  **Level:** Intermediate

Truncate a UTF-8 string to a maximum length without splitting a multi-byte character in the middle.

## The Problem This Solves

Rust strings are UTF-8. A character like `é` takes 2 bytes, a CJK ideograph takes 3, and an emoji takes 4. If you write `&s[..10]`, Rust will panic at runtime if byte 10 falls in the middle of a multi-byte sequence — there is no partial character.

This matters everywhere you display strings in constrained spaces: UI labels, log prefixes, database column limits, API responses. You need a `truncate` that is both safe (never panics) and correct (never produces garbled output).

There are actually two different meanings of "length" in play: byte length (what the database cares about) and character count (what the user sees). A good truncation library exposes both, plus an ellipsis variant for display use.

## The Intuition

Walk the string using `char_indices()`, which yields `(byte_position, char)` pairs. Stop at the *n*th character — the byte position at that point is exactly the safe cut. For byte-limited truncation, walk backwards from the desired byte position until you land on a character boundary (`is_char_boundary`).

## How It Works in Rust

**Byte truncation** — walk back to a boundary:
```rust
fn truncate_bytes(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes { return s; }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) { end -= 1; }
    &s[..end]
}
```
`is_char_boundary` returns `true` at positions where a character starts (or at `s.len()`). Walking backwards always terminates because byte 0 is always a boundary.

**Character truncation** — use `char_indices().nth()`:
```rust
fn truncate_chars(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        Some((byte_pos, _)) => &s[..byte_pos],
        None => s,
    }
}
```
`nth(n)` returns the `(byte_offset, char)` of the *n*th character, or `None` if the string is shorter. The byte offset is the exact slice boundary we need.

**Ellipsis variant**:
```rust
fn truncate_with_ellipsis(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars { return s.to_string(); }
    let truncated = truncate_chars(s, max_chars.saturating_sub(1));
    format!("{}…", truncated)
}
```
Reserve one character position for `…` (U+2026, a single Unicode character).

Since Rust 1.72, `str::floor_char_boundary(n)` does the walk-back for you in the standard library.

## What This Unlocks

- **Safe display formatting** — truncate labels, tooltips, and log entries without panics.
- **Database length limits** — trim UTF-8 to a byte budget before INSERT without corrupting multibyte sequences.
- **Emoji and CJK correctness** — a Japanese or emoji-heavy string behaves correctly with char-based truncation even when byte counts look wrong.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String encoding | Bytes (raw), UTF-8 by convention | UTF-8 guaranteed by type |
| Safe substring | `String.sub` (byte positions, UB if mid-char) | `&s[..]` panics on bad boundary; use `is_char_boundary` |
| Char iteration with positions | `String.foldi` | `s.char_indices()` yields `(usize, char)` |
| Char count vs byte count | `String.length` = bytes | `.chars().count()` = chars, `.len()` = bytes |
