📖 **[View on hightechmind.io →](https://hightechmind.io/rust/477-string-trimming)**

---

# String Trimming
**Difficulty:** ⭐  
**Category:** Functional Programming  



Rust's trim family removes whitespace or custom characters from the start, end, or both ends of a string, returning a `&str` slice into the original — no allocation required.

## Problem Statement

User input, file contents, and network data almost always arrive with unwanted whitespace: leading spaces, trailing newlines, carriage returns from Windows line endings (`\r\n`). Stripping this noise before parsing or comparison is a universal preprocessing step. A well-designed trim API should be zero-copy (return a slice, not a new allocation), support both sides independently, and allow custom character matching.

## Learning Outcomes

- Remove leading and trailing whitespace with `.trim()`
- Strip only the start with `.trim_start()` or only the end with `.trim_end()`
- Remove custom characters or patterns with `.trim_matches(pat)`
- Understand that trim returns a `&str` pointing into the original bytes, not a new allocation
- Use `.trim_start_matches` and `.trim_end_matches` for prefix/suffix removal

## Rust Application

All trim methods return `&str` slices with the same lifetime as the input:

```rust
"  hi  ".trim()        // "hi"
"  hi  ".trim_start()  // "hi  "
"  hi  ".trim_end()    // "  hi"
"##hi##".trim_matches('#')  // "hi"
```

The returned slice points into the original string's memory:

```rust
let s = "  hi  ";
let t = s.trim();
assert!(t.as_ptr() >= s.as_ptr());  // t is a subslice of s
```

`trim_matches` accepts a `char`, `&[char]`, or a closure `|c: char| c.is_ascii_digit()`, enabling flexible custom stripping.

## OCaml Approach

OCaml's standard library does not include a `trim` function until OCaml 4.00's `String.trim`, which removes ASCII whitespace from both ends:

```ocaml
String.trim "  hi  "  (* "hi" *)
```

For one-sided trimming, the `astring` library provides `Astring.String.trim_left`/`trim_right`, or you can write it manually:

```ocaml
let ltrim s =
  let i = ref 0 in
  while !i < String.length s && s.[!i] = ' ' do incr i done;
  String.sub s !i (String.length s - !i)
```

OCaml's `String.sub` allocates a new string; there is no zero-copy slice type.

## Key Differences

1. **Zero-copy**: Rust's trim returns a borrowed slice with no allocation; OCaml's `String.trim` and `String.sub` always allocate.
2. **Pattern flexibility**: Rust's `trim_matches` accepts chars, char arrays, and closures; OCaml's `String.trim` removes only ASCII whitespace, requiring custom code for other patterns.
3. **One-sided trimming**: Rust has `trim_start`/`trim_end` in the standard library; OCaml requires `astring` or manual implementation.
4. **`trim_start_matches` vs. `strip_prefix`**: Rust additionally provides `strip_prefix(pat)` which removes the prefix exactly once (not greedily), returning `Option<&str>`.

## Exercises

1. **Strip CRLF**: Write `normalize_line_ending(s: &str) -> &str` that removes a trailing `\r\n` or `\n`, using `strip_suffix` rather than `trim_end_matches`.
2. **Trim custom set**: Write `trim_chars<'a>(s: &'a str, chars: &[char]) -> &'a str` using `trim_matches` with a closure that checks membership in the `chars` slice.
3. **Allocation count**: Use `criterion` to verify that trimming a 1000-byte string with `.trim()` performs zero heap allocations compared to a `String::from(s.trim())`.
