📖 **[View on hightechmind.io →](https://hightechmind.io/rust/595-trampoline-pattern)**

---

# 595: Trampoline Pattern

**Difficulty:** 4  **Level:** Advanced

Turn deep recursion into a loop by returning thunks instead of recursing — stack overflow prevention without TCO.

## The Problem This Solves

Rust does not guarantee tail-call optimisation. A recursive function that calls itself in tail position still grows the stack. Count down from a million and you'll hit a stack overflow — even if the logic is perfectly tail-recursive.

Functional languages like OCaml and Haskell eliminate this with TCO or lazy evaluation. Rust makes you explicit about it. The trampoline pattern is the idiomatic solution: instead of calling the next step of the recursion, *return a description of that step* (a thunk — a zero-argument closure). A driver loop repeatedly calls thunks until it gets a final value. The stack stays constant-depth; the heap holds the pending computation.

## The Intuition

A ball on a trampoline: it hits the surface and bounces up, then comes back down. It never builds up — it just keeps bouncing at the same height. The driver loop is the trampoline surface. Each recursive "call" is a bounce: you land, get sent back up (execute a thunk), land again, until you're caught (the `Done` case).

## How It Works in Rust

1. **Define the `Bounce` type** — a sum type of "finished" or "more work":
   ```rust
   enum Bounce<T> {
       Done(T),
       More(Box<dyn FnOnce() -> Bounce<T>>),
   }
   ```
2. **Driver loop** — iteratively calls thunks, never grows the stack:
   ```rust
   fn run<T>(mut b: Bounce<T>) -> T {
       loop {
           match b {
               Bounce::Done(v)  => return v,
               Bounce::More(th) => b = th(),
           }
       }
   }
   ```
3. **Stack-safe factorial** — return a thunk instead of recursing:
   ```rust
   fn fact_t(n: u64, acc: u64) -> Bounce<u64> {
       if n == 0 { Bounce::Done(acc) }
       else      { Bounce::More(Box::new(move || fact_t(n - 1, n * acc))) }
   }

   let result = run(fact_t(1_000_000, 1)); // no stack overflow
   ```
4. **Mutually recursive functions** — each calls the other by returning `More`:
   ```rust
   fn even_t(n: u64) -> Bounce<bool> {
       if n == 0 { Bounce::Done(true) }
       else      { Bounce::More(Box::new(move || odd_t(n - 1))) }
   }
   fn odd_t(n: u64) -> Bounce<bool> {
       if n == 0 { Bounce::Done(false) }
       else      { Bounce::More(Box::new(move || even_t(n - 1))) }
   }
   ```
5. **Cost** — each step allocates a `Box`. For performance-critical paths, consider `stacker` (runtime stack growth) or iterative reformulation.

## What This Unlocks

- **Arbitrary recursion depth** — count to a billion without stack overflow; only heap is consumed.
- **Mutually recursive state machines** — the natural encoding of protocols, parsers, and coroutines.
- **Understand async** — async state machines are the compiler-generated trampoline for `Future` chains.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Tail recursion | TCO guaranteed | Not guaranteed → use trampoline |
| Thunk type | `unit -> 'a` (lazy) | `Box<dyn FnOnce() -> Bounce<T>>` |
| Driver loop | Handled by runtime | Explicit `run()` loop |
| Allocation | GC closure | `Box` per step |
