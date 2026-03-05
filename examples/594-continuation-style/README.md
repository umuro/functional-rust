📖 **[View on hightechmind.io →](https://hightechmind.io/rust/594-continuation-style)**

---

# 594: Continuation-Passing Style (CPS)

**Difficulty:** 4  **Level:** Advanced

Instead of returning a value, pass a callback to receive it — making control flow explicit and enabling trampolining.

## The Problem This Solves

Normal functions *return* values. The caller decides what to do next. In deeply recursive code this means the call stack encodes the "what to do next" information — and the stack is finite. Stack overflows are the failure mode.

Continuation-passing style (CPS) makes the *continuation* — "what happens next" — an explicit argument to every function. Instead of `fn factorial(n) -> u64`, you write `fn fact_k(n, k: impl FnOnce(u64))` where `k` is called with the result instead of returning it. The function never returns a meaningful value — it always calls its continuation.

CPS is how compilers model control flow internally (every intermediate language is in CPS). It's how async/await works: the continuation is the code after the `.await`. It enables trampolining (example 595), structured error handling without exceptions, and coroutines.

## The Intuition

You're ordering at a restaurant. Normally you say "give me the soup" and wait for it. CPS is like saying "here's my phone number — call me when the soup is ready, I'll be working on other things." The kitchen (the function) doesn't return soup; it calls you back with soup. The entire control flow is explicit in who calls whom.

## How It Works in Rust

1. **CPS factorial** — the continuation `k` receives the result:
   ```rust
   fn fact_k<R>(n: u64, k: impl FnOnce(u64) -> R) -> R {
       if n <= 1 { k(1) }
       else { fact_k(n - 1, move |r| k(n * r)) }
   }

   fact_k(10, |n| println!("10! = {}", n));
   ```
2. **The continuation captures context** — `move |r| k(n * r)` captures both `n` and the outer continuation `k`. The chain of closures represents the remaining computation.
3. **CPS for error handling** — two continuations: success and failure:
   ```rust
   fn safe_div_k<R>(a: f64, b: f64,
       ok: impl FnOnce(f64) -> R,
       err: impl FnOnce(&str) -> R) -> R {
       if b == 0.0 { err("division by zero") } else { ok(a / b) }
   }
   ```
4. **Boxed continuations** for dynamic dispatch (needed for mutual recursion):
   ```rust
   fn fib_k<R: 'static>(n: u64, k: Box<dyn FnOnce(u64) -> R>) -> R {
       if n <= 1 { k(n) }
       else {
           fib_k(n - 1, Box::new(move |r1| {
               fib_k(n - 2, Box::new(move |r2| k(r1 + r2)))
           }))
       }
   }
   ```
5. **Limitation** — naive CPS still builds a closure chain on the heap proportional to recursion depth. To truly eliminate stack overflow, combine with trampolining (example 595).

## What This Unlocks

- **Explicit control flow** — every possible outcome is visible in the function signature; no hidden exceptions.
- **Composable pipelines** — chaining continuations is function composition with data flowing forward.
- **Foundation for trampolining** — returning a thunk from a continuation instead of calling it immediately gives you the trampoline pattern.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| CPS function | `let fact_k n k = if n<=1 then k 1 else fact_k (n-1) (fun r -> k (n*r))` | Same structure; `impl FnOnce` or `Box<dyn FnOnce>` |
| Tail calls | Optimised by compiler (TCO) | Not guaranteed; use trampoline |
| Heap allocation | GC-managed closures | `Box<dyn FnOnce>` for dynamic continuation chains |
| Async equivalent | `lwt` callbacks | `async/await` (compiler-generated CPS) |
