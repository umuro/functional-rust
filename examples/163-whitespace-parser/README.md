📖 **[View on hightechmind.io →](https://hightechmind.io/rust/163-whitespace-parser)**

---

# 163: Whitespace Parser

**Difficulty:** 3  **Level:** Advanced

Skip, require, and wrap whitespace — the invisible plumbing every real parser needs.

## The Problem This Solves

Almost every language has whitespace between tokens: spaces between keywords, newlines between statements, indentation before blocks. A parser that doesn't handle whitespace will fail on `if x` because after parsing `"if"` it sees `" x"` — not `"x"`.

The naive fix is to manually call a whitespace-skipper before every token. That gets tedious fast and clutters your parser logic with noise. What you want is a small set of primitives: `ws0` (skip any amount), `ws1` (require at least one space), and `ws_wrap` (sandwich a parser in optional whitespace). Add `line_comment` and you can skip `// ...` lines too.

These four combinators are so fundamental that virtually every real parser written in this style starts here. Once you have them, you can compose them with any other parser and whitespace becomes invisible.

## The Intuition

`ws0` consumes zero or more whitespace characters and returns the trimmed input — it never fails. `ws_wrap(p)` runs `ws0`, then `p`, then `ws0` again, so `ws_wrap(number)` parses `"  42  "` just fine.

```
input: "   hello world"
ws0  → consumes "   " → remaining: "hello world"

input: "( 1 + 2 )"
ws_wrap(tag("+")) → matches "+" even with spaces around it
```

## How It Works in Rust

```rust
// ws0: trim leading whitespace, never fail
fn ws0(input: &str) -> ParseResult<()> {
    let rest = input.trim_start();  // built-in, O(n) over whitespace only
    Ok(((), rest))
}

// ws1: at least one whitespace character required
fn ws1(input: &str) -> ParseResult<()> {
    if input.starts_with(|c: char| c.is_ascii_whitespace()) {
        Ok(((), input.trim_start()))
    } else {
        Err(format!("expected whitespace, got {:?}", &input[..4.min(input.len())]))
    }
}

// ws_wrap: run ws0 → parser → ws0
fn ws_wrap<'a, T>(
    parser: impl Fn(&'a str) -> ParseResult<T>,
) -> impl Fn(&'a str) -> ParseResult<T> {
    move |input| {
        let (_, rest) = ws0(input)?;
        let (value, rest) = parser(rest)?;
        let (_, rest) = ws0(rest)?;
        Ok((value, rest))
    }
}

// line_comment: skip "// ..." or "# ..." to end of line
fn line_comment(input: &str) -> ParseResult<()> {
    // find() scans to the newline — no char-by-char loop needed
    let rest = input.find('\n')
        .map(|i| &input[i + 1..])
        .unwrap_or("");  // comment at EOF → consume everything
    Ok(((), rest))
}
```

## What This Unlocks

- **Token parsers that ignore spacing** — parse `"key = value"` and `"key=value"` with the same parser.
- **Comment-aware parsing** — skip `# config comment` lines before trying to parse a key.
- **Readable grammar rules** — `ws_wrap(keyword("fn"))` reads like the grammar, not like a scanner.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Whitespace skip | `many0 (satisfy is_ws ...)` builds a char list | `trim_start()` — direct slice, zero allocation |
| At least one | `many1 (satisfy is_ws ...)` | Check first char, then `trim_start()` |
| Comment scan | Index arithmetic | `str::find('\n')` |
| Unicode whitespace | Manual char list | `char::is_whitespace()` or `is_ascii_whitespace()` |
