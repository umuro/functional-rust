# 215: Recursion Schemes — Separating What From How

**Difficulty:** ⭐⭐⭐  **Category:** Recursion Schemes

Stop rewriting the same recursive traversal for every function you add to your data structure.

## The Problem This Solves

You've got an AST. Maybe it's a math expression, a config file, a query language — doesn't matter. You write `eval` to evaluate it. Then you need `show` to display it. Then `depth` to measure it. Then `count_nodes`, then `optimize`, then `collect_free_vars`.

Each function looks exactly like this:

```rust
fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Lit(n)    => *n,
        Expr::Add(a, b) => eval(a) + eval(b),  // recurse left
        Expr::Mul(a, b) => eval(a) * eval(b),  // recurse right
    }
}

fn show(e: &Expr) -> String {
    match e {
        Expr::Lit(n)    => n.to_string(),
        Expr::Add(a, b) => format!("({} + {})", show(a), show(b)),  // same recursion!
        Expr::Mul(a, b) => format!("({} * {})", show(a), show(b)),  // same recursion!
    }
}
```

See the pattern? *Every* function does the same thing: recurse down to leaves, do some work, combine on the way up. The only part that differs is the "do some work" piece. But you're copying the recursion structure every single time.

Add a new node type (`Neg`, `IfZero`, `Let`) and you have to update *every function*. Miss one and the compiler catches you — but only after you've already written the boilerplate for all the others. This is fine for 3 functions. It's painful for 10. It's a maintenance nightmare for 30.

Recursion schemes solve exactly that pain: separate "how to recurse" from "what to do at each node". Write the recursion logic once. Then each new operation is just a small function that handles one layer, with no recursion at all.

## The Intuition

Think about `Iterator`. When you call `.map()`, you don't implement the traversal — you just say "for each element, do this". The `Iterator` machinery handles stepping through elements. You write the logic; the iterator handles the looping.

Recursion schemes do the same thing for recursive data structures. Instead of:
- "recurse left, recurse right, combine" (embedded in every function)

You write:
- A function that handles *just one layer* — children already evaluated
- A *single* generic `cata` function that handles all the recursion

These single-layer functions are called **algebras** (don't let the name scare you — it just means "a function that knows what to do with one level of the structure").

Here's what the algebra for `eval` looks like. Notice: no recursion!

```rust
fn eval_alg(e: ExprF<i64>) -> i64 {
    match e {
        ExprF::LitF(n)    => n,
        ExprF::AddF(a, b) => a + b,  // a and b are already i64s — already evaluated!
        ExprF::MulF(a, b) => a * b,
    }
}
```

The key insight: by the time your algebra sees `AddF(a, b)`, both `a` and `b` are already evaluated results. The recursion happened before you were called. You just combine them.

## How It Works in Rust

**Step 1: Define the non-recursive functor**

Replace your recursive type with a *generic* one. Where you had `Box<Expr>` (a child node), use `A` (a generic placeholder):

```rust
// Before: recursive
enum Expr {
    Lit(i64),
    Add(Box<Expr>, Box<Expr>),
}

// After: non-recursive functor (the children are generic A, not Box<Expr>)
enum ExprF<A> {
    LitF(i64),
    AddF(A, A),   // A is whatever comes back from recursing into children
    MulF(A, A),
}
```

**Step 2: Add `map` — applies a function to all children**

```rust
impl<A> ExprF<A> {
    fn map_ref<B>(&self, f: impl Fn(&A) -> B) -> ExprF<B> {
        match self {
            ExprF::LitF(n)    => ExprF::LitF(*n),     // no children, nothing to map
            ExprF::AddF(a, b) => ExprF::AddF(f(a), f(b)),  // transform both children
            ExprF::MulF(a, b) => ExprF::MulF(f(a), f(b)),
        }
    }
}
```

**Step 3: Tie the recursive knot with `Fix`**

`Fix` wraps the functor into a recursive type — `Fix` contains an `ExprF<Fix>`, which can contain more `Fix` nodes:

```rust
struct Fix(Box<ExprF<Fix>>);

// Build helpers (so you don't write Fix(Box::new(...)) everywhere)
fn lit(n: i64) -> Fix { Fix(Box::new(ExprF::LitF(n))) }
fn add(a: Fix, b: Fix) -> Fix { Fix(Box::new(ExprF::AddF(a, b))) }
```

**Step 4: `cata` — the universal fold**

```rust
fn cata<A>(alg: &dyn Fn(ExprF<A>) -> A, fix: &Fix) -> A {
    // 1. Get the children of this node
    // 2. Recursively evaluate each child (this is the recursion — written once, forever)
    // 3. Pass the node (with evaluated children) to the algebra
    alg(fix.0.map_ref(|child| cata(alg, child)))
}
```

**Step 5: Write algebra functions — no recursion, just one layer**

```rust
fn eval_alg(e: ExprF<i64>) -> i64 {
    match e {
        ExprF::LitF(n)    => n,
        ExprF::AddF(a, b) => a + b,
        ExprF::MulF(a, b) => a * b,
    }
}

fn show_alg(e: ExprF<String>) -> String {
    match e {
        ExprF::LitF(n)    => n.to_string(),
        ExprF::AddF(a, b) => format!("({a} + {b})"),
        ExprF::MulF(a, b) => format!("({a} * {b})"),
    }
}
```

**Using it:**

```rust
let e = add(lit(1), mul(lit(2), lit(3)));
assert_eq!(cata(&eval_alg, &e), 7);          // "(1 + (2 * 3))" evaluated
assert_eq!(cata(&show_alg, &e), "(1 + (2 * 3))");

// Add a new operation — zero recursion boilerplate:
let count_lits = |e: ExprF<usize>| match e {
    ExprF::LitF(_)           => 1,
    ExprF::AddF(a, b) | ExprF::MulF(a, b) => a + b,
};
assert_eq!(cata(&count_lits, &e), 3);  // 3 literals in the expression
```

## What This Unlocks

- **Add operations without touching existing code.** New analysis passes, pretty-printers, optimizers, serializers — each is just a small algebra function. No risk of breaking existing functions.
- **Compose and reuse.** An algebra that returns `Fix` (instead of a base type) is a *tree transformation* — perfect for optimization passes. Pipe them together.
- **Correctness by structure.** Algebras can't accidentally infinite-loop because they contain no recursion. The termination guarantee is in `cata`, written once.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Functor type | `type 'a expr_f = LitF of int \| AddF of 'a * 'a` | `enum ExprF<A> { LitF(i64), AddF(A, A) }` |
| Fix point | `type fix = Fix of fix expr_f` | `struct Fix(Box<ExprF<Fix>>)` — Box required for heap allocation |
| Functor map | Standalone function | Method `map_ref` on `ExprF<A>` |
| Algebra type | `'a expr_f -> 'a` | `ExprF<A> -> A` (same idea) |
| Calling cata | `cata eval_alg expr` | `cata(&eval_alg, &expr)` |
