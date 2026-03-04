# 499: Escaping and Unescaping Strings

**Difficulty:** 2  **Level:** Intermediate

Safely encode special characters for HTML, JSON, and display — and reverse the process.

## The Problem This Solves

Every format that embeds strings has special characters that need escaping. HTML can't contain raw `<` — it must be `&lt;`. JSON can't contain unescaped `"` — it must be `\"`. Log output can't contain unescaped newlines if you want one-line-per-entry formatting.

Python has `html.escape()`, `json.dumps()`. JavaScript has `encodeURIComponent()`, `JSON.stringify()`. These are usually library functions. Rust's standard library doesn't include HTML or JSON escaping — those live in crates — but the *pattern* of implementing escaping is worth knowing because it appears in parsers, serializers, and any code that bridges format boundaries.

Implementing your own escaper in Rust also teaches an important pattern: character-by-character transformation with `flat_map` on `chars()`, producing multiple output characters per input character. The `escape_control` function shows how to build stateful unescaping with a `Peekable` iterator.

## The Intuition

Escaping is "replace this char with a safe representation." Unescaping is the reverse. Both are string transformations.

For escaping: iterate chars, match on special ones, emit their replacement. The `flat_map` approach is idiomatic — each input char produces 0 or more output chars. Collect into a new `String`.

For unescaping: you need to look ahead — `\n` is two chars that represent one. Use a `Peekable` iterator or a `while let` loop that calls `.next()` explicitly when you see the escape character.

Rust's built-in `{:?}` format already escapes strings for debug output. If you just need to print a string in a way that shows all special characters visibly, `println!("{:?}", s)` is instant and free.

## How It Works in Rust

```rust
// HTML escaping — flat_map: each char → 0 or more output chars
fn escape_html(s: &str) -> String {
    s.chars().flat_map(|c| match c {
        '<'  => "&lt;".chars().collect::<Vec<_>>(),
        '>'  => "&gt;".chars().collect(),
        '&'  => "&amp;".chars().collect(),
        '"'  => "&quot;".chars().collect(),
        '\'' => "&#39;".chars().collect(),
        c    => vec![c],
    }).collect()
}

// HTML unescaping — simple: replace known sequences
fn unescape_html(s: &str) -> String {
    s.replace("&lt;", "<")
     .replace("&gt;", ">")
     .replace("&amp;", "&")
     .replace("&quot;", "\"")
     .replace("&#39;", "'")
}

let html = "<div class=\"hello\">Hello & World!</div>";
let escaped = escape_html(html);
// "&lt;div class=&quot;hello&quot;&gt;Hello &amp; World!&lt;/div&gt;"

assert_eq!(unescape_html(&escaped), html);  // roundtrip ✓

// Control character escaping — peekable iterator for unescaping
fn escape_control(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            '\\' => out.push_str("\\\\"),
            '"'  => out.push_str("\\\""),
            c    => out.push(c),
        }
    }
    out
}

fn unescape_control(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut iter = s.chars().peekable();
    while let Some(c) = iter.next() {
        if c == '\\' {
            match iter.next() {  // consume the NEXT char to interpret the escape
                Some('n')  => out.push('\n'),
                Some('t')  => out.push('\t'),
                Some('\\') => out.push('\\'),
                Some('"')  => out.push('"'),
                Some(other) => { out.push('\\'); out.push(other); }
                None        => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

// Built-in: debug format escapes automatically
let s = "hello\nworld\ttab";
println!("{:?}", s);  // "hello\nworld\ttab"  — visible escapes
```

## What This Unlocks

- **HTML generation** — escape user input before inserting into HTML to prevent XSS.
- **Log formatting** — escape newlines in log values to maintain one-log-per-line format.
- **Custom serialization** — implement the escape/unescape loop pattern for any format (CSV, TOML, protocol buffers).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| HTML escape | Manual Buffer loop | Manual with `flat_map` (or `html-escape` crate) |
| Control char escape | Manual Buffer loop | Manual with `push_str` per char |
| Char-to-multi-char | `Buffer.add_string buf "..."` | `flat_map` → expand each char |
| Stateful unescaping | Manual `ref` loop | `Peekable` iterator + `iter.next()` |
| Debug string repr | `Printf.printf "%s"` | `{:?}` — built-in, shows escapes |
| JSON escaping | No std support | `serde_json` crate |
