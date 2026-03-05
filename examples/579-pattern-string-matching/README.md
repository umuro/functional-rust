📖 **[View on hightechmind.io →](https://hightechmind.io/rust/579-pattern-string-matching)**

---

# 579: String/str Pattern Matching

**Difficulty:** 2  **Level:** Beginner

Match on string slices with literal arms, OR patterns, and guards.

## The Problem This Solves

Matching on strings is one of the first things newcomers try in Rust — and one of the first things that trips them up. You can't `match` on `String` directly with string literal arms because `String` and `&str` are different types. The solution is simple but not obvious: call `.as_str()` on a `String`, or accept a `&str` parameter in the first place.

Once you have a `&str`, pattern matching works exactly as you'd expect: literal arms, OR patterns (`"quit" | "exit" | "q"`), and guards (`s if s.starts_with('/')`) compose cleanly. This enables concise command parsers, protocol decoders, and HTTP method routers without a chain of `if/else if`.

## The Intuition

A `match` on `&str` compares string slices by value. Each arm is a string literal (`&'static str`), which coerces to `&str` for comparison. OR patterns collapse multiple synonyms into one arm. Guards let you branch on string properties (prefix, suffix, length) that can't be expressed as literal patterns.

To match on an owned `String`, coerce it: `match my_string.as_str() { ... }` or `match my_string.as_ref() { ... }` or accept `&str` from the start.

## How It Works in Rust

1. **Literal arms** — `match s { "quit" => ..., "help" => ..., _ => ... }` — works directly on `&str`.
2. **OR patterns** — `"quit" | "exit" | "q" => "quit"` — collapse synonyms; compiler checks exhaustiveness across all arms.
3. **Guards** — `s if s.starts_with('/') => "command"` — runs when the pattern binds `s` and the condition is true.
4. **Binding in arms** — `n if n.starts_with("Dr.") => format!("Good day, {}!", n)` — the arm binds the matched `&str` as `n`.
5. **`&String` coercion** — `match owned.as_str() { ... }` or `day_type(&owned)` — deref coercion handles `&String -> &str` at call sites.

## What This Unlocks

- Write command parsers and protocol handlers as clean `match` expressions instead of `if/else` chains.
- Collapse synonym groups (HTTP methods, CLI aliases, day names) into single arms with OR patterns.
- Mix literal matching with guard-based heuristics in one expression.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String match | `match s with "quit" -> ... \| _ -> ...` — works on `string` | Works on `&str`; `String` needs `.as_str()` first |
| OR patterns | `"quit" \| "exit" -> ...` | `"quit" \| "exit" => ...` — identical syntax |
| Guards | `\| s when String.length s > 0 -> ...` | `s if s.len() > 0 => ...` |
| String vs slice | Uniform `string` type | `String` (owned heap) vs `&str` (borrowed slice); match on `&str` |
