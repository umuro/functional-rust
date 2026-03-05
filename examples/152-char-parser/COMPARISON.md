# Comparison: Example 152 — Character Parsers

## char_parser

**OCaml:**
```ocaml
let char_parser (c : char) : char parser = fun input ->
  match advance input with
  | Some (ch, rest) when ch = c -> Ok (ch, rest)
  | Some (ch, _) -> Error (Printf.sprintf "Expected '%c', got '%c'" c ch)
  | None -> Error (Printf.sprintf "Expected '%c', got EOF" c)
```

**Rust:**
```rust
fn char_parser<'a>(expected: char) -> Parser<'a, char> {
    Box::new(move |input: &'a str| {
        match input.chars().next() {
            Some(c) if c == expected => Ok((c, &input[c.len_utf8()..])),
            Some(c) => Err(format!("Expected '{}', got '{}'", expected, c)),
            None => Err(format!("Expected '{}', got EOF", expected)),
        }
    })
}
```

## any_char

**OCaml:**
```ocaml
let any_char : char parser = fun input ->
  match advance input with
  | Some (ch, rest) -> Ok (ch, rest)
  | None -> Error "Expected any character, got EOF"
```

**Rust:**
```rust
fn any_char<'a>() -> Parser<'a, char> {
    Box::new(|input: &'a str| {
        match input.chars().next() {
            Some(c) => Ok((c, &input[c.len_utf8()..])),
            None => Err("Expected any character, got EOF".to_string()),
        }
    })
}
```

## none_of

**OCaml:**
```ocaml
let none_of (chars : char list) : char parser = fun input ->
  match advance input with
  | Some (ch, rest) ->
    if List.mem ch chars then Error (Printf.sprintf "Unexpected '%c'" ch)
    else Ok (ch, rest)
  | None -> Error "Expected a character, got EOF"
```

**Rust:**
```rust
fn none_of<'a>(chars: Vec<char>) -> Parser<'a, char> {
    Box::new(move |input: &'a str| {
        match input.chars().next() {
            Some(c) if !chars.contains(&c) => Ok((c, &input[c.len_utf8()..])),
            Some(c) => Err(format!("Unexpected character '{}'", c)),
            None => Err("Expected a character, got EOF".to_string()),
        }
    })
}
```
