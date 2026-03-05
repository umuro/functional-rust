# Comparison: Example 157 — Choice Parser

## alt

**OCaml:**
```ocaml
let alt (p1 : 'a parser) (p2 : 'a parser) : 'a parser = fun input ->
  match p1 input with
  | Ok _ as result -> result
  | Error _ -> p2 input
```

**Rust:**
```rust
fn alt<'a, T: 'a>(p1: Parser<'a, T>, p2: Parser<'a, T>) -> Parser<'a, T> {
    Box::new(move |input: &'a str| match p1(input) {
        Ok(result) => Ok(result),
        Err(_) => p2(input),
    })
}
```

## choice

**OCaml:**
```ocaml
let choice (parsers : 'a parser list) : 'a parser = fun input ->
  let rec try_parsers = function
    | [] -> Error "No parser matched"
    | p :: rest ->
      match p input with
      | Ok _ as result -> result
      | Error _ -> try_parsers rest
  in
  try_parsers parsers
```

**Rust:**
```rust
fn choice<'a, T: 'a>(parsers: Vec<Parser<'a, T>>) -> Parser<'a, T> {
    Box::new(move |input: &'a str| {
        for parser in &parsers {
            if let Ok(result) = parser(input) {
                return Ok(result);
            }
        }
        Err("No parser matched".to_string())
    })
}
```
