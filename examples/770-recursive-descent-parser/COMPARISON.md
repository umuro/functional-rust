# OCaml vs Rust: Recursive Descent Parser

## Grammar Implementation

### Rust
```rust
/// expr = term (('+' | '-') term)*
pub fn parse_expr(&mut self) -> Result<Expr, String> {
    let mut left = self.parse_term()?;
    loop {
        match &self.current {
            Token::Plus => {
                self.advance();
                let right = self.parse_term()?;
                left = Expr::BinOp { op: '+', left: Box::new(left), right: Box::new(right) };
            }
            _ => break,
        }
    }
    Ok(left)
}
```

### OCaml
```ocaml
(* expr = term (('+' | '-') term)* *)
let rec parse_expr lexer =
  let left = parse_term lexer in
  match peek lexer with
  | Plus ->
      advance lexer;
      let right = parse_term lexer in
      BinOp { op = '+'; left; right }
  | _ -> left
```

## AST Definition

### Rust
```rust
pub enum Expr {
    Number(f64),
    BinOp { op: char, left: Box<Expr>, right: Box<Expr> },
    UnaryMinus(Box<Expr>),
}
```

### OCaml
```ocaml
type expr =
  | Number of float
  | BinOp of { op: char; left: expr; right: expr }
  | UnaryMinus of expr
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Recursion | Natural | Same |
| Box for recursion | Not needed (GC) | Required |
| Error handling | Exception or result | `Result<T, E>` |
| Pattern match | `match` | `match` |
