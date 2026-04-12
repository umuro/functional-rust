📖 **[View on hightechmind.io →](https://hightechmind.io/rust/498-string-truncation)**

---

# String Truncation
**Difficulty:** ⭐  
**Category:** Functional Programming  



Safe Unicode-aware truncation requires finding char boundaries rather than slicing at arbitrary byte offsets. Rust provides `is_char_boundary` and `char_indices().nth(n)` for correct byte-limit and character-limit truncation.

## Problem Statement

Naively truncating `&s[..N]` panics if `N` falls in the middle of a multi-byte character (e.g., slicing `"café"` at byte 4 lands inside `é`). Database column limits, UI text truncation, and API field limits are measured in bytes or characters — not always the same. A correct truncation implementation must: (1) find the nearest valid char boundary for byte-limited truncation, and (2) find the byte position of the Nth character for character-limited truncation.

## Learning Outcomes

- Truncate to a byte limit safely by walking back to the nearest `is_char_boundary`
- Truncate to a character limit using `char_indices().nth(max_chars)` for the byte position
- Add an ellipsis `…` (U+2026, 3 bytes in UTF-8) when truncating display strings
- Handle the edge case where the string is shorter than the limit
- Apply `saturating_sub` to avoid underflow when reserving space for the ellipsis

## Rust Application

`truncate_bytes` walks backward from the byte limit until a char boundary is found:

```rust
fn truncate_bytes(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes { return s; }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) { end -= 1; }
    &s[..end]
}
```

`truncate_chars` uses `char_indices().nth(max_chars)` to get the byte position of the Nth char:

```rust
fn truncate_chars(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        Some((byte_pos, _)) => &s[..byte_pos],
        None => s,
    }
}
```

`truncate_with_ellipsis` reserves one character for `…` and appends it when truncation occurs.

## OCaml Approach

```ocaml
let truncate_bytes s max_bytes =
  if String.length s <= max_bytes then s
  else
    (* Walk back to UTF-8 boundary using Uutf *)
    let i = ref max_bytes in
    while !i > 0 && not (Uutf.String.is_char_boundary s !i) do decr i done;
    String.sub s 0 !i

let truncate_chars s max_chars =
  let i = ref 0 and count = ref 0 in
  Uutf.String.fold_utf_8 (fun () p _ ->
    if !count < max_chars then (i := p; incr count)) () s;
  if !count <= max_chars then s
  else String.sub s 0 !i
```

OCaml has no standard `is_char_boundary` — it requires `Uutf` for correct Unicode truncation. `String.sub` allocates a new string.

## Key Differences

1. **`is_char_boundary`**: Rust provides this as a standard `str` method; OCaml needs `Uutf` to check UTF-8 boundaries.
2. **Zero-copy truncation**: Both `truncate_bytes` and `truncate_chars` return `&str` pointing into the original — no allocation; OCaml's `String.sub` always allocates.
3. **`char_indices().nth(n)`**: Rust's O(N) character indexing via `char_indices` is explicit about its cost; OCaml's fold is equally O(N) but less idiomatically readable.
4. **Ellipsis as `char`**: `'…'` is 3 UTF-8 bytes; using `saturating_sub(1)` reserves one character position, not one byte — correct in Rust's char-counting `truncate_chars`.

## Exercises

1. **`floor_char_boundary`**: Rust 1.72 added `str::floor_char_boundary(n)`; rewrite `truncate_bytes` to use it and add a `cfg!` fallback for older Rust versions.
2. **Truncate by display width**: Use the `unicode-width` crate to truncate based on terminal column width (CJK characters are 2 columns wide).
3. **Sentence truncation**: Write `truncate_sentence(s: &str, max_chars: usize) -> String` that truncates at the last sentence boundary (`.`/`!`/`?`) before `max_chars` rather than mid-word.
