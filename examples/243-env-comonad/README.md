[![Functional Rust](https://img.shields.io/badge/functional--rust-examples-blue)](https://hightechmind.io)

# Environment Comonad

## Problem Statement

The Environment comonad (also called the Reader comonad's dual, or Co-Reader) pairs a value with a fixed environment: `(e, a)`. You can always extract the value, ask what the environment is, and extend computations that depend on both the current value and the surrounding environment. This models read-only context propagation where each step in a computation can inspect a shared environment without passing it explicitly.

## Learning Outcomes

- Understand Env as the categorical dual of the Reader monad
- Implement `extract`, `extend`, and `duplicate` for the Env comonad
- See how `ask` retrieves the environment and `local` modifies it
- Compare the Env comonad with OCaml's equivalent using tuples and modules
- Recognize when Env comonad is more natural than threading an explicit config argument

## Rust Application

The Env comonad in Rust is a straightforward product type:

```rust
#[derive(Clone, Debug)]
struct Env<E, A> {
    env: E,
    val: A,
}

impl<E: Clone, A: Clone> Env<E, A> {
    fn new(env: E, val: A) -> Self { Env { env, val } }

    // Comonad: extract — return the wrapped value
    fn extract(&self) -> A { self.val.clone() }

    // Ask — retrieve the environment
    fn ask(&self) -> E { self.env.clone() }

    // Extend — run a context-aware computation, keep the same environment
    fn extend<B>(&self, f: impl Fn(&Env<E, A>) -> B) -> Env<E, B> {
        Env {
            env: self.env.clone(),
            val: f(self),
        }
    }

    // Duplicate — Env e a -> Env e (Env e a)
    fn duplicate(&self) -> Env<E, Env<E, A>> {
        Env {
            env: self.env.clone(),
            val: self.clone(),
        }
    }

    // Local — run a computation with a modified environment
    fn local<E2: Clone>(&self, f: impl Fn(E) -> E2) -> Env<E2, A> {
        Env {
            env: f(self.env.clone()),
            val: self.val.clone(),
        }
    }
}

// Practical use: scoring values in context
#[derive(Clone, Debug)]
struct Config {
    multiplier: f64,
    offset: f64,
}

fn score(ctx: &Env<Config, f64>) -> f64 {
    ctx.val * ctx.env.multiplier + ctx.env.offset
}

fn grade(ctx: &Env<Config, f64>) -> &'static str {
    match score(ctx) {
        s if s >= 90.0 => "A",
        s if s >= 75.0 => "B",
        s if s >= 60.0 => "C",
        _ => "F",
    }
}

fn main() {
    let cfg = Config { multiplier: 1.5, offset: 10.0 };
    let raw_score = Env::new(cfg.clone(), 55.0);

    let scored = raw_score.extend(score);
    println!("Score: {}", scored.extract());  // 55.0 * 1.5 + 10.0 = 92.5

    let graded = raw_score.extend(grade);
    println!("Grade: {}", graded.extract());  // "A"

    // Modify environment locally
    let strict_cfg = raw_score.local(|c| Config { multiplier: c.multiplier, offset: 0.0 });
    let strict_graded = strict_cfg.extend(grade);
    println!("Strict grade: {}", strict_graded.extract()); // 55.0*1.5 = 82.5 → "B"
}
```

Each call to `extend` threads the same environment through a new computation layer without any explicit parameter passing.

## OCaml Approach

OCaml represents Env as a plain pair:

```ocaml
type ('e, 'a) env = { env: 'e; val_: 'a }

let extract { val_; _ } = val_
let ask { env; _ } = env

let extend f w = { env = w.env; val_ = f w }
let duplicate w = { env = w.env; val_ = w }
let local f w = { env = f w.env; val_ = w.val_ }
```

OCaml's implicit polymorphism avoids the `Clone` constraints Rust requires. The tradeoff is that OCaml's type inference handles the variance silently while Rust makes it explicit.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Environment sharing | `E: Clone` on each operation | implicit GC sharing |
| Local transformation | returns `Env<E2, A>` (new type) | returns same `env` type |
| Duplicate | requires `A: Clone` | structural copy |
| Composition | `.extend(f).extend(g)` chains | same, pipe-friendly |
| Adjunction | dual to `State` monad | same theoretical dual |

The Env comonad is adjoint to the State monad: Env extracts a value from context while State injects a state into continuation. Together they form the `(- × e) ⊣ (e →)` adjunction.

## Exercises

1. Implement `fmap` for `Env<E, A>` that maps the value while keeping the environment unchanged.
2. Verify the comonad laws: `extend extract = id`, `extract . extend f = f`, `extend f . extend g = extend (f . extend g)`.
3. Build a dependency injection container using `Env<AppConfig, Service>` where each `extend` step configures a new service layer.
4. Implement `asks`: given a function `E -> B`, produce `Env<E, B>` — the comonadic equivalent of Reader's `asks`.
5. Show the adjunction between Env and State by implementing `unit` and `counit` of the adjunction and checking the triangle identities.
