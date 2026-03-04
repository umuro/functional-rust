# 195: Continuation-Passing Style

**Difficulty:** 3  **Level:** Advanced

Transform any function to pass its result to a callback instead of returning — unlocking early exit, async, and stack-safe recursion.

## The Problem This Solves

Normal Rust functions work like this: you call them, they compute something, they return. You wait for the return. Everything stacks up — literally. Each nested call adds a frame to the call stack.

For most code this is fine. But three situations break this model badly:

First: **deep recursion**. A tree with 100,000 levels, a recursive descent parser, a recursive interpreter — these eat stack space call by call until Rust panics with a stack overflow. You could rewrite as a loop with an explicit stack, but that destroys the clarity of the recursive structure.

Second: **early exit across multiple levels**. You want to abort the whole computation when you find a match, not bubble a flag up through every layer. In normal code you end up threading `Option` or `bool` through every function signature — ugly and error-prone.

Third: **async and coroutines**. How does a function "suspend" and "resume"? It has to be able to say "here's what to do with my result when I eventually produce it." That IS a continuation.

CPS solves all three by making "what happens next" a first-class value you can pass, store, modify, and call. This example exists to solve exactly that pain.

## The Intuition

Think of CPS as flipping functions inside out.

Normal function: `compute(inputs) -> output`  
CPS function: `compute(inputs, callback)` — callback receives the output

```rust
// Normal: returns a value
fn add(a: i32, b: i32) -> i32 { a + b }

// CPS: calls callback with the value
fn add_cps<R>(a: i32, b: i32, k: impl Fn(i32) -> R) -> R {
    k(a + b)  // "k" = continuation, the "what to do next"
}

// To use it: pass what you want done with the result
add_cps(3, 4, |sum| println!("Sum is {}", sum));  // prints "Sum is 7"
add_cps(3, 4, |sum| sum * 2);                     // returns 14
```

The magic: the continuation represents *everything that happens after this point*. When you make that explicit, you can:
- Skip it (early exit)
- Store it (suspend)
- Call it with different values (backtrack)

In Rust, you see CPS every day — the `?` operator is syntactic sugar for CPS transformation on `Result`.

## How It Works in Rust

**Basic CPS factorial:**
```rust
use std::rc::Rc;

// Rc<dyn Fn> lets continuations be shared (cloned) when both branches need the same k
fn factorial_cps(n: u64, k: Rc<dyn Fn(u64) -> u64>) -> u64 {
    if n <= 1 {
        k(1)  // base case: give 1 to the continuation
    } else {
        let k2 = k.clone();  // need to share k between the closure and recursive call
        factorial_cps(
            n - 1,
            Rc::new(move |result| k2(n * result))  // new continuation wraps the multiply
        )
    }
}

let id: Rc<dyn Fn(u64) -> u64> = Rc::new(|x| x);  // identity: "just return the result"
assert_eq!(factorial_cps(5, id), 120);
```

**Two continuations = two outcomes (early exit):**
```rust
fn find_cps<T: Copy>(
    pred: &dyn Fn(T) -> bool,
    list: &[T],
    found: &dyn Fn(T) -> Option<T>,    // continuation for success
    not_found: &dyn Fn() -> Option<T>, // continuation for failure
) -> Option<T> {
    if list.is_empty() {
        not_found()                     // exhausted: take the failure path
    } else if pred(list[0]) {
        found(list[0])                  // match: jump directly to success, skip the rest
    } else {
        find_cps(pred, &list[1..], found, not_found)
    }
}

// Find first even number — stops as soon as one is found
let result = find_cps(
    &|x| x % 2 == 0,
    &[1, 3, 4, 5, 8],
    &|x| Some(x),   // found: return it
    &|| None,        // not found: return None
);
assert_eq!(result, Some(4));  // stops at 4, never looks at 5 or 8
```

**Lifting any function into CPS (generic transformer):**
```rust
fn lift_cps<A, B, R>(f: impl Fn(A) -> B, x: A, k: impl Fn(B) -> R) -> R {
    k(f(x))  // compute f(x), pass result to k
}

// Turn any normal function into CPS instantly:
lift_cps(|x: i32| x * x, 7, |r| println!("7² = {}", r));  // prints "7² = 49"
```

**Tail-recursive fold in CPS:**
```rust
fn fold_cps<A: Copy, B: Clone>(
    list: &[A],
    init: B,
    f: &dyn Fn(B, A) -> B,
    k: &dyn Fn(B) -> B,  // the final continuation: what to do with the result
) -> B {
    if list.is_empty() {
        k(init)  // done — call the final continuation
    } else {
        let next = f(init, list[0]);
        fold_cps(&list[1..], next, f, k)  // tail position: no stack frame kept
    }
}
```

## What This Unlocks

- **Async/await explained**: `async fn` desugars to a state machine where each `.await` point is a continuation. Understanding CPS makes the desugaring obvious — and demystifies `Future`.
- **Custom control flow**: With CPS you can implement your own `?`-like operators, early exit from nested loops, or backtracking search — without language-level support.
- **Compiler passes**: CPS is the standard intermediate representation in optimizing compilers. GHC (Haskell), MLton (SML), and others compile to CPS as an intermediate step because it makes all control flow explicit.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Continuation type | `'a -> 'b` (any function) | `Rc<dyn Fn(A) -> B>` or `impl Fn` |
| Sharing continuations | Automatic (GC) | Needs `Rc` for multiple references |
| Stack safety with CPS | Yes (TCO guaranteed) | No — needs trampoline on top |
| The `?` operator | `result >>= k` (bind) | CPS in disguise |
| Recommended for prod | Yes (elegant in OCaml) | Use iterators; CPS = educational |
| Each continuation | Stack-allocated | Heap-allocated `Box` or `Rc` |
