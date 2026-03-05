# Comparison: Example 163 — Whitespace Parser

## ws0

**OCaml:**
```ocaml
let ws0 : unit parser = fun input ->
  match many0 (satisfy is_ws "whitespace") input with
  | Ok (_, rest) -> Ok ((), rest)
  | Error e -> Error e
```

**Rust:**
```rust
fn ws0<'a>() -> Parser<'a, ()> {
    Box::new(|input: &'a str| {
        let trimmed = input.trim_start();
        Ok(((), trimmed))
    })
}
```

## ws_wrap

**OCaml:**
```ocaml
let ws_wrap (p : 'a parser) : 'a parser = fun input ->
  match ws0 input with
  | Ok ((), r1) ->
    (match p r1 with
     | Ok (v, r2) ->
       (match ws0 r2 with
        | Ok ((), r3) -> Ok (v, r3)
        | Error e -> Error e)
     | Error e -> Error e)
  | Error e -> Error e
```

**Rust:**
```rust
fn ws_wrap<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, T> {
    Box::new(move |input: &'a str| {
        let trimmed = input.trim_start();
        let (value, rest) = parser(trimmed)?;
        let trimmed_rest = rest.trim_start();
        Ok((value, trimmed_rest))
    })
}
```
