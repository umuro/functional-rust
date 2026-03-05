# JSON Pretty Print — Comparison

## Core Insight
Pretty-printing is a classic recursive problem. Both languages follow the same algorithm: recurse into nested structures, track indentation depth, and concatenate output. OCaml tends toward `Buffer` for efficiency; Rust's `String::with_capacity` + `format!` + `Vec::join` achieves the same result idiomatically.

## OCaml Approach
- Optional labeled argument `?(indent=0)` gives default indentation cleanly
- `String.make n ' '` builds padding strings
- `String.concat` joins lists of strings with separator
- `Buffer` used for escape_string to avoid O(n²) string concatenation
- Pattern matching handles empty vs non-empty arrays/objects separately

## Rust Approach
- Plain `usize` parameter for indent (no default args — use wrapper fn if needed)
- `" ".repeat(n)` builds padding strings
- `.collect::<Vec<String>>()` then `.join(",\n")` mirrors `String.concat`
- `format!` for string interpolation instead of `Printf.sprintf`
- `if items.is_empty()` guard before match arm (or use pattern guard)

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| String building | `Buffer.add_string` / `^` | `String::push_str` / `format!` |
| Joining list | `String.concat sep list` | `vec.join(sep)` |
| Default args | `?(indent=0)` | No default args — overload or wrapper |
| Padding | `String.make n ' '` | `" ".repeat(n)` |
| Char escaping | `String.iter` + `match c` | `for c in s.chars()` + `match c` |
| Empty collection | Pattern `Array []` | Pattern guard `if items.is_empty()` |
| Float formatting | `Printf.sprintf "%g"` | `format!("{}", n)` |
