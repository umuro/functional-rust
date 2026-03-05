# OCaml vs Rust: Pratt Parser

## Binding Power

### Rust
```rust
fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    match op {
        '+' | '-' => Some((1, 2)),   // left associative
        '*' | '/' => Some((3, 4)),   // left associative
        '^' => Some((6, 5)),         // right associative
        _ => None,
    }
}
```

### OCaml
```ocaml
let infix_binding_power = function
  | '+' | '-' -> Some (1, 2)
  | '*' | '/' -> Some (3, 4)
  | '^' -> Some (6, 5)
  | _ -> None
```

## Core Loop

### Rust
```rust
pub fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, String> {
    let mut lhs = self.parse_prefix()?;
    
    loop {
        let (l_bp, r_bp) = match infix_binding_power(op) {
            Some(bp) => bp,
            None => break,
        };
        if l_bp < min_bp { break; }
        
        self.advance();
        let rhs = self.parse_expr(r_bp)?;
        lhs = Expr::Infix { op, left: Box::new(lhs), right: Box::new(rhs) };
    }
    Ok(lhs)
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Binding power | Tuple `int * int` | `(u8, u8)` |
| Recursion | Tail-recursive | Loop + recursion |
| Box | Not needed | Required for AST |
| Option handling | Pattern match | `?` + match |
