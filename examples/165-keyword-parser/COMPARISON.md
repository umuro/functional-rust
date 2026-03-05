# Comparison: Example 165 — Keyword Parser

## keyword with boundary

**OCaml:**
```ocaml
let keyword (kw : string) : string parser = fun input ->
  match tag kw input with
  | Error e -> Error e
  | Ok (matched, rest) ->
    if String.length rest > 0 && is_ident_char rest.[0] then
      Error (Printf.sprintf "\"%s\" is not a complete keyword" kw)
    else Ok (matched, rest)
```

**Rust:**
```rust
fn keyword<'a>(kw: &str) -> Parser<'a, &'a str> {
    let kw_owned = kw.to_string();
    Box::new(move |input: &'a str| {
        if !input.starts_with(&kw_owned) {
            return Err(format!("Expected \"{}\"", kw_owned));
        }
        let rest = &input[kw_owned.len()..];
        match rest.chars().next() {
            Some(c) if is_ident_char(c) => Err(format!("not a complete keyword")),
            _ => Ok((&input[..kw_owned.len()], rest)),
        }
    })
}
```

## Token enum

**OCaml:**
```ocaml
type token = If | Then | Else | Let | In | Fun
```

**Rust:**
```rust
#[derive(Debug, Clone, PartialEq)]
enum Token { If, Then, Else, Let, In, Fn }
```
