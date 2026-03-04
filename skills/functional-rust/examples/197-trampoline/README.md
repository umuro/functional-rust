# 197: Trampoline — Heap-Based Tail Call Optimization

**Difficulty:** 3  **Level:** Advanced

Convert deep recursion into a loop by returning "do more work" instead of calling recursively — eliminating stack overflow forever.

## The Problem This Solves

You write a beautiful recursive function. Clean, clear, elegant. Then you run it with a large input and Rust kills your program with a stack overflow. The default stack is 8MB. A recursive function with 100,000 levels uses 100,000 stack frames. Math says that won't fit.

The obvious fix: rewrite as a loop. But for mutual recursion — where function A calls B, and B calls A — that rewrite can be genuinely painful. You end up simulating a call stack manually, which is exactly what you were trying to avoid.

The standard advice for OCaml and Haskell is "make it tail-recursive and rely on TCO (Tail Call Optimization)." The compiler eliminates the stack frame for tail calls. But Rust does **not** guarantee TCO. Even a perfect tail-recursive function in Rust can overflow the stack.

The trampoline pattern solves this without changing your logic. Instead of calling the next function directly, you *return* it — wrapped as a closure. A tiny outer loop runs the closures one at a time, staying on constant stack space. The "pending work" lives on the heap (in closures), not the stack. Zero stack overflow, no matter the depth. This is exactly how Rust's async runtime works internally. This example exists to solve exactly that pain.

## The Intuition

Imagine a trampoline as a machine that bounces you up, one hop at a time.

Each "hop" is: "do a little work, then come back down." The machine (loop) keeps bouncing until you say "I'm done."

In code: instead of a function calling another function (which grows the stack), the function *returns* one of two things:
- `Done(value)` — finished, here's the answer
- `More(closure)` — not done yet, run this closure next

```rust
enum Step<A> {
    Done(A),            // computation finished, here's the value
    More(Box<dyn FnOnce() -> Step<A>>),  // do more work next iteration
}

fn run<A>(mut step: Step<A>) -> A {
    loop {  // O(1) stack — same frame every iteration
        match step {
            Step::Done(x) => return x,   // finished
            Step::More(f) => step = f(), // do next hop, stay in loop
        }
    }
}
```

Compare: without trampoline, `is_even(1_000_000)` would need 1,000,000 stack frames. With trampoline, it uses exactly 1 stack frame forever.

## How It Works in Rust

**Mutual recursion — classic stack overflow case:**
```rust
// Without trampoline: even(1_000_000) → odd(999_999) → even(999_998) → ...
// Stack overflow around depth 10_000-100_000 depending on frame size

// With trampoline: each "call" returns a Step instead of recursing
fn is_even_t(n: u64) -> Step<bool> {
    if n == 0 {
        Step::done(true)               // base case: done
    } else {
        Step::more(move || is_odd_t(n - 1))  // defer: "call is_odd next"
    }
}

fn is_odd_t(n: u64) -> Step<bool> {
    if n == 0 {
        Step::done(false)
    } else {
        Step::more(move || is_even_t(n - 1))
    }
}

// The trampoline runs it — no stack growth:
let result = run(is_even_t(1_000_000));  // works! returns true
```

**Factorial with accumulator (tail-recursive trampoline):**
```rust
fn factorial_t(n: u64) -> u64 {
    fn go(n: u64, acc: u64) -> Step<u64> {
        if n <= 1 {
            Step::done(acc)                            // done: return accumulated result
        } else {
            Step::more(move || go(n - 1, n * acc))    // next hop: multiply and continue
        }
    }
    run(go(n, 1))
}

assert_eq!(factorial_t(20), 2_432_902_008_176_640_000);
```

**Fibonacci with CPS to avoid two recursive calls:**
```rust
// Problem: fib needs TWO recursive calls. Trampoline handles one at a time.
// Solution: CPS inside the trampoline — continuation holds "what to do with result A
//           before starting on result B"
fn fibonacci_t(n: u64) -> u64 {
    fn go(n: u64, k: Box<dyn FnOnce(u64) -> Step<u64>>) -> Step<u64> {
        if n <= 1 {
            k(n)  // base: give n to the continuation
        } else {
            Step::more(move || {
                go(n - 1, Box::new(move |a| {          // first: compute fib(n-1) → a
                    go(n - 2, Box::new(move |b| k(a + b)))  // then: compute fib(n-2) → b
                }))
            })
        }
    }
    run(go(n, Box::new(Step::done)))
}
```

**The pattern that appears in async runtimes:**
```
async fn foo() { ... }
// Desugars to something like:
// Poll::Pending → More(waker + closure)
// Poll::Ready(x) → Done(x)
// The executor is the trampoline loop.
```

## What This Unlocks

- **Unlimited recursion depth**: Any recursive algorithm — tree traversal, interpreters, state machines, parsers — can be made stack-safe with this pattern. No algorithm rewrite required.
- **Async runtime comprehension**: Rust's `Future` is a trampoline. `Poll::Pending` = `More`, `Poll::Ready` = `Done`. The executor IS the `run` loop. This clicks once you see trampoline.
- **Mutual recursion between modules**: State machines with many states that call each other freely, without worrying about stack depth. Common in protocol parsers and game AI.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Stack safety for tail calls | Free (compiler TCO) | Must implement manually via trampoline |
| Trampoline needed? | Rarely | Yes, for deep recursion |
| Closure in `More` | GC-allocated, cheap | `Box<dyn FnOnce()>` heap allocation |
| Async/await | Not built-in | Based on this exact pattern |
| Stack size limit | ~1MB default, runtime tunable | 8MB default, thread spawn for more |
| Mutual recursion | Naturally tail-recursive | Needs trampoline |
