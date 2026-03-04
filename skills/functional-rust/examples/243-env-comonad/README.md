# 243: Env Comonad (CoReader)

**Difficulty:** 5  **Level:** Master

Pair a value with a read-only environment — the exact dual of the Reader monad.

## The Problem This Solves

You're writing an expression evaluator, a configuration-aware computation, or a rendering pipeline where every node needs access to some global context (a variable table, a config struct, a render context). The Reader monad lets you *thread* that environment through function calls. The Env comonad does the *dual thing*: it carries the environment *alongside* a value, and you can run context-sensitive transformations on it.

The classic use case is an expression tree evaluator. Instead of passing a `HashMap<String, i64>` as a parameter through every recursive call, you wrap the expression with the environment: `Env { env: variable_table, value: expression }`. Then `eval` is just a function from `&Env<VarEnv, Expr>` to `i64` — a comonad `extend` operation.

The Env comonad also models the "staged computation" pattern: compute a value in one environment, then re-run it in a different environment using `local`. This is how configuration layers work in real systems.

## The Intuition

A **comonad** is the categorical dual of a monad. Where a monad wraps values and lets you chain computations *into* the context (`bind`), a comonad wraps values and lets you extract them *out of* context (`extract`) and extend computations *over* the context (`extend`).

The **Env comonad** is `Env<E, A> = (E, A)` — just a pair. The environment `E` is the context; `A` is the current value.

- **`extract`**: get the value, ignore the environment. `(env, value) → value`.
- **`ask`**: get the environment, ignore the value. `(env, value) → env`.
- **`extend f`**: apply `f` to the whole pair to produce a new value, keeping the same environment. `(env, f(env, value))`. The environment is *never modified* by extend.
- **`local f`**: run with a *temporarily modified* environment. Like a scoped config override.

The key duality with Reader: `Reader<E, A>` is `E → A` (a function that *consumes* an environment to produce a value). `Env<E, A>` is `(E, A)` (a value that *carries* its environment with it). One computes with the environment; the other is annotated by it.

The three comonad laws are mirror images of monad laws:
1. `extract(extend f x) = f(x)` — extending then extracting gets back the original function result
2. `extend extract = identity` — extending with `extract` does nothing
3. `extend f . extend g = extend (f . extend g)` — associativity of extend

## How It Works in Rust

```rust
pub struct Env<E, A> {
    pub env: E,
    pub value: A,
}

impl<E: Clone, A: Clone> Env<E, A> {
    // The two fundamental operations:
    pub fn extract(&self) -> A { self.value.clone() }
    pub fn ask(&self) -> E    { self.env.clone() }

    // extend: compute a new value using the full (env, value) pair.
    // The environment is passed through unchanged.
    pub fn extend<B: Clone>(&self, f: impl Fn(&Env<E, A>) -> B) -> Env<E, B> {
        Env {
            env: self.env.clone(),    // environment unchanged
            value: f(self),           // new value from the computation
        }
    }

    // local: temporarily modify the environment.
    pub fn local(&self, f: impl Fn(E) -> E) -> Env<E, A> {
        Env { env: f(self.env.clone()), value: self.value.clone() }
    }
}
```

Expression evaluation using `extend`:
```rust
// Wrap expression + variable table in Env
let node = Env::new(variable_table, expression);

// eval is the function we pass to extend:
// it can read both the environment (variable table) and value (expression)
let result = node.extend(eval);  // eval: &Env<VarEnv, Expr> -> i64

// To run in a different environment (e.g., staging vs production):
let staging = node.local(|mut env| { env.insert("x".into(), 99); env });
let staging_result = staging.extend(eval);
```

The expression evaluator recursively builds new `Env` nodes with the same environment, threading it down without explicit parameter passing:
```rust
Expr::Add(l, r) => {
    eval(&Env::new(env_expr.env.clone(), *l.clone()))
    + eval(&Env::new(env_expr.env.clone(), *r.clone()))
}
```

## What This Unlocks

- **Expression evaluators and interpreters**: thread a variable binding context without explicit parameter threading.
- **Configuration layers**: `local` models "run this computation with config overrides" — like a stack-based config system.
- **Context-aware rendering**: annotate each tree node with its rendering context, transform with `extend`, read with `extract`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Env type | `type ('e, 'a) env = { env: 'e; value: 'a }` | `struct Env<E, A> { env: E, value: A }` |
| extract | Pattern match or `.value` field | `self.value.clone()` (Clone bound required) |
| extend | Higher-kinded function via modules/functors | Generic method with `impl Fn` |
| local | Close over env value | Close over env value, same pattern |
| Comonad typeclass | `type class Comonad w` via module signature | Trait, or just implement methods directly |
