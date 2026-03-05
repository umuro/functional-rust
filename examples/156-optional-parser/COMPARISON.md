# Comparison: Example 156 — Optional Parser

## opt

**OCaml:**
```ocaml
let opt (p : 'a parser) : 'a option parser = fun input ->
  match p input with
  | Ok (v, rest) -> Ok (Some v, rest)
  | Error _ -> Ok (None, input)
```

**Rust:**
```rust
fn opt<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Option<T>> {
    Box::new(move |input: &'a str| match parser(input) {
        Ok((value, rest)) => Ok((Some(value), rest)),
        Err(_) => Ok((None, input)),
    })
}
```

## with_default

**OCaml:**
```ocaml
let with_default (default : 'a) (p : 'a parser) : 'a parser = fun input ->
  match p input with
  | Ok _ as result -> result
  | Error _ -> Ok (default, input)
```

**Rust:**
```rust
fn with_default<'a, T: Clone + 'a>(default: T, parser: Parser<'a, T>) -> Parser<'a, T> {
    Box::new(move |input: &'a str| match parser(input) {
        Ok(result) => Ok(result),
        Err(_) => Ok((default.clone(), input)),
    })
}
```

## peek

**OCaml:**
```ocaml
let peek (p : 'a parser) : 'a option parser = fun input ->
  match p input with
  | Ok (v, _) -> Ok (Some v, input)  (* don't advance *)
  | Error _ -> Ok (None, input)
```

**Rust:**
```rust
fn peek<'a, T: Clone + 'a>(parser: Parser<'a, T>) -> Parser<'a, Option<T>> {
    Box::new(move |input: &'a str| match parser(input) {
        Ok((value, _)) => Ok((Some(value), input)), // don't advance
        Err(_) => Ok((None, input)),
    })
}
```
