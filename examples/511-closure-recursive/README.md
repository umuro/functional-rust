# 511: Recursive Closures (Y Combinator)

**Difficulty:** 5  **Level:** Advanced

Closures can't reference themselves by name — here are three techniques to make them recursive anyway, culminating in the Y combinator.

## The Problem This Solves

You write `let fact = |n| if n <= 1 { 1 } else { n * fact(n-1) }` and get: `cannot find value 'fact' in this scope`. The closure is being defined, so it doesn't exist yet. Rust (unlike OCaml's `let rec`) has no keyword for recursive bindings.

For 99% of cases, use a named function — that's the right answer. But there are legitimate scenarios where you need anonymous recursion: implementing interpreters, writing generic fixed-point combinators, or studying lambda calculus in Rust. The Y combinator is also a gateway to understanding how recursion is *derived* from lambda calculus without needing it as a primitive.

The real challenge here isn't just syntax — it's types. A recursive closure's type would be infinite: `T = Fn(T) -> U`. Rust's type system requires all types to have known, finite sizes. The solution: break the infinite type with `Box`.

## The Intuition

The Y combinator `Y(f) = f(Y(f))` is a mathematical trick: it finds the fixed point of `f`. Instead of a closure calling *itself*, it receives a copy of itself as an argument (open recursion), which breaks the self-reference cycle.

Think of it like this: instead of `fact(n)` calling `fact`, you write `fact(self, n)` where `self` is whatever-makes-me-recurse, and then you build the machinery that passes the right thing for `self`.

In OCaml, `let rec fact n = ...` is syntax sugar that the compiler expands using the Y combinator internally. Rust exposes the machinery — which is educational, even if you'd normally use a named function.

## How It Works in Rust

```rust
// Approach 1: Named function — ALWAYS prefer this for real code
fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

// Approach 2: Open recursion — pass self as argument
// The closure takes itself via a dyn Fn reference
fn apply(step: &dyn Fn(&dyn Fn(u64) -> u64, u64) -> u64, n: u64) -> u64 {
    step(&|m| apply(step, m), n)
}
let fact_step = |self_: &dyn Fn(u64) -> u64, n: u64| -> u64 {
    if n <= 1 { 1 } else { n * self_(n - 1) }
};
println!("{}", apply(&fact_step, 6)); // 720

// Approach 3: Y combinator using Box<dyn Fn> to break the type recursion
// Without Box, the type `Y<A,B>` would contain itself — infinite type
struct Y<A, B>(Box<dyn Fn(&Y<A, B>, A) -> B>);
impl<A, B> Y<A, B> {
    fn call(&self, arg: A) -> B { (self.0)(self, arg) }
}

fn y_combinator<A, B, F>(f: F) -> impl Fn(A) -> B
where
    F: Fn(&dyn Fn(A) -> B, A) -> B + 'static,
    A: 'static, B: 'static,
{
    let step = Y(Box::new(move |this: &Y<A, B>, arg: A| f(&|a| this.call(a), arg)));
    move |arg| step.call(arg)
}

let fact = y_combinator(|self_: &dyn Fn(u64) -> u64, n: u64| -> u64 {
    if n <= 1 { 1 } else { n * self_(n - 1) }
});
println!("{}", fact(10)); // 3628800

// Approach 4: Rc<RefCell<Box<dyn Fn>>> — self-referential via shared pointer
let fib: std::rc::Rc<std::cell::RefCell<Box<dyn Fn(u64) -> u64>>> =
    std::rc::Rc::new(std::cell::RefCell::new(Box::new(|_| 0)));
let fib_clone = fib.clone();
*fib.borrow_mut() = Box::new(move |n| {
    if n <= 1 { n } else { fib_clone.borrow()(n-1) + fib_clone.borrow()(n-2) }
});
println!("{}", fib.borrow()(10)); // 55
```

## What This Unlocks

- **Lambda calculus in Rust** — the Y combinator is the proof that recursion is derivable; implement it here to deeply understand Rust's type system.
- **Interpreter patterns** — when building expression evaluators or rule engines, fixed-point combinators allow recursive evaluation rules without named global functions.
- **Type system mastery** — understanding *why* `Box` breaks the infinite type recursion teaches you how Rust's type system handles recursive data structures generally.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Self-referential closure | `let rec f x = f (x-1)` | Not directly — need workaround |
| Open recursion | `let f self x = self self (x-1)` | `\|self_, x\| self_(x-1)` with `dyn Fn` |
| Y combinator | Natural — types allow it directly | Requires `Box<dyn Fn>` to break infinite type |
| Named recursion | `let rec` keyword | Named `fn` — always the practical choice |
| Tail-call optimization | Guaranteed | Not guaranteed — stack overflow risk |
