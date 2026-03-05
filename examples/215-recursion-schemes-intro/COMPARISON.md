# OCaml vs Rust: Recursion Schemes — Separating What From How

## Side-by-Side Code

### OCaml (direct recursion — the problem)
```ocaml
let rec eval = function
  | Lit n       -> n
  | Add (a, b)  -> eval a + eval b
  | Mul (a, b)  -> eval a * eval b

let rec show = function
  | Lit n       -> string_of_int n
  | Add (a, b)  -> "(" ^ show a ^ " + " ^ show b ^ ")"
  | Mul (a, b)  -> "(" ^ show a ^ " * " ^ show b ^ ")"
```

### OCaml (recursion scheme — the solution)
```ocaml
(* Base functor: recursive positions become type variable 'a *)
type 'a expr_f =
  | LitF of int
  | AddF of 'a * 'a
  | MulF of 'a * 'a

let fmap f = function
  | LitF n      -> LitF n
  | AddF (a, b) -> AddF (f a, f b)
  | MulF (a, b) -> MulF (f a, f b)

(* project: peel one layer *)
let project = function
  | Lit n      -> LitF n
  | Add (a, b) -> AddF (a, b)
  | Mul (a, b) -> MulF (a, b)

(* cata: the ONE place recursion lives *)
let rec cata alg e = alg (fmap (cata alg) (project e))

(* algebras: zero recursion *)
let eval_alg = function LitF n -> n | AddF (a,b) -> a+b | MulF (a,b) -> a*b
let eval e = cata eval_alg e

let show_alg = function
  | LitF n      -> string_of_int n
  | AddF (a, b) -> "(" ^ a ^ " + " ^ b ^ ")"
  | MulF (a, b) -> "(" ^ a ^ " * " ^ b ^ ")"
let show e = cata show_alg e
```

### Rust (idiomatic — direct recursion)
```rust
pub fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Lit(n)    => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

pub fn show(e: &Expr) -> String {
    match e {
        Expr::Lit(n)    => n.to_string(),
        Expr::Add(a, b) => format!("({} + {})", show(a), show(b)),
        Expr::Mul(a, b) => format!("({} * {})", show(a), show(b)),
    }
}
```

### Rust (recursion scheme — catamorphism)
```rust
// Base functor: A replaces every recursive Expr position
pub enum ExprF<A> { Lit(i64), Add(A, A), Mul(A, A) }

impl<A> ExprF<A> {
    pub fn map<B, F: Fn(A) -> B>(self, f: F) -> ExprF<B> {
        match self {
            ExprF::Lit(n)    => ExprF::Lit(n),
            ExprF::Add(a, b) => ExprF::Add(f(a), f(b)),
            ExprF::Mul(a, b) => ExprF::Mul(f(a), f(b)),
        }
    }
}

fn project(e: Expr) -> ExprF<Box<Expr>> { /* strip one layer */ }

// The ONE place recursion lives
pub fn cata<A, F: Fn(ExprF<A>) -> A>(e: Expr, alg: &F) -> A {
    alg(project(e).map(|child| cata(*child, alg)))
}

// Algebras: pure logic, no recursion
pub fn eval_cata(e: Expr) -> i64 {
    cata(e, &|node| match node {
        ExprF::Lit(n)    => n,
        ExprF::Add(a, b) => a + b,
        ExprF::Mul(a, b) => a * b,
    })
}

pub fn show_cata(e: Expr) -> String {
    cata(e, &|node| match node {
        ExprF::Lit(n)    => n.to_string(),
        ExprF::Add(a, b) => format!("({a} + {b})"),
        ExprF::Mul(a, b) => format!("({a} * {b})"),
    })
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Base functor | `type 'a expr_f = LitF of int \| AddF of 'a * 'a \| MulF of 'a * 'a` | `enum ExprF<A> { Lit(i64), Add(A, A), Mul(A, A) }` |
| Functor map | `val fmap : ('a -> 'b) -> 'a expr_f -> 'b expr_f` | `fn map<B, F: Fn(A) -> B>(self, f: F) -> ExprF<B>` |
| Catamorphism | `val cata : ('a expr_f -> 'a) -> expr -> 'a` | `fn cata<A, F: Fn(ExprF<A>) -> A>(e: Expr, alg: &F) -> A` |
| Algebra | `'a expr_f -> 'a` | `Fn(ExprF<A>) -> A` |
| Direct eval | `expr -> int` | `fn eval(e: &Expr) -> i64` |
| Cata eval | `expr -> int` | `fn eval_cata(e: Expr) -> i64` |

## Key Insights

1. **The problem is duplicated recursion.** Every function that processes a tree (`eval`, `show`, `depth`, `count_nodes`) rewrites the same `match` arms with recursive calls. The recursion pattern never changes — only the leaf/combine logic does. This is the boilerplate recursion schemes eliminate.

2. **The base functor is the key abstraction.** `ExprF<A>` is `Expr` with every recursive `Expr` replaced by a type variable `A`. In OCaml this is `type 'a expr_f`; in Rust it's `enum ExprF<A>`. The type parameter is the "hole" where the result of recursive calls slots in.

3. **`map` makes it a functor.** Implementing `fmap`/`map` for `ExprF` lets the catamorphism apply any function to the recursive positions without caring about tree structure. In OCaml, `fmap` is a standalone function; in Rust, it's a method on `ExprF<A>`.

4. **Ownership shapes the API.** OCaml's GC means `cata` can pass values freely. In Rust, `cata` *consumes* the `Expr` (owned value) to avoid reference-counting overhead. The direct approach uses `&Expr` (borrowed) since it doesn't need to restructure the tree — a natural split the type system enforces.

5. **Closures as algebras.** In OCaml, algebras are ordinary functions passed to `cata`. In Rust, they're closures `|node: ExprF<A>| -> A`, passed as `&F` where `F: Fn(ExprF<A>) -> A`. Rust's monomorphisation means this compiles to the same machine code as writing the recursion by hand — zero abstraction cost.

6. **Extensibility gain.** Once `cata` exists, every new traversal costs exactly one algebra function — no match arms, no `Box`, no recursion. Adding `count_nodes` took three lines. Adding a new `Expr` variant (e.g., `Neg`) requires updating `ExprF`, `project`, and the `map` — but then every existing algebra fails to compile until updated, giving compile-time exhaustiveness checking across all consumers at once.

## When to Use Each Style

**Use direct recursion when:** the data structure is small and stable (2–3 variants, unlikely to grow), or you are writing a one-off transformation and the indirection of a functor layer adds more complexity than it removes.

**Use catamorphism / recursion schemes when:** you have multiple traversals over the same type, the type is growing (new variants expected), or you want to guarantee that the recursion logic is correct *once* and reuse it everywhere without risk of per-function bugs.
