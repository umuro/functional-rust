# Comparison: Example 154 — String Parser

## tag (exact match)

**OCaml:**
```ocaml
let tag (expected : string) : string parser = fun input ->
  let len = String.length expected in
  if String.length input >= len && String.sub input 0 len = expected then
    Ok (expected, String.sub input len (String.length input - len))
  else
    Error (Printf.sprintf "Expected \"%s\"" expected)
```

**Rust:**
```rust
fn tag<'a>(expected: &str) -> Parser<'a, &'a str> {
    let expected_owned = expected.to_string();
    Box::new(move |input: &'a str| {
        if input.starts_with(&expected_owned) {
            let rest = &input[expected_owned.len()..];
            Ok((&input[..expected_owned.len()], rest))
        } else {
            Err(format!("Expected \"{}\"", expected_owned))
        }
    })
}
```

## tag_no_case

**OCaml:**
```ocaml
let tag_no_case (expected : string) : string parser = fun input ->
  let len = String.length expected in
  if String.length input >= len &&
     String.lowercase_ascii (String.sub input 0 len) = String.lowercase_ascii expected then
    Ok (String.sub input 0 len, String.sub input len (String.length input - len))
  else
    Error (Printf.sprintf "Expected \"%s\" (case insensitive)" expected)
```

**Rust:**
```rust
fn tag_no_case<'a>(expected: &str) -> Parser<'a, &'a str> {
    let expected_lower = expected.to_lowercase();
    let len = expected.len();
    Box::new(move |input: &'a str| {
        if input.len() >= len && input[..len].to_lowercase() == expected_lower {
            Ok((&input[..len], &input[len..]))
        } else {
            Err(format!("Expected \"{}\" (case insensitive)", expected_lower))
        }
    })
}
```
