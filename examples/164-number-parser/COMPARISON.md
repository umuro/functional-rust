# Comparison: Example 164 — Number Parser

## Imperative scanner

**OCaml:**
```ocaml
let float_string : string parser = fun input ->
  let buf = Buffer.create 16 in
  let pos = ref 0 in
  let len = String.length input in
  if !pos < len && (input.[!pos] = '+' || input.[!pos] = '-') then begin
    Buffer.add_char buf input.[!pos]; incr pos end;
  while !pos < len && is_digit input.[!pos] do
    Buffer.add_char buf input.[!pos]; incr pos done;
  (* ... decimal, exponent ... *)
  Ok (Buffer.contents buf, String.sub input !pos (len - !pos))
```

**Rust:**
```rust
fn float_string<'a>() -> Parser<'a, &'a str> {
    Box::new(|input: &'a str| {
        let bytes = input.as_bytes();
        let mut pos = 0;
        if pos < bytes.len() && (bytes[pos] == b'+' || bytes[pos] == b'-') { pos += 1; }
        while pos < bytes.len() && bytes[pos].is_ascii_digit() { pos += 1; }
        // ... decimal, exponent ...
        Ok((&input[..pos], &input[pos..]))
    })
}
```

## String to float conversion

**OCaml:**
```ocaml
float_of_string "3.14"  (* 3.14 *)
```

**Rust:**
```rust
"3.14".parse::<f64>()  // Ok(3.14)
```
