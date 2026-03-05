# OCaml vs Rust: Visitor Pattern via Fold

## Side-by-Side Code

### OCaml
```ocaml
type expr =
  | Lit of float
  | Add of expr * expr
  | Mul of expr * expr
  | Neg of expr

let rec fold ~lit ~add ~mul ~neg = function
  | Lit x -> lit x
  | Add (a, b) -> add (fold ~lit ~add ~mul ~neg a) (fold ~lit ~add ~mul ~neg b)
  | Mul (a, b) -> mul (fold ~lit ~add ~mul ~neg a) (fold ~lit ~add ~mul ~neg b)
  | Neg a -> neg (fold ~lit ~add ~mul ~neg a)

let eval = fold ~lit:Fun.id ~add:(+.) ~mul:( *.) ~neg:(fun x -> -.x)
```

### Rust (idiomatic)
```rust
pub fn fold<R>(
    expr: &Expr,
    lit: &dyn Fn(f64) -> R,
    add: &dyn Fn(R, R) -> R,
    mul: &dyn Fn(R, R) -> R,
    neg: &dyn Fn(R) -> R,
) -> R {
    match expr {
        Expr::Lit(x) => lit(*x),
        Expr::Add(a, b) => add(fold(a, lit, add, mul, neg), fold(b, lit, add, mul, neg)),
        Expr::Mul(a, b) => mul(fold(a, lit, add, mul, neg), fold(b, lit, add, mul, neg)),
        Expr::Neg(a) => neg(fold(a, lit, add, mul, neg)),
    }
}

pub fn eval(expr: &Expr) -> f64 {
    fold(expr, &|x| x, &|a, b| a + b, &|a, b| a * b, &|x| -x)
}
```

### Rust (trait-based visitor)
```rust
pub trait ExprVisitor<R> {
    fn visit_lit(&self, x: f64) -> R;
    fn visit_add(&self, a: R, b: R) -> R;
    fn visit_mul(&self, a: R, b: R) -> R;
    fn visit_neg(&self, a: R) -> R;
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Expression type | `type expr = Lit of float \| ...` | `enum Expr { Lit(f64), ... }` |
| Fold signature | `val fold : lit:... -> add:... -> expr -> 'b` | `fn fold<R>(expr: &Expr, lit: &dyn Fn, ...) -> R` |
| Recursive boxing | Implicit (GC-managed) | `Box<Expr>` (explicit heap) |
| Labeled args | `~lit ~add ~mul ~neg` | Positional parameters |

## Key Insights

1. **Fold IS the visitor pattern** — OCaml developers never think "visitor pattern" because fold naturally provides the same capability. One function per variant, passed as closures.
2. **Labeled arguments vs positional** — OCaml's `~lit ~add ~mul ~neg` is self-documenting. Rust's positional `&dyn Fn` parameters require careful ordering or helper type aliases.
3. **Box<Expr> is the price of no-GC** — OCaml's recursive types just work. Rust needs `Box` to put recursive variants on the heap, adding syntactic noise.
4. **Both share the core insight** — data structure + multiple interpretations = fold with different closures. Neither OOP inheritance nor trait hierarchies needed.
5. **Performance trade-off** — Rust's `&dyn Fn` has dynamic dispatch overhead similar to OCaml's closures. For hot paths, Rust could use generics with `impl Fn` for monomorphization.

## When to Use Each Style

**Use fold (closure-based) when:** You want maximum flexibility — new interpretations are just new sets of closures. Perfect for interpreters, pretty-printers, optimizers.
**Use trait-based visitor when:** You want named, reusable visitor implementations that can carry state and be tested independently.
