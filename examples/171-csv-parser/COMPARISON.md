# Comparison: Example 171 — CSV Parser

## Quoted field

**OCaml:**
```ocaml
let quoted_field : string parser = fun input ->
  match satisfy (fun c -> c = '"') "quote" input with
  | Ok (_, rest) ->
    let buf = Buffer.create 32 in
    let rec go remaining =
      if remaining.[0] = '"' then
        let after = String.sub remaining 1 ... in
        if after.[0] = '"' then (Buffer.add_char buf '"'; go ...)
        else Ok (Buffer.contents buf, after)
      else (Buffer.add_char buf remaining.[0]; go ...)
    in go rest
```

**Rust:**
```rust
fn quoted_field(input: &str) -> ParseResult<String> {
    if !input.starts_with('"') { return Err(...); }
    let mut result = String::new();
    let mut chars = input[1..].chars();
    loop {
        match chars.next() {
            Some('"') => match chars.next() {
                Some('"') => result.push('"'),  // escaped
                _ => return Ok((result, ...)),   // end
            },
            Some(c) => result.push(c),
            None => return Err("Unterminated".into()),
        }
    }
}
```

## Full CSV

**OCaml:**
```ocaml
let csv : string list list parser = fun input ->
  let rec go acc remaining =
    match row remaining with
    | Ok (r, rest) ->
      match line_ending rest with
      | Ok ((), rest') -> go (r :: acc) rest'
  in go [] input
```

**Rust:**
```rust
fn csv(input: &str) -> ParseResult<Vec<Vec<String>>> {
    let mut rows = Vec::new();
    let mut remaining = input;
    while !remaining.is_empty() {
        let (r, rest) = row(remaining)?;
        rows.push(r);
        let ((), rest) = line_ending(rest)?;
        remaining = rest;
    }
    Ok((rows, ""))
}
```
