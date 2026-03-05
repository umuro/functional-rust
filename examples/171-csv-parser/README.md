# 171: CSV Parser

**Difficulty:** 3  **Level:** Advanced

A complete CSV parser — quoted fields, escaped quotes, embedded newlines — built from the combinators you've learned.

## The Problem This Solves

CSV looks trivial until you try to parse it correctly. `Alice,30,Engineer` is easy. But what about `"Smith, Jr.",42,"Software Engineer"` — commas inside quoted fields? Or `"line one\nline two"` — newlines inside quoted fields? Or `"He said ""hello"""` — escaped quotes encoded as `""`?

The RFC 4180 spec for CSV handles all of these with one quoting rule: fields wrapped in double quotes can contain anything; two consecutive double quotes inside a quoted field mean one literal double quote. Building this correctly from scratch requires careful state management and proves that parser combinators scale to real-world formats.

This is also the payoff example: you wrote `tag`, `take_while`, `separated_list`, and `many0` in earlier examples. Now you assemble them into a complete, working parser for a format used in millions of data pipelines.

## The Intuition

A CSV file is `rows`, where each row is `fields separated by commas`, where each field is either `unquoted` (no special chars) or `quoted` (wrapped in `"`, with `""` escaping). Parse bottom-up: field → row → file.

```
"Alice","Smith, Jr.",42
 ^quoted  ^quoted, has comma  ^unquoted
```

## How It Works in Rust

```rust
// Unquoted field: everything up to the next comma or newline
fn parse_unquoted_field(input: &str) -> ParseResult<String> {
    let end = input.find(|c| c == ',' || c == '\n' || c == '\r')
        .unwrap_or(input.len());
    Ok((input[..end].to_string(), &input[end..]))
}

// Quoted field: "..." with "" meaning a literal quote inside
fn parse_quoted_field(input: &str) -> ParseResult<String> {
    let input = input.strip_prefix('"')
        .ok_or("expected '\"'")?;
    let mut result = String::new();
    let mut chars = input.char_indices();

    loop {
        match chars.next() {
            None => return Err("unterminated quoted field".to_string()),
            Some((_, '"')) => {
                // Peek: is the next char also '"'? That's an escaped quote.
                match chars.clone().next() {
                    Some((_, '"')) => {
                        chars.next(); // consume the second quote
                        result.push('"');
                    }
                    _ => {
                        // Closing quote — find current position in input
                        let pos = /* offset after closing quote */;
                        return Ok((result, &input[pos..]));
                    }
                }
            }
            Some((_, ch)) => result.push(ch),
        }
    }
}

// One field: try quoted first, fall back to unquoted
fn parse_field(input: &str) -> ParseResult<String> {
    if input.starts_with('"') {
        parse_quoted_field(input)
    } else {
        parse_unquoted_field(input)
    }
}

// One row: comma-separated fields
fn parse_row(input: &str) -> ParseResult<Vec<String>> {
    let comma = |s: &str| s.strip_prefix(',')
        .map(|r| ((), r))
        .ok_or("expected ','".to_string());
    separated_list1(comma, parse_field)(input)
}

// Full CSV: rows separated by newlines
fn parse_csv(input: &str) -> ParseResult<Vec<Vec<String>>> {
    let mut rows = Vec::new();
    let mut remaining = input;
    while !remaining.is_empty() {
        let (row, rest) = parse_row(remaining)?;
        rows.push(row);
        // Skip \r\n or \n between rows
        remaining = rest.strip_prefix("\r\n")
            .or_else(|| rest.strip_prefix('\n'))
            .unwrap_or(rest);
    }
    Ok((rows, remaining))
}
```

## What This Unlocks

- **Real data ingestion** — parse CSV exports from Excel, databases, APIs.
- **Combinator payoff** — see how all the primitives (tag, take_while, separated_list) compose into something production-usable.
- **Quoting patterns** — the `""` → `"` escape pattern reappears in SQL strings, INI files, and more.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String building | `Buffer.t` + `Buffer.add_char` | `String::new()` + `String::push` |
| Line endings | Manual `\r\n` check | `starts_with("\r\n")` then `starts_with('\n')` |
| Char iteration | Recursive with index | `char_indices()` iterator |
| Field trimming | `String.trim` | `str::trim()` |
