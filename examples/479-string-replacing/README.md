📖 **[View on hightechmind.io →](https://hightechmind.io/rust/479-string-replacing)**

---

# String Replacing
**Difficulty:** ⭐  
**Category:** Functional Programming  



Rust provides `replace` for substituting all occurrences, `replacen` for a capped substitution count, and `retain` for in-place character filtering — covering the most common string transformation patterns.

## Problem Statement

Sanitising input, normalising separators, redacting sensitive data, and applying text patches all require replacing substrings. The key design decisions are: replace all occurrences vs. a fixed count; return a new string vs. modify in-place; match on exact substring vs. a predicate. Rust's replace API covers all these cases with allocating (`replace`/`replacen`) and in-place (`retain`) variants.

## Learning Outcomes

- Replace all occurrences of a pattern with `.replace(from, to)` returning a new `String`
- Limit replacements with `.replacen(from, to, n)`
- Filter characters in-place with `String::retain(|c| predicate)`
- Understand that `replace` always allocates a new string even if no substitution occurs
- Recognise the pattern `pat` can be a char, `&str`, or closure in both `replace` and `retain`

## Rust Application

`.replace` scans the string and builds a new `String` with substitutions:

```rust
"aabaa".replace('a', "x")         // "xxbxx" — all occurrences
"aabaa".replacen('a', "x", 2)     // "xxbaa" — first 2 only
"hello".replace("xyz", "abc")     // "hello" — no match, still allocates
```

`retain` is the in-place counterpart — it modifies a `&mut String` keeping only chars matching the predicate:

```rust
let mut s = String::from("h3llo");
s.retain(|c| c.is_alphabetic());
assert_eq!(s, "hllo");
```

`retain` is O(N) and does not allocate; it is equivalent to `filter` + `collect` but avoids the intermediate allocation.

## OCaml Approach

OCaml's standard library has no `replace`; the `Str` module provides:

```ocaml
Str.global_replace (Str.regexp_string "a") "x" "aabaa"  (* "xxbxx" *)
Str.replace_first  (Str.regexp_string "a") "x" "aabaa"  (* "xabaa" *)
```

For `replacen` behaviour, manual recursion or `String.concat` is needed. In-place filtering:

```ocaml
let retain pred s =
  String.concat "" (List.filter_map
    (fun c -> if pred c then Some (String.make 1 c) else None)
    (List.of_seq (String.to_seq s)))
```

## Key Differences

1. **Standard library**: Rust's `replace`/`replacen` are on `str` with no imports; OCaml requires `Str` (a separate library) or `Re` (third-party).
2. **In-place `retain`**: Rust's `retain` modifies a `String` without reallocating (if possible); OCaml always allocates a new string for filtered results.
3. **`replacen`**: Rust has a `n`-replacement limit built in; OCaml's `Str` has `replace_first` (one) and `global_replace` (all) but no direct `n`-replacement.
4. **Pattern types**: Rust's `replace` accepts `char`, `&str`, `&[char]`, and closures; `Str.global_replace` requires a compiled `Str.regexp`.

## Exercises

1. **Template engine**: Write `render(template: &str, vars: &HashMap<&str, &str>) -> String` that replaces `{{key}}` placeholders using `replace` in a loop over the map.
2. **Redact emails**: Write `redact_emails(text: &str) -> String` using a `Regex` (from the `regex` crate) to replace email addresses with `[REDACTED]`.
3. **Benchmark retain vs. filter+collect**: Use `criterion` to compare `s.retain(|c| c.is_alphanumeric())` against `s.chars().filter(|c| c.is_alphanumeric()).collect::<String>()` on a 10KB string.
