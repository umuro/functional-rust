# Comparison: Example 159 — Map Parser

## map

**OCaml:**
```ocaml
let map (f : 'a -> 'b) (p : 'a parser) : 'b parser = fun input ->
  match p input with
  | Ok (v, rest) -> Ok (f v, rest)
  | Error e -> Error e
```

**Rust:**
```rust
fn map<'a, A: 'a, B: 'a, F>(parser: Parser<'a, A>, f: F) -> Parser<'a, B>
where F: Fn(A) -> B + 'a {
    Box::new(move |input: &'a str| {
        let (value, rest) = parser(input)?;
        Ok((f(value), rest))
    })
}
```

## Practical: parse_nat

**OCaml:**
```ocaml
let parse_nat : int parser =
  map (fun chars ->
    List.fold_left (fun acc c -> acc * 10 + (Char.code c - Char.code '0')) 0 chars
  ) (many1 is_digit)
```

**Rust:**
```rust
fn parse_nat<'a>() -> Parser<'a, u64> {
    map(
        many1(satisfy(|c| c.is_ascii_digit(), "digit")),
        |digits| digits.iter().fold(0u64, |acc, &d| acc * 10 + (d as u64 - '0' as u64)),
    )
}
```

## map_const

**OCaml:**
```ocaml
let map_const (value : 'b) (p : 'a parser) : 'b parser = fun input ->
  match p input with
  | Ok (_, rest) -> Ok (value, rest)
  | Error e -> Error e
```

**Rust:**
```rust
fn map_const<'a, A: 'a, B: Clone + 'a>(parser: Parser<'a, A>, value: B) -> Parser<'a, B> {
    Box::new(move |input: &'a str| {
        let (_, rest) = parser(input)?;
        Ok((value.clone(), rest))
    })
}
```
