# Comparison: Example 155 — Many Parser

## many0

**OCaml:**
```ocaml
let many0 (p : 'a parser) : 'a list parser = fun input ->
  let rec go acc remaining =
    match p remaining with
    | Ok (v, rest) -> go (v :: acc) rest
    | Error _ -> Ok (List.rev acc, remaining)
  in
  go [] input
```

**Rust:**
```rust
fn many0<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Box::new(move |mut input: &'a str| {
        let mut results = Vec::new();
        while let Ok((value, rest)) = parser(input) {
            results.push(value);
            input = rest;
        }
        Ok((results, input))
    })
}
```

## many1

**OCaml:**
```ocaml
let many1 (p : 'a parser) : 'a list parser = fun input ->
  match p input with
  | Error e -> Error e
  | Ok (first, rest) ->
    match many0 p rest with
    | Ok (others, remaining) -> Ok (first :: others, remaining)
    | Error e -> Error e
```

**Rust:**
```rust
fn many1<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Box::new(move |input: &'a str| {
        let (first, mut remaining) = parser(input)?;
        let mut results = vec![first];
        while let Ok((value, rest)) = parser(remaining) {
            results.push(value);
            remaining = rest;
        }
        Ok((results, remaining))
    })
}
```

## many_till

**OCaml:**
```ocaml
let many_till (p : 'a parser) (stop : 'b parser) : ('a list * 'b) parser = fun input ->
  let rec go acc remaining =
    match stop remaining with
    | Ok (term, rest) -> Ok ((List.rev acc, term), rest)
    | Error _ ->
      match p remaining with
      | Ok (v, rest) -> go (v :: acc) rest
      | Error e -> Error e
  in
  go [] input
```

**Rust:**
```rust
fn many_till<'a, T: 'a, U: 'a>(
    parser: Parser<'a, T>,
    stop: Parser<'a, U>,
) -> Parser<'a, (Vec<T>, U)> {
    Box::new(move |mut input: &'a str| {
        let mut results = Vec::new();
        loop {
            if let Ok((term, rest)) = stop(input) {
                return Ok(((results, term), rest));
            }
            let (value, rest) = parser(input)?;
            results.push(value);
            input = rest;
        }
    })
}
```
