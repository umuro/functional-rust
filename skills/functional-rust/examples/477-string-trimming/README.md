# 477: trim(), trim_start(), trim_end()

**Difficulty:** 1  **Level:** Beginner

Remove whitespace and custom characters from string edges — without allocating.

## The Problem This Solves

Every language trims strings. Python has `.strip()`, `.lstrip()`, `.rstrip()`. JavaScript has `.trim()`, `.trimStart()`, `.trimEnd()`. These are basic operations. What makes Rust's version interesting is what it returns: not a new `String`, but a `&str` **slice of the original**.

That means no heap allocation. When you call `s.trim()`, you get back a reference to a substring of `s` — the pointer moves forward past leading whitespace, the length shrinks to exclude trailing whitespace. Zero bytes copied. This matters when you're trimming thousands of lines from a log file.

Rust also generalizes trimming beyond whitespace. `trim_matches(pattern)` can trim any char, any set of chars, or even a closure. `trim_start_matches('/')` strips leading slashes from a path. These flexible patterns make many one-off string cleaning tasks trivial.

## The Intuition

`.trim()` is Python's `.strip()`. `.trim_start()` is `.lstrip()`. `.trim_end()` is `.rstrip()`. The naming is slightly different but the semantics are the same — remove Unicode whitespace from one or both ends.

The key difference: Rust returns a `&str` that's a slice of the original. You haven't allocated a new string — you've just adjusted the view. That's why `trim` + `push_str` is common: trim returns the slice, then you decide whether to copy it or just borrow it.

`trim_matches` is more powerful than Python's equivalent: it trims *any character matching the pattern* from both ends, not just a specific character. Pass a `char`, a `&[char]`, or a closure `|c: char| c.is_ascii_digit()`.

## How It Works in Rust

```rust
let s = "  hello world  ";

// Trim both ends (most common)
s.trim();        // "hello world"   ← &str slice, no allocation
s.trim_start();  // "hello world  " ← remove leading only
s.trim_end();    // "  hello world" ← remove trailing only

// trim_matches — custom pattern (both ends)
"***hello***".trim_matches('*');         // "hello"
"123hello456".trim_matches(|c: char| c.is_ascii_digit()); // "hello"
"--foo--".trim_matches(['-', ' '].as_ref()); // "foo"

// trim_start_matches / trim_end_matches (one side only)
"///path/to/file".trim_start_matches('/');        // "path/to/file"
"https://example.com/////".trim_end_matches('/'); // "https://example.com"

// Common pattern: trim lines from multi-line text
let text = "\n  line1  \n  line2  \n";
let cleaned: Vec<&str> = text
    .lines()
    .map(str::trim)          // trim each line
    .filter(|l| !l.is_empty()) // skip empty lines
    .collect();
// ["line1", "line2"]

// Check if string is blank after trimming
"   ".trim().is_empty()  // true
```

## What This Unlocks

- **Input sanitization** — trim user input before validation or storage, zero cost.
- **Config file parsing** — trim each line, skip blank lines, without extra allocations.
- **Path normalization** — `trim_start_matches('/')` to strip leading slashes from relative paths.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Trim both ends | `String.trim s` (whitespace only) | `s.trim()` |
| Trim left | No built-in (custom loop) | `s.trim_start()` |
| Trim right | No built-in (custom loop) | `s.trim_end()` |
| Trim custom char | Custom loop | `s.trim_matches('*')` |
| Trim with closure | Custom loop | `s.trim_matches(\|c: char\| ...)` |
| Returns | New `string` (allocated) | `&str` slice of original (zero-copy) |
