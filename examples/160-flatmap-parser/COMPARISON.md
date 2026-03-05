# Comparison: Example 160 — FlatMap Parser

## and_then / bind

**OCaml:**
```ocaml
let and_then (p : 'a parser) (f : 'a -> 'b parser) : 'b parser = fun input ->
  match p input with
  | Error e -> Error e
  | Ok (v, rest) -> (f v) rest

let ( >>= ) = and_then
```

**Rust:**
```rust
fn and_then<'a, A: 'a, B: 'a, F>(parser: Parser<'a, A>, f: F) -> Parser<'a, B>
where F: Fn(A) -> Parser<'a, B> + 'a {
    Box::new(move |input: &'a str| {
        let (value, rest) = parser(input)?;
        (f(value))(rest)
    })
}
```

## Length-prefixed string

**OCaml:**
```ocaml
let length_prefixed : string parser =
  parse_nat >>= fun n ->
  (satisfy (fun c -> c = ':') "colon") >>= fun _ ->
  (fun input ->
    if String.length input >= n then
      Ok (String.sub input 0 n, String.sub input n (String.length input - n))
    else Error "Not enough characters")
```

**Rust:**
```rust
fn length_prefixed<'a>() -> Parser<'a, &'a str> {
    and_then(parse_nat(), |n| {
        Box::new(move |input: &'a str| {
            if input.starts_with(':') {
                let rest = &input[1..];
                if rest.len() >= n {
                    Ok((&rest[..n], &rest[n..]))
                } else {
                    Err("Not enough characters".to_string())
                }
            } else {
                Err("Expected ':'".to_string())
            }
        })
    })
}
```
