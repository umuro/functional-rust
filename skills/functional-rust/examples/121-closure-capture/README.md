# 121: Closure Capture Modes

**Difficulty:** 2  **Level:** Intermediate

Rust closures capture the minimum needed — by reference, by mutable reference, or by ownership — and you control which with the `move` keyword.

## The Problem This Solves

When a closure refers to variables from its enclosing scope, those variables have to come from somewhere. In OCaml, the GC handles this transparently — closures capture a reference to the environment, and the GC keeps it alive as long as needed. In Rust there's no GC, so the compiler must decide: does the closure borrow the variable, or does it own it?

The compiler chooses the least restrictive mode: shared reference for read-only access, mutable reference if the closure mutates the variable. This is good — but it has a limit. If the closure needs to outlive its creation scope (e.g., returned from a function or sent to a thread), it can't hold borrows into a stack frame that's about to disappear. The compiler will refuse.

The fix is the `move` keyword. `move || ...` forces all captured variables to be moved into the closure, giving it ownership. The closure is now self-contained and can live as long as needed. For `Copy` types like `i32`, the "move" is just a copy, so the original variable is still usable.

## The Intuition

By default closures borrow; add `move` when the closure needs to own its captures — typically when returning a closure from a function or sending it to another thread.

## How It Works in Rust

```rust
// By shared reference — default for read-only access
let x = 42;
let f = || x + 1;   // borrows &x
assert_eq!(f(), 43);
assert_eq!(x, 42);  // x is still usable — just borrowed

// By mutable reference — default when closure mutates
let mut total = 0;
let mut add = |n: i32| total += n;  // borrows &mut total
add(10);
add(20);
drop(add);          // mutable borrow ends when closure is dropped
assert_eq!(total, 30);

// By move — needed when closure outlives the creator
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
    // Without `move`, `factor` is borrowed from make_multiplier's stack.
    // With `move`, factor is owned by the closure — safe to return.
}
let double = make_multiplier(2);
let triple = make_multiplier(3);
assert_eq!(double(5), 10);
assert_eq!(triple(5), 15);

// Move with non-Copy: original variable is gone
let name = String::from("Alice");
let greet = move || format!("Hello, {}!", name);
// name is moved — this would fail: println!("{}", name);
println!("{}", greet());  // "Hello, Alice!"
```

## What This Unlocks

- **Closures returned from functions** — `move` closures are self-contained and don't borrow the local stack.
- **Thread safety** — `thread::spawn` requires `move` closures because the new thread might outlive the current scope.
- **Factory functions** — `make_multiplier`, `make_formatter`, `make_predicate`: build specialized closures with baked-in state.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Capture mode | Always by GC reference — implicit | By ref (default) or owned (`move`) |
| Outliving the scope | Always works — GC handles it | Requires `move` (owned closure) |
| Copy types with `move` | N/A | Copied, not moved — original still valid |
| Mutable captures | Via `ref` cell inside closure | Mutable borrow (`&mut`) — one at a time |
