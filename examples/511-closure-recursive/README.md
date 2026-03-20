📖 **[View on hightechmind.io →](https://hightechmind.io/rust/511-closure-recursive)**

---

# Recursive Closures and Y Combinator

Rust closures cannot reference themselves directly, so recursive anonymous computations require either named inner functions, open recursion (passing self as argument), or the Y combinator pattern using a `struct` wrapper.

## Problem Statement

In lambda calculus, the Y combinator `Y(f) = f(Y(f))` enables recursion without named functions — it passes the function to itself as an argument. Rust closures are anonymous and cannot capture themselves (they don't exist yet when their body is evaluated). The workarounds are: (1) a named `fn` (simplest), (2) open recursion — a `step(self_, n)` function where `self_` is the recursive delegate, (3) a `struct Y(Box<dyn Fn(&Y, A) -> B>)` that passes itself explicitly. These are not just curiosities — they illustrate how recursive computation works at the type level.

## Learning Outcomes

- Understand why closures cannot be self-referential in Rust
- Implement open recursion via a higher-order `step` function
- Build a `Y<A, B>` combinator struct with `call(&self, arg) -> B`
- Implement `y_factorial` and `fib_open` using the Y combinator
- Recognise that named functions are always the practical choice for real code

## Rust Application

`Y` struct combinator:

```rust
pub struct Y<A, B>(pub Box<dyn Fn(&Y<A, B>, A) -> B>);

impl<A, B> Y<A, B> {
    pub fn call(&self, arg: A) -> B { (self.0)(self, arg) }
}

pub fn y_factorial() -> Y<u64, u64> {
    Y(Box::new(|y, n| if n <= 1 { 1 } else { n * y.call(n - 1) }))
}
```

Open recursion via `factorial_open`:

```rust
pub fn factorial_open<F>(step: F, n: u64) -> u64
where F: Fn(&dyn Fn(u64) -> u64, u64) -> u64 { ... }

factorial_open(|self_, n| if n <= 1 { 1 } else { n * self_(n-1) }, 10)
```

## OCaml Approach

OCaml's `let rec` makes recursive closures trivial:

```ocaml
let rec factorial n = if n <= 1 then 1 else n * factorial (n-1)

(* Y combinator — possible but not idiomatic *)
let y f = (fun x -> f (fun v -> x x v)) (fun x -> f (fun v -> x x v))
let factorial = y (fun self_ n -> if n <= 1 then 1 else n * self_ (n-1))
```

OCaml's `let rec` is the normal way; the Y combinator is a theoretical exercise or used when recursion must be abstracted.

## Key Differences

1. **`let rec` vs. named `fn`**: OCaml's `let rec` makes anonymous recursive closures natural; Rust requires named `fn` for the recursive case.
2. **Y combinator cost**: Rust's `Y` struct uses `Box<dyn Fn>` — heap allocation and vtable dispatch per call. OCaml's Y combinator similarly uses indirect dispatch.
3. **Open recursion pattern**: Both Rust and OCaml can express open recursion (pass `self_` as an argument); OCaml's is syntactically lighter.
4. **Practical recommendation**: In both languages, named recursive functions are the right choice for production code; the Y combinator is for learning and theory.

## Exercises

1. **Fibonacci with Y**: Implement Fibonacci via the `Y` combinator and verify `y_fib.call(10) == 55`.
2. **Trampolining**: Implement a `Trampoline<T>` enum (`Done(T) | More(Box<dyn FnOnce() -> Trampoline<T>>)`) to make the Y-combinator stack-safe for large inputs.
3. **Memoised Y**: Extend the Y combinator to wrap the recursive step with a `HashMap` cache — implement `memo_y_factorial` that uses memoization inside the Y combinator.
