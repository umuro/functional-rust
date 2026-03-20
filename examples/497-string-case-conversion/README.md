📖 **[View on hightechmind.io →](https://hightechmind.io/rust/497-string-case-conversion)**

---

# String Case Conversion
**Difficulty:** ⭐  
**Category:** Functional Programming  


Rust's standard library provides `to_uppercase`/`to_lowercase` for Unicode-correct case folding, while `snake_case`, `camelCase`, and `Title Case` conversions are implemented with `char_indices`, `split`, and `flat_map` iterator combinators.

## Problem Statement

Case conversion appears in: code generation (struct names in camelCase, field names in snake_case), API normalisation (HTTP headers are case-insensitive), URL slugs (lowercase-with-hyphens), and display formatting (title case for headings). `str::to_uppercase` handles the simple case but does not perform format conversion between naming conventions. These require splitting on boundaries (`_`, uppercase chars, spaces) and reassembling with different rules.

## Learning Outcomes

- Use `.to_uppercase()` and `.to_lowercase()` for Unicode-correct case conversion
- Implement `snake_case` conversion by inserting `_` before uppercase chars
- Implement `camelCase` by capitalising the first char of each `_`-delimited word
- Implement `Title Case` by capitalising the first char of each whitespace-delimited word
- Apply `char_indices`, `flat_map`, and `split` for case transformation pipelines

## Rust Application

`to_snake_case` inserts `_` before each uppercase letter (except the first):

```rust
fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 { out.push('_'); }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}
```

`to_camel_case` splits on `_` and capitalises each word after the first using `chars().next()` + `to_uppercase()`:

```rust
fn to_camel_case(s: &str) -> String {
    s.split('_').enumerate().flat_map(|(i, word)| { ... }).collect()
}
```

`to_title_case` maps `split_whitespace` words through first-char capitalisation.

## OCaml Approach

```ocaml
(* Simple upper/lower via standard library *)
String.uppercase_ascii "hello"  (* "HELLO" *)
String.lowercase_ascii "HELLO"  (* "hello" *)

(* snake_case — manual loop *)
let to_snake_case s =
  let buf = Buffer.create (String.length s) in
  String.iteri (fun i c ->
    if Char.uppercase_ascii c = c && c <> ' ' && i > 0
    then (Buffer.add_char buf '_'; Buffer.add_char buf (Char.lowercase_ascii c))
    else Buffer.add_char buf c) s;
  Buffer.contents buf
```

The `stringext` library provides `String.split_on_string` and helpers; `snake-case` and `camelCase` conversions are common in code generation libraries like `ppx_deriving`.

## Key Differences

1. **Unicode case**: Rust's `to_uppercase`/`to_lowercase` are Unicode-correct (`'ß'.to_uppercase() == "SS"`); OCaml's `String.uppercase_ascii` is ASCII-only.
2. **Iterator combinators**: Rust's `flat_map` + `collect` for `camelCase` is idiomatic; OCaml requires `Buffer` + imperative loops for the same transformation.
3. **`char.to_lowercase()`**: Returns a `ToLowercase` iterator (because one char can expand to multiple chars, e.g. `'ß'`); OCaml's `Char.lowercase_ascii` returns a single `char`.
4. **Convention support**: Rust's ecosystem has the `heck` crate for all naming convention conversions; OCaml needs `stringext` or manual code.

## Exercises

1. **kebab-case**: Implement `to_kebab_case(s: &str) -> String` that converts `CamelCase` or `snake_case` to `kebab-case`.
2. **SCREAMING_SNAKE_CASE**: Implement `to_screaming_snake(s: &str) -> String` for constant naming (`MAX_VALUE`).
3. **Round-trip property test**: Write a property test (using `proptest`) that verifies `camelCase → snake_case → camelCase` is an identity for ASCII-alphabetic identifiers.
