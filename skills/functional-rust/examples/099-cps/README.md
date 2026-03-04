# 099: CPS — Continuation-Passing Style

**Difficulty:** 3  **Level:** Advanced

Instead of returning a value, pass a callback that receives the result — giving you total control over what happens next.

## The Problem This Solves

You write a recursive function. It works perfectly for small inputs. Then someone calls it with a deeply nested tree or a large number, and you get a stack overflow. The Rust default stack is 8MB, and deep recursion will eat through it fast.

The usual fix is to rewrite the recursion as a loop. But sometimes — for tree traversal, parsers, interpreters — that rewrite is painful. You lose the clarity of the recursive structure while adding complex state management.

CPS (Continuation-Passing Style) is a systematic transformation that fixes this. Instead of a function that *returns* a value, you write a function that *calls a callback* with the value. That callback — the "continuation" — represents "what happens next." With CPS, every call becomes a tail call, which means the stack stays constant.

There's a deeper payoff: once you hold "what happens next" as a first-class value, you can manipulate it. You can skip it (early exit). You can store it (suspending a computation). You can call it multiple times (backtracking). This is how async, generators, and coroutines work under the hood — and it's all CPS. This example exists to teach you that pattern.

## The Intuition

Normal functions: compute result → return it to whoever called you.  
CPS functions: compute result → pass it to a callback you were given.

```rust
// Normal style: returns the value
fn double(x: i32) -> i32 {
    x * 2
}

// CPS style: calls the callback with the value  
// k is the "continuation" — pronounced "what to do next"
fn double_cps<R>(x: i32, k: impl Fn(i32) -> R) -> R {
    k(x * 2)  // instead of returning, we CALL k
}

// Using it:
let result = double(5);                    // normal: result = 10
double_cps(5, |result| println!("{}", result)); // CPS: callback receives 10
```

The big insight for recursion: with CPS, you build up the "pending work" in the callback chain instead of on the call stack. The stack stays flat; the heap grows instead.

```rust
// Normal factorial — stack grows with n (can overflow!)
fn factorial(n: u64) -> u64 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
    //                      ^--- each call waits here, stack frame stays alive
}

// CPS factorial — each call is a TAIL CALL (no waiting)
fn factorial_cps(n: u64, k: Box<dyn FnOnce(u64) -> u64>) -> u64 {
    if n == 0 {
        k(1)  // base case: call continuation with 1
    } else {
        // Recursive call is LAST thing we do — tail call!
        // The "multiply by n" is wrapped into the continuation
        factorial_cps(n - 1, Box::new(move |result| k(n * result)))
    }
}
```

## How It Works in Rust

**Direct recursion (not stack-safe):**
```rust
fn factorial(n: u64) -> u64 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
    // Stack frame for each call! Depth 100_000 → stack overflow
}
```

**CPS transformation:**
```rust
fn factorial_cps(n: u64) -> u64 {
    fn go(n: u64, k: Box<dyn FnOnce(u64) -> u64>) -> u64 {
        if n == 0 {
            k(1)  // done: call the continuation with 1
        } else {
            // Pass a new continuation that wraps "multiply by n"
            go(n - 1, Box::new(move |result| k(n * result)))
        }
    }
    go(n, Box::new(|x| x))  // identity continuation to start
}
```

Note: in OCaml, this is truly stack-safe because the compiler does Tail Call Optimization (TCO). Rust does NOT guarantee TCO, so in Rust, CPS alone doesn't prevent stack overflow — you also need a trampoline (see example 197). But CPS is the essential first step.

**CPS for early exit — two continuations:**
```rust
fn find_cps<T: Copy>(
    pred: &dyn Fn(T) -> bool,
    list: &[T],
    found: &dyn Fn(T) -> Option<T>,      // called when found
    not_found: &dyn Fn() -> Option<T>,   // called when exhausted
) -> Option<T> {
    if list.is_empty() {
        not_found()          // nothing matched: take this branch
    } else if pred(list[0]) {
        found(list[0])       // found one: take this branch immediately
    } else {
        find_cps(pred, &list[1..], found, not_found)
    }
}
// Two continuations = two possible outcomes = early exit for free!
```

**The idiomatic Rust way** (for most cases): avoid CPS entirely, use iterators.
```rust
// This does the same as CPS find, with zero overhead:
let first_even = nums.iter().find(|&&x| x % 2 == 0);
```

## What This Unlocks

- **Stack safety**: Combine CPS with a trampoline (example 197) and you get infinite recursion depth with O(1) stack.
- **Control flow superpowers**: Async/await is CPS. `?` operator is CPS. Generators are CPS. Once you see this, you see it everywhere in Rust code.
- **Interpreter and compiler writing**: CPS is the standard intermediate representation in optimizing compilers (used by GHC, MLTON, and others). Understanding it makes compiler papers readable.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| CPS is tail-recursive? | Yes — OCaml guarantees TCO | No — needs trampoline too |
| Continuation type | `'a -> 'b` (lightweight) | `Box<dyn FnOnce(A) -> B>` (heap) |
| Closure allocation cost | Near-zero (GC stack) | One heap alloc per continuation |
| Practical recommendation | CPS is viable | Use iterators; CPS = educational |
| The `?` operator | `result >>= continuation` | CPS in disguise! |
