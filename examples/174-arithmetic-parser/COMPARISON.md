# Comparison: Example 174 — Arithmetic Evaluator

## Additive level

**OCaml:**
```ocaml
and eval_additive input =
  match eval_multiplicative input with
  | Error e -> Error e
  | Ok (lhs, rest) -> eval_additive_loop lhs rest

and eval_additive_loop lhs input =
  let s = ws0 input in
  if String.length s > 0 && s.[0] = '+' then
    match eval_multiplicative (String.sub s 1 ...) with
    | Ok (rhs, rest) -> eval_additive_loop (lhs +. rhs) rest
  else Ok (lhs, s)
```

**Rust:**
```rust
fn eval_additive(input: &str) -> ParseResult<f64> {
    let (mut lhs, mut rest) = eval_multiplicative(input)?;
    loop {
        let s = rest.trim_start();
        if s.starts_with('+') {
            let (rhs, r) = eval_multiplicative(&s[1..])?;
            lhs += rhs; rest = r;
        } else if s.starts_with('-') {
            let (rhs, r) = eval_multiplicative(&s[1..])?;
            lhs -= rhs; rest = r;
        } else { break; }
    }
    Ok((lhs, rest))
}
```

## evaluate wrapper

**OCaml:**
```ocaml
let evaluate expr =
  match eval_expr expr with
  | Ok (v, rest) ->
    if String.length (ws0 rest) = 0 then Ok v
    else Error (Printf.sprintf "Unexpected trailing: \"%s\"" rest)
```

**Rust:**
```rust
fn evaluate(expr: &str) -> Result<f64, String> {
    let (val, rest) = eval_expr(expr)?;
    if rest.trim().is_empty() { Ok(val) }
    else { Err(format!("Unexpected trailing: \"{}\"", rest.trim())) }
}
```
