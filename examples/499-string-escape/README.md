📖 **[View on hightechmind.io →](https://hightechmind.io/rust/499-string-escape)**

---

# String Escaping
**Difficulty:** ⭐  
**Category:** Functional Programming  



Escaping and unescaping strings — for HTML, JSON, SQL, and shell contexts — requires replacing special characters with entity sequences on output and reversing the process on input, handling both streaming and complete-string variants.

## Problem Statement

Injecting unescaped user input into HTML causes XSS; into SQL causes injection; into shell commands causes command injection. Every output context has a set of special characters that must be escaped. The inverse — unescaping — is equally important when reading stored or transmitted data. A correct implementation must handle the escape sequences as a state machine (for unescaping, `\` introduces a two-character sequence), not just a series of `replace` calls that could double-escape.

## Learning Outcomes

- Escape HTML entities (`<`, `>`, `&`, `"`, `'`) using `flat_map` over chars
- Unescape HTML entities using sequential `replace` calls (order matters: `&amp;` last)
- Escape control characters (`\n`, `\t`, `\r`, `\\`, `"`) with a `push_str` pattern
- Unescape with a stateful character iterator using `Peekable`
- Understand why `unescape_html` must replace `&amp;` last to avoid double-unescaping

## Rust Application

`escape_html` uses `flat_map` to expand each special char to its entity:

```rust
fn escape_html(s: &str) -> String {
    s.chars().flat_map(|c| match c {
        '<' => "&lt;".chars().collect::<Vec<_>>(),
        '>' => "&gt;".chars().collect(),
        '&' => "&amp;".chars().collect(),
        '"' => "&quot;".chars().collect(),
        '\'' => "&#39;".chars().collect(),
        c => vec![c],
    }).collect()
}
```

`unescape_control` uses a `Peekable` iterator to consume escape sequences as two-character tokens:

```rust
while let Some(c) = iter.next() {
    if c == '\\' {
        match iter.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            // ...
        }
    } else { out.push(c); }
}
```

## OCaml Approach

```ocaml
let escape_html s =
  let buf = Buffer.create (String.length s) in
  String.iter (fun c -> match c with
    | '<' -> Buffer.add_string buf "&lt;"
    | '>' -> Buffer.add_string buf "&gt;"
    | '&' -> Buffer.add_string buf "&amp;"
    | '"' -> Buffer.add_string buf "&quot;"
    | '\'' -> Buffer.add_string buf "&#39;"
    | c -> Buffer.add_char buf c) s;
  Buffer.contents buf
```

OCaml's `Buffer`-based approach avoids intermediate allocation. The `tyxml` library provides HTML-safe string escaping as part of its typed HTML API; `yojson` and `ezjsonm` handle JSON escaping.

## Key Differences

1. **`flat_map` vs. `Buffer`**: Rust's `flat_map` is declarative but allocates intermediate `Vec<char>` per character; the `Buffer`/`push_str` approach (like OCaml's) is more allocation-efficient for hot paths.
2. **Unescape ordering**: Both Rust and OCaml must unescape `&amp;` last in HTML unescaping; the `replace` chain is equivalent to sequential OCaml `Buffer` scans.
3. **`Peekable` iterator**: Rust's `chars().peekable()` provides lookahead for the two-character escape state machine; OCaml uses integer index advancement.
4. **Context-specific escaping**: Rust's `html-escape` crate and OCaml's `tyxml` provide well-tested context-aware escaping; handwritten implementations are error-prone for edge cases.

## Exercises

1. **JSON string escaping**: Extend `escape_control` to produce a valid JSON string (wrap in `"`, escape all required characters per RFC 8259 including Unicode escapes `\uXXXX` for control chars).
2. **SQL escaping**: Implement `escape_sql_string(s: &str) -> String` that escapes single quotes by doubling them (`'` → `''`), suitable for SQL string literals (but note: parameterised queries are always preferred).
3. **Round-trip property test**: Write a `proptest` test that verifies `unescape_html(escape_html(s)) == s` for all strings not containing existing HTML entities.
