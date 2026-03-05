📖 **[View on hightechmind.io →](https://hightechmind.io/rust/175-json-parser)**

---

# 175: Complete JSON Parser

**Difficulty:** 3  **Level:** Advanced

Parse all of JSON — null, booleans, numbers, strings, arrays, objects — from scratch. Zero dependencies. This is the capstone.

## The Problem This Solves

JSON is the universal data exchange format. Understanding how to parse it teaches you everything: keyword matching (`null`, `true`, `false`), number scanning with scientific notation, string parsing with escape sequences including `\uXXXX` Unicode escapes, recursive data structures (arrays contain JSON values, objects contain JSON values), and the full dispatch-on-first-character technique.

Every technique from examples 151–174 appears here. `tag` for keywords. `parse_number` for numbers. `separated_list0` for array elements and object entries. Recursive calls for nested arrays and objects. `ws0` to skip whitespace between tokens. The `Display` trait for round-trip serialization.

This is the *proof that the approach works at scale*. A hand-written JSON parser in ~200 lines of Rust, no external crates, fully spec-compliant, with round-trip testing.

## The Intuition

JSON has six value types. Dispatch on the first character: `'n'` → `null`, `'t'`/`'f'` → boolean, `'"'` → string, `'['` → array, `'{'` → object, digit/`'-'` → number. Parse that value, return the result and the remaining input.

```
input: {"x": [1, null, true]}
dispatch '{' → parse_object
  key: "x"
  dispatch '[' → parse_array
    dispatch '1' → parse_number → 1.0
    dispatch 'n' → parse_keyword "null" → Null
    dispatch 't' → parse_keyword "true" → Bool(true)
  close ']' → Array([Number(1.0), Null, Bool(true)])
close '}' → Object([("x", Array(...))])
```

## How It Works in Rust

```rust
#[derive(Debug, Clone)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

fn parse_json(input: &str) -> ParseResult<Json> {
    let input = input.trim_start();  // ws0 built-in
    match input.as_bytes().first() {
        Some(b'n') => tag("null")(input).map(|(_, r)| (Json::Null, r)),
        Some(b't') => tag("true")(input).map(|(_, r)| (Json::Bool(true), r)),
        Some(b'f') => tag("false")(input).map(|(_, r)| (Json::Bool(false), r)),
        Some(b'"') => parse_json_string(input),
        Some(b'[') => parse_json_array(&input[1..]),
        Some(b'{') => parse_json_object(&input[1..]),
        Some(b'-') | Some(b'0'..=b'9') => {
            parse_number(input).map(|(n, r)| (Json::Number(n), r))
        }
        _ => Err(format!("unexpected input: {:?}", &input[..10.min(input.len())])),
    }
}

fn parse_json_string(input: &str) -> ParseResult<Json> {
    // Strip opening quote, scan char by char
    let input = &input[1..];
    let mut result = String::new();
    let mut chars = input.char_indices().peekable();
    loop {
        match chars.next() {
            None => return Err("unterminated string".to_string()),
            Some((_, '"')) => {
                let pos = chars.next().map(|(i, _)| i).unwrap_or(input.len());
                // Adjust pos: it's the index of the char AFTER the closing quote
                // Use byte offset tracking for correctness
                break; // (simplified — see example.rs for full byte tracking)
            }
            Some((_, '\\')) => match chars.next() {
                Some((_, '"'))  => result.push('"'),
                Some((_, '\\')) => result.push('\\'),
                Some((_, '/'))  => result.push('/'),
                Some((_, 'n'))  => result.push('\n'),
                Some((_, 't'))  => result.push('\t'),
                Some((_, 'r'))  => result.push('\r'),
                Some((_, 'u'))  => {
                    // \uXXXX — collect 4 hex digits
                    let hex: String = (0..4).filter_map(|_| chars.next().map(|(_, c)| c)).collect();
                    let code = u32::from_str_radix(&hex, 16)
                        .map_err(|_| "invalid \\uXXXX escape")?;
                    result.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
                }
                _ => return Err("invalid escape".to_string()),
            },
            Some((_, ch)) => result.push(ch),
        }
    }
    Ok((Json::Str(result), /* remaining */))
}

fn parse_json_array(input: &str) -> ParseResult<Json> {
    let comma = |s: &str| s.trim_start().strip_prefix(',')
        .map(|r| ((), r)).ok_or("expected ','".to_string());
    let item  = |s: &str| parse_json(s);
    let (items, rest) = separated_list0(comma, item)(input.trim_start())?;
    let rest = rest.trim_start().strip_prefix(']').ok_or("expected ']'")?;
    Ok((Json::Array(items), rest))
}

// Display: round-trip serialization
impl std::fmt::Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Json::Null       => write!(f, "null"),
            Json::Bool(b)    => write!(f, "{}", b),
            Json::Number(n)  => write!(f, "{}", n),
            Json::Str(s)     => write!(f, "\"{}\"", s.replace('"', "\\\"")),
            Json::Array(xs)  => {
                write!(f, "[")?;
                for (i, x) in xs.iter().enumerate() {
                    if i > 0 { write!(f, ",")?; }
                    write!(f, "{}", x)?;
                }
                write!(f, "]")
            }
            Json::Object(kv) => {
                write!(f, "{{")?;
                for (i, (k, v)) in kv.iter().enumerate() {
                    if i > 0 { write!(f, ",")?; }
                    write!(f, "\"{}\":{}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}
```

## What This Unlocks

- **The full picture** — every parser combinator technique from this series in one working program.
- **Dependency-free JSON** — useful in `no_std` contexts, embedded systems, or anywhere `serde_json` is overkill.
- **Template for other formats** — YAML, TOML, MessagePack all follow the same dispatch-on-type pattern.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| JSON type | `type json = Null \| Bool of bool \| Number of float \| Str of string \| Array of json list \| Object of (string * json) list` | `enum Json { Null, Bool(bool), Number(f64), Str(String), Array(Vec<Json>), Object(Vec<(String, Json)>) }` |
| Dispatch | `match s.[0] with 'n' → ...` | `match s.as_bytes().first() { Some(b'n') → ... }` |
| String building | `Buffer.t` + `Buffer.add_char` | `String::new()` + `String::push` |
| Unicode escapes | Manual (often omitted in examples) | `char::from_u32(code).unwrap_or('\u{FFFD}')` |
| Round-trip | `json_to_string` function | `impl Display for Json` |
| Memory | GC-managed values | Owned `String` and `Vec` — explicit ownership |
