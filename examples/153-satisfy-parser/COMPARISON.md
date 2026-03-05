# Comparison: Example 153 — Satisfy Parser

## Core satisfy

**OCaml:**
```ocaml
let satisfy (pred : char -> bool) (desc : string) : char parser = fun input ->
  match advance input with
  | Some (ch, rest) when pred ch -> Ok (ch, rest)
  | Some (ch, _) -> Error (Printf.sprintf "Character '%c' does not satisfy %s" ch desc)
  | None -> Error (Printf.sprintf "Expected %s, got EOF" desc)
```

**Rust:**
```rust
fn satisfy<'a, F>(pred: F, desc: &str) -> Parser<'a, char>
where
    F: Fn(char) -> bool + 'a,
{
    let desc = desc.to_string();
    Box::new(move |input: &'a str| {
        match input.chars().next() {
            Some(c) if pred(c) => Ok((c, &input[c.len_utf8()..])),
            Some(c) => Err(format!("'{}' does not satisfy {}", c, desc)),
            None => Err(format!("Expected {}, got EOF", desc)),
        }
    })
}
```

## Building specific parsers

**OCaml:**
```ocaml
let is_digit = satisfy (fun c -> c >= '0' && c <= '9') "digit"
let is_letter = satisfy (fun c ->
  (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) "letter"
```

**Rust:**
```rust
fn is_digit<'a>() -> Parser<'a, char> {
    satisfy(|c| c.is_ascii_digit(), "digit")
}

fn is_letter<'a>() -> Parser<'a, char> {
    satisfy(|c| c.is_ascii_alphabetic(), "letter")
}
```
