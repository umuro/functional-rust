# 497: Case Conversion Patterns

**Difficulty:** 1  **Level:** Beginner

Unicode-aware case conversion — plus snake_case, camelCase, and title case transformations.

## The Problem This Solves

Python's `.upper()` and `.lower()` are Unicode-aware. JavaScript's `.toUpperCase()` and `.toLowerCase()` are too. Rust is also Unicode-aware for case conversion — but with a twist: `to_uppercase()` returns an **iterator**, not a `String`. This is because some characters expand when uppercased: the German `ß` becomes `SS` (two characters). Returning an iterator avoids the awkward question of whether to pre-allocate.

For ASCII-only text where you know there are no special characters, Rust provides `to_ascii_uppercase()` and `to_ascii_lowercase()` which return a `String` directly and are faster — no Unicode table lookup.

Beyond simple case flipping, case conversion often means transforming between naming conventions: `MyFunctionName` → `my_function_name` (snake_case), `my_function_name` → `myFunctionName` (camelCase). Rust's std library doesn't include these, but they're straightforward to implement with the character iterator tools you've already seen.

## The Intuition

`to_uppercase()` returns `ToUppercase` — an iterator of `char`. To get a `String`, chain `.collect()`. This is different from Python and OCaml which return a string directly. The reason: Unicode case mappings aren't always 1-to-1 in character count.

For ASCII text (English, numbers, common symbols), use `to_ascii_uppercase()` — it returns `String` directly and is faster because it only handles the ASCII range, no Unicode table needed.

Case convention transformations (snake, camel, title) are just char-level iterator operations: detect uppercase chars, insert underscores, capitalize first letters. The pattern: `chars()` + `map`/`flat_map` + `collect()`.

## How It Works in Rust

```rust
let s = "Hello, World! café";

// Unicode-aware (ß → SS, returns iterator)
let upper: String = s.chars().flat_map(|c| c.to_uppercase()).collect();
// Shorthand: s.to_uppercase() also works (collects for you)
println!("{}", s.to_uppercase());  // "HELLO, WORLD! CAFÉ"
println!("{}", s.to_lowercase());  // "hello, world! café"

// Special case: German ß
"straße".to_uppercase()   // "STRASSE" — expands to 7 chars from 6!

// ASCII-only (faster, no Unicode table lookup)
s.to_ascii_uppercase()   // returns String, only affects A-Z
s.to_ascii_lowercase()   // returns String, only affects a-z

// snake_case: detect uppercase, insert underscore before it
fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 { out.push('_'); }
            out.extend(c.to_lowercase()); // handles ß-style expansions
        } else {
            out.push(c);
        }
    }
    out
}
// "MyFunctionName" → "my_function_name"

// camelCase: split on _, capitalize first char of each word (except first)
fn to_camel_case(s: &str) -> String {
    s.split('_').enumerate().flat_map(|(i, word)| {
        let mut chars = word.chars();
        if i == 0 {
            chars.map(|c| c).collect::<String>()
        } else {
            let first = chars.next()
                .map(|c| c.to_uppercase().collect::<String>())
                .unwrap_or_default();
            first + chars.as_str()
        }.chars().collect::<Vec<_>>()
    }).collect()
}
// "my_function_name" → "myFunctionName"

// Title case: capitalize first letter of each word
fn to_title_case(s: &str) -> String {
    s.split_whitespace().map(|word| {
        let mut cs = word.chars();
        cs.next()
            .map(|c| c.to_uppercase().collect::<String>() + cs.as_str())
            .unwrap_or_default()
    }).collect::<Vec<_>>().join(" ")
}
// "the quick brown fox" → "The Quick Brown Fox"
```

## What This Unlocks

- **Database and API field naming** — convert between JSON camelCase and Rust snake_case conventions automatically.
- **User-facing text** — title case for headings, lowercase for normalization before comparison.
- **Code generation** — transform identifiers between naming conventions when generating source code.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Uppercase | `String.map Char.uppercase_ascii s` (ASCII only) | `s.to_uppercase()` (Unicode) |
| Lowercase | `String.map Char.lowercase_ascii s` (ASCII only) | `s.to_lowercase()` (Unicode) |
| Return type | `string` | Iterator — `.collect::<String>()` needed |
| ASCII-only fast path | Default behavior | `to_ascii_uppercase()` → `String` |
| ß expansion | Not handled | `"straße".to_uppercase()` → `"STRASSE"` |
| snake_case | Manual with Buffer | Manual with `chars()` + `push` |
| Title case | Manual with Buffer | Manual with `split_whitespace()` |
