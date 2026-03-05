# Comparison: Example 172 — INI Parser

## Section header

**OCaml:**
```ocaml
let parse_section_header input =
  let s = ws0 input in
  if s.[0] = '[' then
    match String.index_opt s ']' with
    | Some i ->
      let name = String.trim (String.sub s 1 (i - 1)) in
      Ok (name, skip_line (String.sub s (i+1) ...))
```

**Rust:**
```rust
fn parse_section_header(input: &str) -> ParseResult<String> {
    let s = input.trim_start();
    if !s.starts_with('[') { return Err(...); }
    match s.find(']') {
        Some(i) => {
            let name = s[1..i].trim().to_string();
            Ok((name, skip_line(&s[i + 1..])))
        }
        None => Err("Expected ']'".into()),
    }
}
```

## Key-value entry

**OCaml:**
```ocaml
let parse_entry input =
  match String.index_opt s '=' with
  | Some i ->
    let key = String.trim (String.sub s 0 i) in
    let value = String.trim (String.sub rest_line 0 value_end) in
    Ok ((key, value), remaining)
```

**Rust:**
```rust
fn parse_entry(input: &str) -> ParseResult<(String, String)> {
    let line = &s[..line_end];
    match line.find('=') {
        Some(eq_pos) => {
            let key = line[..eq_pos].trim().to_string();
            let mut value = line[eq_pos + 1..].trim().to_string();
            Ok(((key, value), rest))
        }
    }
}
```
