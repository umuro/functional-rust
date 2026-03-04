# 476: split(), splitn(), split_once()

**Difficulty:** 1  **Level:** Beginner

Split strings lazily, with limits, or exactly once — zero-copy slices of the original.

## The Problem This Solves

Every language has string splitting. Python's `str.split()` returns a list. JavaScript's `.split()` returns an array. Both create new string objects for every piece. Rust's `.split()` returns a lazy **iterator of `&str` slices** — each piece is just a pointer + length into the original string. No allocation until you actually need it.

This matters when you're processing large inputs. Splitting a 1 MB CSV line in Python allocates all fields up front. In Rust, you iterate lazily and only allocate what you use. And since each piece is a `&str` slice of the original, you can pass them directly to functions without copying.

Rust also has two specialized variants that fill common gaps: `split_once()` for key=value parsing (no need to split the whole string when you only want the first delimiter), and `splitn()` for limiting the number of pieces (so the last piece contains the remainder as-is).

## The Intuition

`.split(pattern)` gives you an iterator. Think of it as Python's `str.split()` but lazy — you pull pieces on demand. Call `.collect::<Vec<_>>()` when you need all pieces at once.

`split_once('=')` on `"host=localhost:8080"` gives you `Some(("host", "localhost:8080"))`. It splits on the *first* occurrence only. Python equivalent: `s.split('=', 1)` returning a 2-element list. Perfect for parsing config lines, HTTP headers, query params.

`splitn(3, '/')` gives at most 3 pieces — the last piece is the remaining string unsplit. OCaml's `String.split_on_char` has no limit; you'd implement this manually.

## How It Works in Rust

```rust
// split — lazy iterator of &str (zero-copy slices)
let csv = "alice,30,amsterdam,developer";
let parts: Vec<_> = csv.split(',').collect();
// ["alice", "30", "amsterdam", "developer"]

// splitn — at most n pieces (last piece = remainder)
let path = "a/b/c/d/e";
let parts: Vec<_> = path.splitn(3, '/').collect();
// ["a", "b", "c/d/e"]  ← last piece is unsplit remainder

// split_once — exactly one split, returns Option<(&str, &str)>
let kv = "host=localhost:8080";
if let Some((key, value)) = kv.split_once('=') {
    println!("key='{}' val='{}'", key, value);
    // key='host' val='localhost:8080'  ← colon preserved in value
}

// rsplit_once — split from the RIGHT (last occurrence)
let file = "/home/user/file.txt";
if let Some((dir, name)) = file.rsplit_once('/') {
    println!("dir='{}' name='{}'", dir, name);
    // dir='/home/user'  name='file.txt'
}

// split_whitespace — handles all Unicode whitespace, trims leading/trailing
let words: Vec<_> = "  hello   world\t!\n  ".split_whitespace().collect();
// ["hello", "world", "!"]  — no empty strings

// lines() — split on \n or \r\n
for (i, line) in "line1\nline2\r\nline3".lines().enumerate() {
    println!("{}: {}", i + 1, line);
}
```

## What This Unlocks

- **CSV/TSV parsing** — `.split(',')` yields lazy field slices, collect only what you need.
- **HTTP header parsing** — `header.split_once(':')` cleanly separates name from value.
- **URL/path parsing** — `splitn` and `rsplit_once` give you clean directory/filename splits.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Split on char | `String.split_on_char ',' s` → `string list` | `s.split(',')` → lazy iterator |
| Split with limit | Manual | `s.splitn(n, sep)` |
| Split once | Manual with pattern match | `s.split_once(sep)` → `Option<(&str, &str)>` |
| Split from right | Manual | `s.rsplit_once(sep)` |
| Split on whitespace | Manual filter | `s.split_whitespace()` |
| Split on newlines | `String.split_on_char '\n'` | `s.lines()` (handles `\r\n` too) |
| Allocation | New `string` per piece | `&str` slices — zero-copy |
