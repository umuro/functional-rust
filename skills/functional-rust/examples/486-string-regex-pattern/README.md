# 486: Regex-Like Matching Without Crates

**Difficulty:** 2  **Level:** Intermediate

Rust's `str` methods cover most real-world pattern matching — no regex engine needed.

## The Problem This Solves

Reaching for a regex engine is a habit from languages where string methods are thin wrappers around character arrays. Regex adds a compile step, a runtime dependency, and cognitive overhead for patterns that `str` handles directly. Many codebases import `regex` just for things like "does this string start with `http`" or "split on whitespace."

Rust's `str` API is deliberately rich. `starts_with`, `ends_with`, `contains`, `find`, `split`, `trim`, `matches`, and pattern-based methods accept not just string literals but also `char`, `&[char]`, and closures — giving you predicate-based matching with zero overhead.

Understanding what `str` can do well prevents over-engineering and teaches you the building blocks for writing your own matchers when you do need custom logic.

## The Intuition

A Swiss army knife versus a chainsaw. For most string operations — "does this line start with `#`?", "strip leading whitespace", "find the first digit" — `str` is the Swiss army knife: always available, no setup. Reach for `regex` only when you genuinely need backreferences, quantifiers, or alternation across complex patterns.

## How It Works in Rust

1. **Prefix/suffix checks**:
   ```rust
   url.starts_with("https://")
   filename.ends_with(".rs")
   ```
2. **Substring search**:
   ```rust
   line.contains("ERROR")
   line.find("->").map(|i| &line[i+2..])
   ```
3. **Character class matching** — use a closure or `char::is_*`:
   ```rust
   s.chars().all(|c| c.is_ascii_alphanumeric())
   s.trim_matches(|c: char| !c.is_alphabetic())
   ```
4. **Split on pattern**:
   ```rust
   "a,b,,c".split(',').filter(|s| !s.is_empty())
   csv_line.splitn(3, ',')  // max 3 parts
   ```
5. **Multiple delimiters** — pass a slice of chars:
   ```rust
   text.split(&[',', ';', '\t'][..])
   ```
6. **Rolling your own matcher** — for simple wildcards, write a recursive function; for finite automata, a `match` on states.

## What This Unlocks

- **Zero dependencies** for common patterns — no `Cargo.toml` edit, no compile time, no unsafe.
- **Composability** — `str` methods return `&str` slices or iterators that chain naturally.
- **Foundation** — when you do need `regex`, you already know the simpler path and can recognise when the crate is actually necessary.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Pattern type | String or `Re` | `&str`, `char`, `&[char]`, closure |
| Regex | `Re` / `Str` stdlib modules | External `regex` crate |
| Predicate match | `String.exists` | `str::contains(|c: char| ...)` |
| Split | `String.split_on_char` | `str::split`, `splitn`, `split_once` |
