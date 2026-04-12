📖 **[View on hightechmind.io →](https://hightechmind.io/rust/486-string-regex-pattern)**

---

# String Regex Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  



Simple pattern matching — glob `*` wildcards and SQL LIKE `%`/`_` wildcards — can be implemented with pure `str` methods and recursive descent without pulling in a regex engine.

## Problem Statement

Regex engines are powerful but heavyweight: they compile patterns, maintain state machines, and have non-trivial binary size impact. Many real-world matching tasks need only simple patterns: file globs (`*.txt`), SQL LIKE queries (`user%`), or path prefix matching. These can be implemented in a few lines with `starts_with`, `ends_with`, and recursive char-slice matching — zero dependencies, no compilation step, and predictable performance.

## Learning Outcomes

- Implement a single-wildcard glob matcher using `starts_with` and `ends_with`
- Implement SQL LIKE pattern matching with recursive descent on `&[char]` slices
- Understand why recursion on slice patterns (`[h, ..rest]`) is cleaner than index loops
- Recognise when to use the `regex` crate vs. handwritten matchers
- Test edge cases: empty string, pure wildcard, no match, anchored match

## Rust Application

`glob_match` handles patterns with at most one `*`:

```rust
fn glob_match(pattern: &str, s: &str) -> bool {
    if let Some((pre, suf)) = pattern.split_once('*') {
        s.starts_with(pre) && s.ends_with(suf)
            && s.len() >= pre.len() + suf.len()
    } else {
        s == pattern
    }
}
```

`like_match` uses recursive descent on `Vec<char>` slices — Rust's slice patterns make the logic match the grammar directly:

```rust
fn rec(s: &[char], p: &[char]) -> bool {
    match (s, p) {
        (_, []) => s.is_empty(),
        ([], ['%', ..pr]) => rec(s, pr),
        ([_, ..sr], ['%', ..pr]) => rec(s, pr) || rec(sr, p),
        ([sc, ..sr], ['_', ..pr]) => rec(sr, pr),
        ([sc, ..sr], [pc, ..pr]) if sc == pc => rec(sr, pr),
        _ => false,
    }
}
```

## OCaml Approach

OCaml pattern matching on lists mirrors the Rust slice pattern approach:

```ocaml
let rec like_match s p = match s, p with
  | _, [] -> s = []
  | [], '%' :: pr -> like_match [] pr
  | _ :: sr, '%' :: pr -> like_match s pr || like_match sr p
  | _ :: sr, '_' :: pr -> like_match sr pr
  | sc :: sr, pc :: pr when sc = pc -> like_match sr pr
  | _ -> false
```

OCaml's `Str` module provides `Str.string_match` and `Str.regexp` for full regex; `Re` (third-party) provides a safer, more composable API.

## Key Differences

1. **Slice patterns**: Rust's `[h, rest @ ..]` destructuring on `&[char]` slices enables clean recursive descent; OCaml's list patterns `h :: t` are analogous but require `List.of_seq (String.to_seq s)` to convert.
2. **Allocation for pattern matching**: Both implementations convert the string to a `Vec<char>` / `char list` for recursive matching — this is a design trade-off for clarity over zero-copy.
3. **Standard regex**: OCaml's `Str` is in the standard library; Rust's `regex` crate is a separate dependency.
4. **Memoisation**: The LIKE recursion has exponential worst-case (multiple `%`). OCaml and Rust both benefit from memoisation via a `HashMap` on `(s_idx, p_idx)` pairs.

## Exercises

1. **Multi-wildcard glob**: Extend `glob_match` to handle multiple `*` wildcards by recursing on segments between `*` characters.
2. **Memoised LIKE**: Rewrite `like_match` with a `HashMap<(usize, usize), bool>` memo table to achieve O(N×M) time complexity.
3. **Benchmark vs. regex**: Use the `regex` crate to compile `^h.*o$` and `^he.lo$`, then benchmark against `glob_match` and `like_match` for 100,000 matches on 20-char strings.
