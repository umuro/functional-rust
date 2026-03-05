# 763: JSON-Like Format Built From Scratch

**Difficulty:** 4  **Level:** Advanced

A complete recursive AST, hand-written serializer via `Display`, and a recursive-descent parser — all in ~200 lines of safe, std-only Rust.

## The Problem This Solves

Every web developer uses JSON daily without understanding the parsing machinery behind it. When you need to parse a domain-specific language, a configuration format, or a protocol, you need to write a parser. The JSON grammar is small enough to implement in a few hundred lines, but rich enough to teach every technique you'll need: recursive types, string escaping, number parsing, whitespace handling, and proper error messages.

Understanding how JSON works from the inside also makes you a better user of `serde_json`. You'll know why certain shapes are valid, why nested objects require recursion, and why string escaping is non-trivial. When `serde_json` produces an unexpected result, you'll be able to reason about what the parser is doing.

This is also a practical foundation. Real-world formats like TOML, custom DSLs, and binary protocols use the same structural patterns: an AST (enum with recursive variants), a serializer (walking the tree), and a parser (consuming bytes and building the tree).

## The Intuition

A JSON value is one of: null, bool, number, string, array (of values), or object (of string→value pairs). That's a recursive enum in Rust — `Vec<Json>` and `Vec<(String, Json)>` reference the same type being defined. Rust's `Box<T>` handles the heap allocation that makes recursive types work.

The serializer is a `Display` implementation that walks the tree and writes output. The parser is a `Parser` struct that holds a byte slice and a position, advancing through the input and calling itself recursively for nested values.

The same pattern appears in every parser you'll ever write: `peek()` looks at the current byte, `next()` consumes it, `skip_ws()` skips spaces, and specialized methods like `parse_string()` and `parse_value()` handle each grammar production.

## How It Works in Rust

```rust
// Recursive enum — Vec<Json> means this type contains itself
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),   // ordered like real JSON parsers
}

// Serializer — implement Display for pretty formatting
impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Json::Null          => write!(f, "null"),
            Json::Bool(b)       => write!(f, "{b}"),
            Json::Str(s)        => {
                write!(f, "\"")?;
                for c in s.chars() {
                    match c {
                        '"'  => write!(f, "\\\"")?,
                        '\\' => write!(f, "\\\\")?,
                        '\n' => write!(f, "\\n")?,
                        c    => write!(f, "{c}")?,
                    }
                }
                write!(f, "\"")
            }
            Json::Array(arr)    => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{v}")?;   // recursion via Display
                }
                write!(f, "]")
            }
            // Object and Number similar...
        }
    }
}

// Parser — position-based, recursive
struct Parser<'a> { s: &'a [u8], pos: usize }

impl<'a> Parser<'a> {
    fn parse_value(&mut self) -> Result<Json, ParseError> {
        self.skip_ws();
        match self.peek()? {
            b'"' => Ok(Json::Str(self.parse_string()?)),
            b't' => { self.pos += 4; Ok(Json::Bool(true)) }
            b'[' => {
                self.pos += 1;
                let mut arr = vec![];
                loop {
                    arr.push(self.parse_value()?);   // RECURSION
                    self.skip_ws();
                    match self.peek()? {
                        b',' => self.pos += 1,
                        b']' => { self.pos += 1; break }
                        _    => return Err(ParseError("expected ',' or ']'".into())),
                    }
                }
                Ok(Json::Array(arr))
            }
            // null, false, numbers, objects...
        }
    }
}

pub fn parse(s: &str) -> Result<Json, ParseError> {
    Parser::new(s).parse_value()
}
```

Key points:
- `Vec<Json>` in an enum variant requires no `Box` because `Vec` is already heap-allocated
- The parser keeps a `pos: usize` cursor into `&[u8]` — byte-level parsing avoids UTF-8 complexity for ASCII structure characters
- String parsing handles `\"`, `\\`, `\n`, `\t` — these are the tricky cases
- `Object` uses `Vec<(String, Json)>` not `HashMap` — preserves insertion order, same as V8's JSON parser

## What This Unlocks

- **Foundation for custom DSLs**: any language with expressions, nested structures, and literals follows this same pattern — grammar → AST → serializer + parser
- **Understand serde_json internals**: the `Json` enum here is structurally identical to `serde_json::Value`; this is what you're working with when you use `.as_array()` and `.as_object()`
- **Parser combinators**: once you understand recursive-descent parsing, `nom` and `winnow` become intuitive — they're just this pattern with reusable combinators

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive type | `type json = Null \| Array of json list` | `enum Json { Array(Vec<Json>) }` — `Vec` provides indirection |
| Serializer | `Format.fprintf` or `Buffer.add_string` | `impl fmt::Display` — integrate with `println!` |
| Parser state | Functional: thread `pos` through returns | Imperative: `Parser` struct with mutable `pos` |
| String escaping | Manual char-by-char | Same — match on `char` and write escape sequences |
| Error type | Exception or `result` | `struct ParseError(String)` |
| Production library | `yojson`, `ezjsonm` | `serde_json`, `json` |
