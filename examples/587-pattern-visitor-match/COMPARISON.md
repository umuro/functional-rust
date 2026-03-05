# OCaml vs Rust: Visitor Pattern via Match

## Expression Type

### OCaml
```ocaml
type expr =
  | Lit of float
  | Add of expr * expr
  | Sub of expr * expr
  | Mul of expr * expr
  | Div of expr * expr
```

### Rust
```rust
enum Expr {
    Lit(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}
```

## Visitors as Functions

### OCaml
```ocaml
let rec eval = function
  | Lit n    -> n
  | Add(l,r) -> eval l +. eval r
  | Sub(l,r) -> eval l -. eval r
  | Mul(l,r) -> eval l *. eval r
  | Div(l,r) -> eval l /. eval r

let rec count_ops = function
  | Lit _    -> 0
  | Add(l,r)|Sub(l,r)|Mul(l,r)|Div(l,r) -> 
      1 + count_ops l + count_ops r
```

### Rust
```rust
fn eval(e: &Expr) -> f64 {
    match e {
        Expr::Lit(n) => *n,
        Expr::Add(l, r) => eval(l) + eval(r),
        Expr::Sub(l, r) => eval(l) - eval(r),
        Expr::Mul(l, r) => eval(l) * eval(r),
        Expr::Div(l, r) => eval(l) / eval(r),
    }
}

fn count_ops(e: &Expr) -> usize {
    match e {
        Expr::Lit(_) => 0,
        Expr::Add(l, r) | Expr::Sub(l, r) | 
        Expr::Mul(l, r) | Expr::Div(l, r) => 
            1 + count_ops(l) + count_ops(r),
    }
}
```

## Key Insight

In functional languages, the visitor pattern is simply "write a recursive function with pattern matching." No interfaces, no accept/visit methods, no double dispatch.

Each "visitor" is just a function that matches on the structure.

## Advantages over OOP Visitor

| Aspect | OOP Visitor | FP Pattern Match |
|--------|-------------|------------------|
| **New operation** | Add visit method to all | Add one function |
| **Boilerplate** | Accept/Visit interfaces | None |
| **Double dispatch** | Required | Not needed |
| **Exhaustiveness** | Manual | Compiler-checked |
| **Code location** | Scattered across classes | Single function |
