📖 **[View on hightechmind.io →](https://hightechmind.io/rust/165-keyword-parser)**

---

# 165: Keyword Parser

**Difficulty:** 3  **Level:** Advanced

Parse reserved words with word boundary checking — so `"if"` matches in `"if x"` but not in `"iffy"`.

## The Problem This Solves

Every language has keywords: `if`, `else`, `fn`, `return`, `let`. They look just like identifiers — but they're reserved. A naive approach parses `"if"` with `tag("if")`, which matches correctly in `"if x = 1"`. But it also matches the start of `"iffy"`, treating the variable `iffy` as a keyword followed by garbage.

The fix is word boundary detection: after matching the keyword string, check that the *next* character is not a letter, digit, or underscore. If it is, the match is inside a longer identifier and should fail.

Add token mapping (parse `"if"` → `Token::If`) and multi-keyword choice (try `"else if"` before `"else"`) and you have the complete keyword layer that sits between your scanner and your grammar.

## The Intuition

A keyword parser = `tag(keyword_str)` + boundary check. The boundary check peeks at the next character: if it could continue an identifier, reject the match.

```
input: "if x > 0"
tag("if") → ok, remaining: " x > 0"
next char: ' ' — not an ident char → keyword match ✓

input: "iffy_name"
tag("if") → ok, remaining: "fy_name"  
next char: 'f' — IS an ident char → reject ✗
```

## How It Works in Rust

```rust
#[derive(Clone, Debug, PartialEq)]
enum Token { If, Else, Let, Fn, Return }

fn keyword<'a>(word: &'static str, tok: Token) -> impl Fn(&'a str) -> ParseResult<Token> {
    move |input| {
        if !input.starts_with(word) {
            return Err(format!("expected '{}'", word));
        }
        let rest = &input[word.len()..];

        // Word boundary: next char must NOT be an identifier character
        if rest.starts_with(|c: char| c.is_alphanumeric() || c == '_') {
            return Err(format!("'{}' is a prefix, not a keyword", word));
        }

        Ok((tok.clone(), rest))
    }
}

// Try multiple keywords, longest first to avoid prefix ambiguity
fn any_keyword(input: &str) -> ParseResult<Token> {
    // Sort by length descending: try "else if" before "else"
    let mut candidates = vec![
        ("if",     Token::If),
        ("else",   Token::Else),
        ("let",    Token::Let),
        ("fn",     Token::Fn),
        ("return", Token::Return),
    ];
    candidates.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    for (word, tok) in candidates {
        if let Ok(result) = keyword(word, tok)(input) {
            return Ok(result);
        }
    }
    Err("no keyword matched".to_string())
}
```

## What This Unlocks

- **Token classification** — turn raw strings into typed `Token` values for your grammar layer.
- **Identifier vs keyword** — parse `let` as a keyword, `lethal` as an identifier.
- **Extensible keyword tables** — add a keyword by adding one entry to the candidates list.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Token type | `type token = If \| Then \| Else` | `enum Token { If, Then, Else }` |
| Clone for reuse | Automatic (structural equality) | `#[derive(Clone)]` required |
| Boundary check | `String.get rest 0` | `rest.chars().next()` |
| Longest-first sort | `List.sort (fun a b -> compare (len b) (len a))` | `Vec::sort_by(&#124;a, b&#124; b.0.len().cmp(&a.0.len()))` |
