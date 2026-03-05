# Comparison: Example 162 — Identifier Parser

## Allocating identifier

**OCaml:**
```ocaml
let identifier : string parser = fun input ->
  match satisfy is_ident_start "letter or _" input with
  | Error e -> Error e
  | Ok (first, rest) ->
    match many0 (satisfy is_ident_char "alphanumeric or _") rest with
    | Ok (chars, rem) ->
      let s = String.make 1 first ^ String.init (List.length chars) (List.nth chars) in
      Ok (s, rem)
    | Error e -> Error e
```

**Rust:**
```rust
fn identifier<'a>() -> Parser<'a, String> {
    Box::new(|input: &'a str| {
        let start = satisfy(|c| c.is_ascii_alphabetic() || c == '_', "letter or _");
        let (first, rest) = start(input)?;
        let cont = many0(satisfy(|c| c.is_ascii_alphanumeric() || c == '_', "ident char"));
        let (chars, rem) = cont(rest)?;
        let mut s = String::with_capacity(1 + chars.len());
        s.push(first);
        for c in chars { s.push(c); }
        Ok((s, rem))
    })
}
```

## Zero-copy (Rust only)

```rust
fn identifier_slice<'a>() -> Parser<'a, &'a str> {
    Box::new(|input: &'a str| {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                let mut end = c.len_utf8();
                for ch in chars {
                    if ch.is_ascii_alphanumeric() || ch == '_' { end += ch.len_utf8(); }
                    else { break; }
                }
                Ok((&input[..end], &input[end..]))
            }
            _ => Err("Expected identifier".to_string()),
        }
    })
}
```
