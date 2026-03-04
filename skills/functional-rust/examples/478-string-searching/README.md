# 478: contains(), find(), starts_with()

**Difficulty:** 1  **Level:** Beginner

Search strings for substrings, chars, or patterns — with flexible pattern matching.

## The Problem This Solves

Python has `in`, `.find()`, `.index()`, `.startswith()`, `.endswith()`. JavaScript has `.includes()`, `.indexOf()`, `.startsWith()`, `.endsWith()`. These work on literal strings only. If you want to search by character category — "find the first digit" — you need a regex or manual loop.

Rust's string search methods accept *patterns* — not just string literals, but also single chars, slices of chars, and **closures**. `s.find(|c: char| c.is_ascii_digit())` finds the first digit without importing any regex crate. The same API works for `contains`, `starts_with`, `ends_with`, `find`, `rfind`, `split`, `trim`, and more.

And unlike Python's `.find()` which returns `-1` on failure (a classic footgun), Rust's `.find()` returns `Option<usize>` — `None` when not found. You're forced to handle the "not found" case. The byte position you get back is also valid for slicing, since `.find()` always returns a char boundary.

## The Intuition

The pattern system is Rust's key insight here. Anything that can match a char or substring is a pattern:
- `"World"` — substring match
- `'!'` — single char match  
- `|c: char| c.is_uppercase()` — closure match
- `['a', 'e', 'i', 'o', 'u'].as_ref()` — any-of match

This is like Python's `re.search()` for simple cases, but without the overhead of compiling a regex.

`find()` returns a byte position (`usize`). Use it for slicing. `rfind()` searches from the right — useful for finding file extensions, last path component, etc.

## How It Works in Rust

```rust
let s = "Hello, World! Hello, Rust!";

// Boolean checks — O(n) scan
s.contains("World")       // true
s.starts_with("Hello")    // true
s.ends_with("Rust!")      // true

// find — returns byte position of FIRST match
s.find("World")   // Some(7)
s.find(',')       // Some(5)
s.find('z')       // None

// rfind — returns byte position of LAST match
s.rfind("Hello")  // Some(14)

// Pattern as closure — "first digit"
let t = "abc123";
t.find(|c: char| c.is_ascii_digit())  // Some(3)

// match_indices — iterator over all (position, match) pairs
for (pos, sub) in s.match_indices("Hello") {
    println!("'{}' at byte {}", sub, pos);
}
// 'Hello' at byte 0
// 'Hello' at byte 14

// matches — count occurrences
s.matches("Hello").count()  // 2

// Pattern with char slice — find first vowel
"Hello".find(['a', 'e', 'i', 'o', 'u'].as_ref())  // Some(1) → 'e'
```

## What This Unlocks

- **Input validation** — check for forbidden characters using closure patterns without a regex crate.
- **Parsing structured text** — `rfind('.')` to locate file extension, `find(':')` to split host:port.
- **Text analysis** — count occurrences with `.matches(pat).count()`, locate all positions with `.match_indices()`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Substring check | Custom `contains` function | `s.contains(pat)` |
| Find position | `String.index_opt s c` (char only) | `s.find(pat)` → `Option<usize>` |
| Find from right | `String.rindex_opt s c` | `s.rfind(pat)` |
| Starts with | `String.sub s 0 n = prefix` | `s.starts_with(pat)` |
| Ends with | Manual slice check | `s.ends_with(pat)` |
| Pattern types | Char only | `char`, `&str`, `&[char]`, closure |
| Not found value | `None` (via `_opt`) | `None` — `Option<usize>` |
| All occurrences | Manual loop | `.match_indices(pat)` iterator |
