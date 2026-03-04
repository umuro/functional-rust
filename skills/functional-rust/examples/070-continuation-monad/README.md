# 070: Continuation Monad

**Difficulty:** 3  **Level:** Advanced

Encode computations as callbacks — the monad that can express every other monad.

## The Problem This Solves

Normal functions return values. Continuation-passing style (CPS) inverts this: instead of returning a value, a function *receives a callback* and passes its result to the callback. This sounds backwards, but it unlocks control flow that normal functions can't express.

CPS lets you pause a computation, save it (the "continuation" is the saved rest-of-the-computation), and resume it later — sometimes in a different order, sometimes multiple times, sometimes not at all. This is the foundation of: async/await, generators, exceptions, coroutines, backtracking search, and early exit patterns.

The continuation monad wraps this pattern in a composable interface. It's called "the mother of all monads" because every other monad (Result, Option, State, IO) can be derived from it.

## The Intuition

A continuation is "everything that remains to be done." If you're computing `2 + 3 * 4`, and you're in the middle of evaluating `3 * 4`, the continuation is "add 2 to whatever comes out, then return the result."

CPS makes continuations explicit. Instead of:
```
let x = add(2, 3)
let y = mul(x, 4)
```
You write:
```
add_cps(2, 3, |x| mul_cps(x, 4, |y| return y))
```

The callback (`|x| ...`) is the continuation. The function calls the callback when it has its answer. The entire call stack is encoded in nested closures.

## How It Works in Rust

```rust
// CPS addition: compute a+b, pass result to k
let add_cps = |a: i32, b: i32, k: &dyn Fn(i32) -> i32| k(a + b);
let result = add_cps(2, 3, &|x| x * 10);  // pass result to k — gives 50

// CPS factorial: recurse with an explicit continuation
fn factorial_cps(n: i32, k: &dyn Fn(i32) -> i32) -> i32 {
    if n <= 1 {
        k(1)                                          // base case: pass 1 to k
    } else {
        factorial_cps(n - 1, &|r| k(n * r))          // continuation captures current n
    }
}
factorial_cps(5, &|x| x)  // → 120

// The Cont<R, A> monad wraps: (A -> R) -> R
pub struct Cont<R, A> {
    run: Box<dyn Fn(Box<dyn Fn(A) -> R>) -> R>,
}

impl<R: 'static, A: 'static> Cont<R, A> {
    pub fn run_cont(self, k: impl Fn(A) -> R + 'static) -> R {
        (self.run)(Box::new(k))  // supply the final continuation
    }
}
```

`&dyn Fn` is essential: continuations are passed through recursive calls, and the concrete closure type changes at each level. Dynamic dispatch handles the heterogeneous chain of closures.

## What This Unlocks

- **Understanding async/await** — Rust's `async fn` compiles to a state machine that encodes the continuation explicitly. CPS is the mental model.
- **Deriving other monads** — `Option`, `Result`, and `State` can all be encoded as specialisations of `Cont`. Understanding this reveals the deep structure of monadic programming.
- **Trampoline-style recursion** — CPS factorial can be made fully iterative (trampolined), eliminating stack overflow for deeply recursive computations.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Continuation type | `('a -> 'r) -> 'r` | `Box<dyn Fn(Box<dyn Fn(A) -> R>) -> R>` |
| Closure capture | Automatic under GC | `move` + lifetime annotations (`'static`) |
| `dyn Fn` necessity | Not required | Required for heterogeneous callback chains |
| Tail-call CPS | TCO makes CPS stack-safe | No TCO — CPS factorial still builds stack |
| Monad syntax | `let*` (bind) operator | Manual `.and_then()` / `.run_cont()` chaining |
