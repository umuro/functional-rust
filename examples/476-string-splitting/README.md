📖 **[View on hightechmind.io →](https://hightechmind.io/rust/476-string-splitting)**

---

# String Splitting
**Difficulty:** ⭐  
**Category:** Functional Programming  



Rust provides `split`, `splitn`, `split_once`, and `split_whitespace` for dividing strings into parts, all returning lazy iterators that yield `&str` slices into the original string without allocation.

## Problem Statement

Parsing structured text — CSV records, HTTP headers, key=value config lines, shell command tokens — requires splitting strings on delimiters. A good split API must handle: unlimited splits, capped splits, single split at first delimiter, and whitespace-normalised tokenisation. Rust's split family covers all cases with a consistent iterator-based interface, avoiding the allocation of a `Vec<String>` unless the caller explicitly collects.

## Learning Outcomes

- Split on a char, `&str`, or closure predicate with `.split()`
- Limit the number of parts with `.splitn(n, pat)`, leaving the remainder unsplit
- Extract a key-value pair efficiently with `.split_once(pat)` returning `Option<(&str, &str)>`
- Tokenise whitespace-separated input with `.split_whitespace()`, handling multiple spaces
- Chain split results with iterator adaptors before collecting

## Rust Application

`.split(pat)` yields all parts as `&str` slices; collecting into `Vec` is optional:

```rust
"a,b,c".split(',').collect::<Vec<_>>()  // ["a", "b", "c"]
```

`.splitn(3, ':')` stops after producing 3 parts — the last part is the remainder:

```rust
"a:b:c:d".splitn(3, ':')  // ["a", "b", "c:d"]
```

`.split_once('=')` is the idiomatic way to parse `key=value` without regex:

```rust
"k=v=extra".split_once('=')  // Some(("k", "v=extra"))
```

`.split_whitespace()` splits on any whitespace and skips empty tokens, so `"  a  b  "` yields `["a", "b"]`.

## OCaml Approach

OCaml 4.04+ has `String.split_on_char`:

```ocaml
String.split_on_char ',' "a,b,c"  (* ["a"; "b"; "c"] *)
```

For `splitn`-equivalent behaviour, the `Str` module provides `Str.bounded_split`:

```ocaml
Str.bounded_split (Str.regexp ",") "a,b,c,d" 3  (* ["a"; "b"; "c,d"] *)
```

`split_once` has no direct equivalent; the idiom is `match String.index_opt s '=' with Some i -> ...`. The `astring` library provides a richer split API.

## Key Differences

1. **Lazy vs. eager**: Rust's `split` returns a lazy `Split` iterator; OCaml's `split_on_char` returns an allocated `string list` immediately.
2. **`split_once`**: Rust's standard library includes `split_once`; OCaml requires manual `index_opt` + `sub` or an external library.
3. **Pattern types**: Rust's `split` accepts `char`, `&str`, `&[char]`, or any `Pattern` (including closures); OCaml's standard `split_on_char` accepts only `char`.
4. **Empty tokens**: Rust's `split` yields empty strings between adjacent delimiters; `split_whitespace` skips them. OCaml's `split_on_char` also yields empty strings.

## Exercises

1. **CSV line parser**: Write `parse_csv_line(s: &str) -> Vec<&str>` that splits on commas and trims whitespace from each field, returning slices into the original string.
2. **Key-value config**: Write `parse_config(text: &str) -> HashMap<&str, &str>` that processes each line with `split_once('=')`, ignoring lines without `=`.
3. **Re-join with modified parts**: Split a path `"a/b/c/d"` on `/`, uppercase each component, then rejoin with `::` — implement without intermediate `Vec<String>` allocation.
