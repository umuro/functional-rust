# Comparison: Example 158 — Sequence Parser

## pair

**OCaml:**
```ocaml
let pair (p1 : 'a parser) (p2 : 'b parser) : ('a * 'b) parser = fun input ->
  match p1 input with
  | Error e -> Error e
  | Ok (v1, rest) ->
    match p2 rest with
    | Error e -> Error e
    | Ok (v2, remaining) -> Ok ((v1, v2), remaining)
```

**Rust:**
```rust
fn pair<'a, A: 'a, B: 'a>(p1: Parser<'a, A>, p2: Parser<'a, B>) -> Parser<'a, (A, B)> {
    Box::new(move |input: &'a str| {
        let (v1, rest) = p1(input)?;
        let (v2, remaining) = p2(rest)?;
        Ok(((v1, v2), remaining))
    })
}
```

## preceded / terminated

**OCaml:**
```ocaml
let preceded (prefix : 'a parser) (p : 'b parser) : 'b parser = fun input ->
  match prefix input with
  | Error e -> Error e
  | Ok (_, rest) -> p rest
```

**Rust:**
```rust
fn preceded<'a, A: 'a, B: 'a>(prefix: Parser<'a, A>, p: Parser<'a, B>) -> Parser<'a, B> {
    Box::new(move |input: &'a str| {
        let (_, rest) = prefix(input)?;
        p(rest)
    })
}
```

## delimited

**OCaml:**
```ocaml
let delimited open_p p close_p = fun input ->
  preceded open_p (terminated p close_p) input
```

**Rust:**
```rust
fn delimited<'a, A: 'a, B: 'a, C: 'a>(
    open: Parser<'a, A>, p: Parser<'a, B>, close: Parser<'a, C>,
) -> Parser<'a, B> {
    Box::new(move |input: &'a str| {
        let (_, r1) = open(input)?;
        let (value, r2) = p(r1)?;
        let (_, r3) = close(r2)?;
        Ok((value, r3))
    })
}
```
