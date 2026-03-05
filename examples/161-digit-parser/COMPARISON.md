# Comparison: Example 161 — Digit Parser

## Single digit

**OCaml:**
```ocaml
let digit : int parser =
  map (fun c -> Char.code c - Char.code '0')
    (satisfy (fun c -> c >= '0' && c <= '9') "digit")
```

**Rust:**
```rust
fn digit<'a>() -> Parser<'a, u32> {
    map(satisfy(|c| c.is_ascii_digit(), "digit"), |c| c as u32 - '0' as u32)
}
```

## Natural number

**OCaml:**
```ocaml
let natural : int parser =
  map (fun digits -> List.fold_left (fun acc d -> acc * 10 + d) 0 digits)
    (many1 digit)
```

**Rust:**
```rust
fn natural<'a>() -> Parser<'a, u64> {
    map(
        many1(satisfy(|c| c.is_ascii_digit(), "digit")),
        |digits| digits.iter().fold(0u64, |acc, &d| acc * 10 + (d as u64 - '0' as u64)),
    )
}
```

## Signed integer

**OCaml:**
```ocaml
let integer : int parser = fun input ->
  match opt (satisfy (fun c -> c = '+' || c = '-') "sign") input with
  | Ok (sign, rest) ->
    (match natural rest with
     | Ok (n, rem) ->
       let value = match sign with Some '-' -> -n | _ -> n in
       Ok (value, rem)
     | Error e -> Error e)
  | Error e -> Error e
```

**Rust:**
```rust
fn integer<'a>() -> Parser<'a, i64> {
    Box::new(|input: &'a str| {
        let (sign, rest) = opt(satisfy(|c| c == '+' || c == '-', "sign"))(input)?;
        let (n, rem) = natural()(rest)?;
        let value = match sign {
            Some('-') => -(n as i64),
            _ => n as i64,
        };
        Ok((value, rem))
    })
}
```
