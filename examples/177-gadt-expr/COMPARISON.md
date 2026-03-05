# Comparison: Example 177 — GADT Typed Expression Evaluator

## Type Definition

### OCaml
```ocaml
type _ expr =
  | Lit  : int -> int expr
  | BLit : bool -> bool expr
  | Add  : int expr * int expr -> int expr
  | Eq   : int expr * int expr -> bool expr
  | If   : bool expr * 'a expr * 'a expr -> 'a expr
  | Pair : 'a expr * 'b expr -> ('a * 'b) expr
  | Fst  : ('a * 'b) expr -> 'a expr
```

### Rust
```rust
trait Expr: fmt::Debug {
    type Value;
    fn eval(&self) -> Self::Value;
}

struct Lit(i64);
struct Add<A: Expr<Value = i64>, B: Expr<Value = i64>>(A, B);
struct Eq<A: Expr<Value = i64>, B: Expr<Value = i64>>(A, B);
struct IfExpr<C: Expr<Value = bool>, T: Expr, F: Expr<Value = T::Value>>(C, T, F);
```

## Evaluation

### OCaml
```ocaml
let rec eval : type a. a expr -> a = function
  | Lit n -> n
  | Add (a, b) -> eval a + eval b
  | Eq (a, b) -> eval a = eval b
  | If (c, t, f) -> if eval c then eval t else eval f
  | Pair (a, b) -> (eval a, eval b)
  | Fst p -> fst (eval p)
```

### Rust
```rust
impl Expr for Lit {
    type Value = i64;
    fn eval(&self) -> i64 { self.0 }
}

impl<A: Expr<Value = i64>, B: Expr<Value = i64>> Expr for Add<A, B> {
    type Value = i64;
    fn eval(&self) -> i64 { self.0.eval() + self.1.eval() }
}
```

## Constant Folding

### OCaml
```ocaml
let rec optimize : type a. a expr -> a expr = function
  | Add (Lit 0, b) -> optimize b
  | Mul (Lit 0, _) -> Lit 0
  | e -> e
```

### Rust
```rust
fn optimize(self) -> Self {
    match self {
        IntExpr::Add(a, b) => match (&a.optimize(), &b.optimize()) {
            (IntExpr::Lit(0), _) => b,
            _ => IntExpr::Add(a, b),
        },
        other => other,
    }
}
```
