# 479: replace(), replacen()

**Difficulty:** 1  **Level:** Beginner

Replace substrings and filter characters — with the same flexible pattern system.

## The Problem This Solves

Python's `str.replace(old, new, count)` and JavaScript's `str.replace()`/`str.replaceAll()` are fundamental. Rust's `.replace()` and `.replacen()` cover the same ground. What makes Rust distinctive here is consistency: the same *pattern* system from `.find()`, `.split()`, and `.contains()` works for `.replace()` too.

You can replace a `char`, a `&str`, or use a closure as the pattern. `s.replace(|c: char| c.is_ascii_digit(), "")` removes all digits — without a regex crate. This composability is Rust's design philosophy: one pattern trait, used everywhere.

There's also `.retain()` — an in-place character filter that modifies the `String` directly. Python's equivalent would be `"".join(c for c in s if keep(c))`, which allocates a new string. `retain()` modifies in place, keeping the same allocation.

## The Intuition

`.replace(pat, replacement)` scans for all matches of `pat` and returns a new `String` with each occurrence replaced. The original is unchanged (immutable borrow). `.replacen(pat, replacement, n)` limits to the first `n` occurrences — like Python's `s.replace(old, new, count)`.

`.retain(|c: char| ...)` is different: it modifies the string **in place**, keeping only characters where the closure returns `true`. No new allocation. Think of it as an in-place filter for characters.

Key mental model: `replace` = new string, never touches the original. `retain` = mutates in place.

## How It Works in Rust

```rust
let s = "Hello, World! Hello, Rust!";

// replace — all occurrences, returns new String
s.replace("Hello", "Hi");    // "Hi, World! Hi, Rust!"
s.replace('!', ".");          // char pattern — replaces all '!'

// replacen — first n occurrences only
s.replacen("Hello", "Hi", 1); // "Hi, World! Hello, Rust!" ← only first

// Original is unchanged
let orig = String::from("hello world");
let modified = orig.replace("world", "rust");
println!("{}", orig);     // "hello world" — untouched
println!("{}", modified); // "hello rust"

// retain — in-place character filter (no new allocation)
let mut s2 = String::from("h3ll0 w0rld");
s2.retain(|c| !c.is_ascii_digit());
// s2 = "hll wrld"

// Pattern as closure — remove all non-alphanumeric
let mut slug = String::from("Hello, World! 2024");
slug.retain(|c| c.is_alphanumeric() || c == '-' || c == ' ');
// "Hello World 2024"

// Chain replacements (each returns a new String)
"foo bar foo"
    .replace("foo", "qux")
    .replace("bar", "quux")
// "qux quux qux"
```

## What This Unlocks

- **Text sanitization** — remove or replace forbidden characters with `retain()` or `replace()`.
- **Slug generation** — chain `replace` calls to normalize text for URLs or filenames.
- **Template expansion** — `replace("{{name}}", actual_name)` for simple template rendering.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Replace all | Custom `replace_all` (Buffer loop) | `s.replace(pat, rep)` |
| Replace first N | Custom loop | `s.replacen(pat, rep, n)` |
| Replace char | `String.map` | `s.replace('c', "rep")` |
| In-place filter | `Bytes.map` (not truly in-place) | `s.retain(\|c\| ...)` |
| Pattern types | Char or manual | `char`, `&str`, `&[char]`, closure |
| Returns | New string | New `String` (original unchanged) |
