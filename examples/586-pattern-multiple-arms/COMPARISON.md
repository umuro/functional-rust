# OCaml vs Rust: Multiple Arms Pattern Matching

## Consolidating Variants

### OCaml
```ocaml
type token = Plus|Minus|Star|Slash|Eq|Ne|Lt|Le|Gt|Ge|...

let token_type = function
  | Plus|Minus|Star|Slash -> "arithmetic"
  | Eq|Ne|Lt|Le|Gt|Ge     -> "comparison"
  | LParen|RParen         -> "bracket"
  | Num _                 -> "number"
  | Ident _               -> "identifier"
```

### Rust
```rust
fn token_type(t: &Token) -> &'static str {
    match t {
        Token::Plus | Token::Minus | Token::Star | Token::Slash => "arithmetic",
        Token::Eq | Token::Ne | Token::Lt | Token::Le | Token::Gt | Token::Ge => "comparison",
        Token::LParen | Token::RParen => "bracket",
        Token::Num(_) => "number",
        Token::Ident(_) => "identifier",
    }
}
```

## Range Patterns

### OCaml
```ocaml
(* OCaml doesn't have range patterns for integers *)
let status_category code =
  if code >= 100 && code <= 199 then "informational"
  else if code >= 200 && code <= 299 then "success"
  (* ... *)
```

### Rust
```rust
fn status_category(code: u16) -> &'static str {
    match code {
        100..=199 => "informational",
        200..=299 => "success",
        300..=399 => "redirection",
        400..=499 => "client error",
        500..=599 => "server error",
        _ => "unknown",
    }
}
```

## matches! Macro for Boolean

### Rust
```rust
fn is_operator(t: &Token) -> bool {
    matches!(t, Token::Plus | Token::Minus | Token::Star | ...)
}

fn is_success(code: u16) -> bool {
    matches!(code, 200..=299)
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **OR patterns** | `P1 \| P2` | `P1 \| P2` |
| **Range patterns** | Not supported | `start..=end` |
| **matches! macro** | No equivalent | Boolean pattern test |
| **Char ranges** | `'a'..'z'` (in guards) | `'a'..='z'` |
