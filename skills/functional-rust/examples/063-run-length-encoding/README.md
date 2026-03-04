# 063: Run-Length Encoding

**Difficulty:** ⭐  **Level:** Foundations

Compress consecutive repeated characters into count+char pairs — and decompress them back.

## The Problem This Solves

You have a string with long runs of repeated characters: `"AABCCCDEEEE"`. Instead of storing all those bytes, you want to say "2 A's, 1 B, 3 C's, 1 D, 4 E's" → `"2AB3CD4E"`. This is run-length encoding (RLE), one of the simplest lossless compression schemes used in fax machines, BMP images, and PCX graphics.

Implementing it from scratch teaches you how to build output strings character-by-character in Rust. In Python you'd use a generator and `itertools.groupby`. In JavaScript you'd accumulate into an array and join. In Rust, you push onto a `String` — and you need to think explicitly about when to flush the current run.

The tricky part is the "last run" problem: your loop processes each character when the *next* one differs, so the final run never triggers a flush. You have to handle it after the loop ends.

## The Intuition

Walk through the string tracking two things: the current character and how many times you've seen it in a row. When the character changes, write what you accumulated. The only gotcha: after the loop finishes, you still have the last run sitting in your counter — write it too.

OCaml uses `Buffer.t` (a mutable byte buffer) to accumulate output. Rust uses `String` directly, with `push_str()` for string slices and `push()` for single chars. The logic is identical; the API names differ.

## How It Works in Rust

```rust
pub fn encode(s: &str) -> String {
    if s.is_empty() { return String::new(); }

    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    let mut count = 1;

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
        } else {
            if count > 1 {
                result.push_str(&count.to_string()); // write count only if > 1
            }
            result.push(chars[i - 1]);               // write the char
            count = 1;
        }
    }
    // Handle the final run — easy to forget!
    if count > 1 { result.push_str(&count.to_string()); }
    result.push(*chars.last().unwrap());
    result
}
```

Decoding reads digits until a non-digit, then repeats that char:

```rust
pub fn decode(s: &str) -> String {
    let mut result = String::new();
    let mut count = 0;
    for c in s.chars() {
        if c.is_ascii_digit() {
            count = count * 10 + (c as usize - '0' as usize);
        } else {
            let repeat = if count == 0 { 1 } else { count };
            for _ in 0..repeat { result.push(c); }
            count = 0;
        }
    }
    result
}
```

## What This Unlocks

- **Simple compression** — encode/decode any repetitive byte sequence (binary data, pixel rows)
- **Streaming transforms** — the same "track current + flush on change" pattern applies to grouping, deduplication, and sliding-window algorithms
- **String building patterns** — `push` / `push_str` / `to_string()` are the primitives for constructing strings in performance-sensitive Rust code

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable string buffer | `Buffer.t` + `Buffer.add_char` | `String` + `.push()` / `.push_str()` |
| Int to string | `string_of_int n` | `n.to_string()` |
| Array indexing | `s.[i]` (byte) | `chars[i]` after `.chars().collect()` |
| Last element | `s.[n-1]` | `*chars.last().unwrap()` |
