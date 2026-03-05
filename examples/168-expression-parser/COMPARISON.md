# Comparison: Example 168 — Expression Parser

## Binding power tables

**OCaml:**
```ocaml
let infix_binding_power = function
  | "+" | "-" -> Some (5, 6)
  | "*" | "/" -> Some (7, 8)
  | "^" -> Some (10, 9)  (* right-associative *)
  | _ -> None
```

**Rust:**
```rust
fn infix_binding_power(op: &str) -> Option<(u8, u8)> {
    match op {
        "+" | "-" => Some((5, 6)),
        "*" | "/" => Some((7, 8)),
        "^" => Some((10, 9)),  // right-associative
        _ => None,
    }
}
```

## Pratt loop

**OCaml:**
```ocaml
and pratt_loop min_bp lhs input =
  match parse_op input with
  | Error _ -> Ok (lhs, input)
  | Ok (op, _) ->
    match infix_binding_power op with
    | Some (lbp, rbp) when lbp >= min_bp ->
      match parse_op input with
      | Ok (_, after_op) ->
        match pratt_expr rbp after_op with
        | Ok (rhs, rem) -> pratt_loop min_bp (BinOp (op, lhs, rhs)) rem
```

**Rust:**
```rust
loop {
    let op = match parse_op(rest) {
        Ok((op, _)) => op.to_string(),
        Err(_) => break,
    };
    let (lbp, rbp) = match infix_binding_power(&op) {
        Some(bp) => bp,
        None => break,
    };
    if lbp < min_bp { break; }
    let (_, after_op) = parse_op(rest)?;
    let (rhs, r) = pratt_expr(after_op, rbp)?;
    lhs = Expr::BinOp(op, Box::new(lhs), Box::new(rhs));
    rest = r;
}
```
