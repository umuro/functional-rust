# 094: Run-Length Encoding

**Difficulty:** 1  **Level:** Beginner

Compress a string by replacing runs of identical characters with a count and character.

## The Problem This Solves

"AABCCCDEEEE" is 11 characters. "2AB3CD4E" is 8. Run-length encoding (RLE) exploits repetition: instead of storing each character individually, store how many times it appears consecutively. For inputs with long runs (fax images, simple graphics, repeated log entries), RLE achieves significant compression.

Beyond compression, the grouping pattern — "track current element and count, flush when element changes" — appears constantly: counting word frequencies, summarising log streams, detecting transitions in sensor data.

## The Intuition

Walk the string character by character. Keep a counter for the current character. When the next character matches the current one, increment the counter. When it doesn't match (or you reach the end), emit the count (if > 1) and the character, then reset.

The key insight: you must flush the last group after the loop. It's easy to forget — a common off-by-one in grouping algorithms.

The functional version uses `fold` to build a list of `(char, count)` pairs, then maps each pair to its encoded string. The "current group" becomes `groups.last_mut()` — mutate the last entry if the character matches, push a new entry if it doesn't.

## How It Works in Rust

```rust
// Imperative — explicit loop with tracking
pub fn encode(s: &str) -> String {
    if s.is_empty() { return String::new(); }
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    let mut count = 1;

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
        } else {
            if count > 1 { result.push_str(&count.to_string()); }
            result.push(chars[i - 1]);
            count = 1;
        }
    }
    if count > 1 { result.push_str(&count.to_string()); }
    result.push(*chars.last().unwrap());  // flush last group
    result
}

// Fold-based — groups built functionally
pub fn encode_fold(s: &str) -> String {
    s.chars()
        .fold(Vec::<(char, usize)>::new(), |mut acc, c| {
            match acc.last_mut() {
                Some((last, count)) if *last == c => *count += 1,  // extend group
                _ => acc.push((c, 1)),                             // new group
            }
            acc
        })
        .iter()
        .map(|&(c, n)| if n > 1 { format!("{}{}", n, c) } else { c.to_string() })
        .collect()
}
```

The fold approach eliminates the "flush after loop" problem: every group is pushed explicitly when the character changes, and the last group is naturally the last element of `acc`. No special-case needed.

## What This Unlocks

- **The "flush on change" pattern** — the core of any consecutive-grouping algorithm: extend the current group, flush when the element changes.
- **Imperative vs functional grouping** — the imperative version is explicit about state; the fold version makes the accumulation structure visible.
- **String building in Rust** — `String::push_str`, `format!()`, and `collect::<String>()` are the three ways to build strings incrementally.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String building | `Buffer.create` + `Buffer.add_string` | `String::push_str` or `collect::<String>()` |
| Current-group tracking | Mutable `current` + `count` in recursive helper | `last_mut()` on accumulator Vec |
| Functional grouping | Recursive with accumulator | `fold` building `Vec<(char, usize)>` |
| Flush last group | Handled at base case of recursion | `chars.last().unwrap()` after loop (imperative) or natural in fold |
| Type annotation | Inferred | `Vec::<(char, usize)>::new()` often needed for fold |
