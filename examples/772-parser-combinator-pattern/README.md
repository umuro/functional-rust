📖 **[View on hightechmind.io →](https://hightechmind.io/rust/772-parser-combinator-pattern)**

---

# 772: Parser Combinator Pattern (nom-Style)

**Difficulty:** 4  **Level:** Advanced

Build a composable parsing library from scratch: parsers as functions, combinators as higher-order functions, no external dependencies.

## The Problem This Solves

Writing parsers by hand with index tracking and ad-hoc string splitting doesn't compose. Every new format requires new special-case code, and testing is difficult because the parser is entangled with the format it handles.

Parser combinators solve this by making parsers *first-class values*. Each primitive (`tag`, `digits`, `alpha`) is a function that takes an input slice and returns the remaining input plus a parsed value. Combinators (`pair`, `map`, `many0`) take parsers and return new parsers. Complex grammars are built by composing small, independently testable pieces — the same model that `nom` uses, but expressed here in vanilla Rust without macros.

This is how you understand what `nom` and `winnow` are actually doing under the hood.

## The Intuition

A parser is a function: `&str → Result<(&str, T), Error>`. On success it returns `(remaining_input, parsed_value)` — the input it didn't consume, plus what it found. On failure it returns an error.

A combinator is a function that takes parsers and returns a new parser. `pair(p1, p2)` runs `p1` on the input, then runs `p2` on the remaining input, and returns both results. `map(p, f)` runs `p` and transforms the result. `many0(p)` runs `p` in a loop until it fails, collecting results into a `Vec`.

The key insight: every combinator returns `impl Fn(Input) -> PResult<T>` — a parser, just like its inputs. They compose arbitrarily.

## How It Works in Rust

**Type aliases** — the shape of a parser:
```rust
pub type Input<'a>      = &'a str;
pub type PResult<'a, T> = Result<(Input<'a>, T), String>;
```

**Primitives** — return closures (parsers):
```rust
pub fn tag<'a>(prefix: &'static str) -> impl Fn(Input<'a>) -> PResult<'a, &'a str> {
    move |s| {
        if s.starts_with(prefix) { Ok((&s[prefix.len()..], &s[..prefix.len()])) }
        else { Err(format!("expected {prefix:?}")) }
    }
}

pub fn take_while<F: Fn(char) -> bool>(pred: F) -> impl Fn(Input<'_>) -> PResult<'_, &str> {
    move |s| {
        let end = s.find(|c| !pred(c)).unwrap_or(s.len());
        Ok((&s[end..], &s[..end]))
    }
}
```

**Combinators** — compose parsers:
```rust
pub fn pair<'a, A, B, P1, P2>(p1: P1, p2: P2) -> impl Fn(Input<'a>) -> PResult<'a, (A, B)>
where P1: Fn(Input<'a>) -> PResult<'a, A>, P2: Fn(Input<'a>) -> PResult<'a, B> {
    move |s| {
        let (rest, a) = p1(s)?;
        let (rest, b) = p2(rest)?;
        Ok((rest, (a, b)))
    }
}

pub fn many0<'a, T, P: Fn(Input<'a>) -> PResult<'a, T>>(p: P)
    -> impl Fn(Input<'a>) -> PResult<'a, Vec<T>>
{
    move |mut s| {
        let mut acc = Vec::new();
        loop {
            match p(s) {
                Ok((rest, v)) => { acc.push(v); s = rest; }
                Err(_) => break,
            }
        }
        Ok((s, acc))
    }
}
```

**Building a real parser from combinators:**
```rust
fn key_value(s: Input<'_>) -> PResult<'_, (&str, &str)> {
    pair(
        terminated(alpha, char_p('=')),   // key before '='
        take_while(|c| c != ',' && c != '\n'),  // value until delimiter
    )(s)
}

fn kv_list(s: Input<'_>) -> PResult<'_, Vec<(&str, &str)>> {
    sep_by(key_value, char_p(','))(s)
}

// Usage:
let (rest, pairs) = kv_list("name=Alice,age=30,city=Berlin").unwrap();
```

## What This Unlocks

- **Understanding `nom`/`winnow`** — these crates follow exactly this pattern, extended with error recovery and binary input support.
- **Composable grammar building** — add a new rule by composing existing parsers, not by modifying existing code.
- **Testable parsing** — each primitive and combinator is independently testable with simple string inputs.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Parser type | `string -> ('a * string)` or `angstrom` | `Fn(&str) -> Result<(&str, T), E>` |
| Higher-order combinators | `let ( >>= ) p f s = ...` | `fn pair<P1,P2>(p1: P1, p2: P2) -> impl Fn(...)` |
| Many/repeat | `many : 'a t -> 'a list t` | `many0(p)` returning `impl Fn → Vec<T>` |
| Returning closures | `fun s -> ...` | `move |s| ...` returned as `impl Fn` |
