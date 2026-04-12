📖 **[View on hightechmind.io →](https://hightechmind.io/rust/860-reader-monad)**

---

# Reader Monad
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Functions that need a shared configuration or environment — database connection, logger, feature flags, request context — pass it as the first argument. As this environment grows or more functions need it, threading it explicitly becomes tedious and brittle. The Reader monad encapsulates this: `Reader<R, A>` represents a computation `R -> A` that reads from environment R and produces A. Computations are composed without explicit environment passing — the monad threads it automatically. This is dependency injection made explicit in the type system. It appears in: web request handlers, database query builders, configuration-driven computations, and compiler passes reading symbol tables.

## Learning Outcomes

- Understand `Reader<R, A>` as a wrapper around `Fn(&R) -> A`
- Implement `ask()` which retrieves the full environment
- Implement `local(f, reader)` which runs a computation with a modified environment
- Implement monadic bind: `reader.then(|a| next_reader)` composing environment-reading computations
- Recognize the connection to dependency injection: Reader monad = explicit DI in types

## Rust Application

```rust
pub struct Reader<'a, R, A> {
    run: Box<dyn FnOnce(&R) -> A + 'a>,
}
impl<'a, R: 'a, A: 'a> Reader<'a, R, A> {
    pub fn new(f: impl FnOnce(&R) -> A + 'a) -> Self {
        Reader { run: Box::new(f) }
    }
    pub fn run_reader(self, env: &R) -> A { (self.run)(env) }
    pub fn ask() -> Reader<'a, R, R> where R: Clone {
        Reader::new(|env| env.clone())
    }
    pub fn asks<B>(f: impl Fn(&R) -> B + 'a) -> Reader<'a, R, B> {
        Reader::new(move |env| f(env))
    }
}
```

`ask()` retrieves the entire environment; `asks(f)` extracts a specific field or projection. The `'a` lifetime ties the reader computation to the environment's lifetime, avoiding unnecessary clones. `FnOnce` allows the closure to consume captured values. `run_reader(env)` executes the computation by providing the environment. For multiple readers, bind threads the environment through both computations without explicit passing.

## OCaml Approach

OCaml's Reader: `type ('r, 'a) reader = Reader of ('r -> 'a)`. `run_reader (Reader f) env = f env`. `ask = Reader (fun env -> env)`. `asks f = Reader f`. Monadic bind: `let bind (Reader f) k = Reader (fun env -> let a = f env in let Reader g = k a in g env)`. OCaml's partial application makes `asks` cleaner: `asks = Fun.id |> Reader`. The `local` function: `let local f (Reader g) = Reader (fun env -> g (f env))` runs with a modified environment.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Type | `Box<dyn FnOnce(&R) -> A>` | `Reader of ('r -> 'a)` |
| Environment access | `ask()` returns `R: Clone` | `ask = Reader Fun.id` |
| Field extraction | `asks(|env| env.field)` | `asks = Reader f` |
| Local modification | `local(f, reader)` | `let local f (Reader g)` |
| Lifetime | `'a` ties to env lifetime | No lifetime issues |
| DI framework | Reader monad + trait objects | Reader monad or GADTs |

## Exercises

1. Implement monadic bind for Reader and compose three configuration-reading computations without explicit passing.
2. Use `local` to run a subcomputation with a modified environment: test a function with a mock environment.
3. Implement a database query builder where Reader's environment is the database connection.
4. Compare Reader monad with Rust's conventional approach of passing `&Config` explicitly — show when Reader adds value.
5. Implement `sequence_readers(readers: Vec<Reader<R, A>>) -> Reader<R, Vec<A>>` that combines multiple readers.
