# Comparison: Example 176 — Introduction to GADTs

## GADT Type Definition

### OCaml
```ocaml
type _ expr =
  | Int  : int  -> int expr
  | Bool : bool -> bool expr
  | Add  : int expr * int expr -> int expr
  | If   : bool expr * 'a expr * 'a expr -> 'a expr
```

### Rust
```rust
use std::marker::PhantomData;

enum ExprInner {
    Int(i64),
    Bool(bool),
    Add(Box<ExprInner>, Box<ExprInner>),
    If(Box<ExprInner>, Box<ExprInner>, Box<ExprInner>),
}

struct Expr<T> {
    inner: ExprInner,
    _phantom: PhantomData<T>,
}
```

## Type-Safe Evaluation

### OCaml
```ocaml
let rec eval : type a. a expr -> a = function
  | Int n -> n
  | Bool b -> b
  | Add (a, b) -> eval a + eval b
  | If (cond, t, f) -> if eval cond then eval t else eval f
```

### Rust
```rust
impl Expr<i64> {
    fn eval(&self) -> i64 {
        match &self.inner {
            ExprInner::Int(n) => *n,
            ExprInner::Add(a, b) => { /* reconstruct typed wrappers */ }
            _ => unreachable!(),
        }
    }
}

impl Expr<bool> {
    fn eval(&self) -> bool {
        match &self.inner {
            ExprInner::Bool(b) => *b,
            _ => unreachable!(),
        }
    }
}
```

## Trait-Based Alternative (Rust)

### Rust
```rust
trait Eval {
    type Output;
    fn eval(&self) -> Self::Output;
}

struct IntLit(i64);
impl Eval for IntLit {
    type Output = i64;
    fn eval(&self) -> i64 { self.0 }
}

struct AddExpr(Box<dyn Eval<Output = i64>>, Box<dyn Eval<Output = i64>>);
impl Eval for AddExpr {
    type Output = i64;
    fn eval(&self) -> i64 { self.0.eval() + self.1.eval() }
}
```

## Heterogeneous Lists

### OCaml
```ocaml
type _ hlist =
  | HNil  : unit hlist
  | HCons : 'a * 'b hlist -> ('a * 'b) hlist

let hd : type a b. (a * b) hlist -> a = function
  | HCons (x, _) -> x
```

### Rust
```rust
trait HList {}
impl HList for () {}
impl<H, T: HList> HList for (H, T) {}

trait Head {
    type Item;
    fn head(&self) -> &Self::Item;
}

impl<H, T: HList> Head for (H, T) {
    type Item = H;
    fn head(&self) -> &H { &self.0 }
}

// Usage: (42, ("hello", (true, ())))
```
