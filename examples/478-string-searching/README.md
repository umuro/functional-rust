📖 **[View on hightechmind.io →](https://hightechmind.io/rust/478-string-searching)**

---

# String Searching

Rust's string search methods — `contains`, `find`, `rfind`, `starts_with`, `ends_with`, and `matches` — provide zero-allocation substring and pattern searching with a unified `Pattern` trait.

## Problem Statement

Searching strings is fundamental: checking whether a URL starts with `"https://"`, finding the position of a delimiter for manual parsing, counting occurrences of a substring, validating file extensions. A well-designed API avoids returning raw byte indices without validation, distinguishes first-occurrence (`find`) from last-occurrence (`rfind`), and supports both exact substring and character-predicate matching through a single interface.

## Learning Outcomes

- Check substring presence with `.contains(pat)`
- Find the byte offset of the first match with `.find(pat)` returning `Option<usize>`
- Find the last occurrence with `.rfind(pat)`
- Check prefix/suffix with `.starts_with(pat)` / `.ends_with(pat)`
- Count non-overlapping occurrences with `.matches(pat).count()`

## Rust Application

`.contains`, `.find`, `.starts_with` all accept any `Pattern`: a `char`, `&str`, `&[char]`, or `|c: char| bool` closure:

```rust
"hello world".contains("world")  // true
"hello".find('l')                // Some(2)
"hello".rfind('l')               // Some(3)
"hello".starts_with("hel")       // true
"aaabaa".matches('a').count()    // 5
```

`.find` returns the **byte offset**, not a character index — safe to use with `&s[pos..]` because `find` always returns a valid char boundary. `matches` returns an iterator of overlapping or non-overlapping (default: non-overlapping) substrings.

## OCaml Approach

```ocaml
(* contains *)
let contains_sub s sub =
  let n = String.length sub in
  let found = ref false in
  for i = 0 to String.length s - n do
    if String.sub s i n = sub then found := true
  done; !found

(* find — String.index_opt for single char *)
String.index_opt "hello" 'l'   (* Some 2 *)

(* starts_with — OCaml 4.13+ *)
String.starts_with ~prefix:"hel" "hello"  (* true *)
```

OCaml 4.13 added `String.starts_with` and `String.ends_with`. For older versions and substring search, `Str.search_forward` or `astring` were the common choices.

## Key Differences

1. **Pattern abstraction**: Rust's `Pattern` trait unifies char, `&str`, `&[char]`, and closures under one API; OCaml has separate functions for each (`index_opt`, `contains`, `Str.search_forward`).
2. **Byte vs. char offset**: Rust's `find` returns a byte offset (always a valid char boundary for the match start); OCaml's `String.index_opt` returns a byte offset (same caveat for non-ASCII).
3. **`matches` iterator**: Rust's `.matches(pat)` returns a lazy iterator; OCaml has no standard equivalent — count requires a loop.
4. **`starts_with`/`ends_with`**: Available in OCaml only since 4.13; Rust has had them since 1.0.

## Exercises

1. **All occurrences**: Write `find_all(haystack: &str, needle: &str) -> Vec<usize>` that returns the byte offset of every non-overlapping occurrence using `find` in a loop.
2. **Fuzzy prefix match**: Write `matches_prefix_ci(s: &str, prefix: &str) -> bool` that does case-insensitive prefix checking without allocating a lowercase copy.
3. **Regex vs. `find`**: Benchmark `.contains("error")` vs. a compiled `Regex` pattern for scanning 10,000 log lines; measure when the regex overhead becomes worthwhile.
