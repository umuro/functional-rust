# 504: Returning Closures from Functions

**Difficulty:** 3  **Level:** Intermediate

Factory functions that produce behavior — closures as return values with `impl Fn` or `Box<dyn Fn>`.

## The Problem This Solves

You want to write a function that *configures* behavior and hands it back to the caller. Think `make_validator(min, max)` that returns a function checking whether a value is in range, or `make_adder(5)` that returns a function adding 5 to anything.

Without returning closures, you'd have to duplicate this logic everywhere, or pass both the function *and* its configuration as separate arguments wherever you need it. The configuration and the behavior get separated, making code harder to reason about.

The tricky part in Rust: closures are anonymous types. You can't write `fn make_adder(n: i32) -> |i32| -> i32` — the compiler needs to know the return type's size. Two solutions: `impl Fn` for a single concrete type, `Box<dyn Fn>` when you need to return different closure types from different branches.

## The Intuition

Returning a closure is like a factory producing configured tools. You call `make_adder(5)` once, and every subsequent call to the returned closure adds 5. The `5` lives *inside* the closure — no global state, no class, no struct needed.

In Python: `def make_adder(n): return lambda x: x + n` — the returned lambda closes over `n`. In JavaScript: `const makeAdder = n => x => x + n`. Rust needs one extra step: telling the compiler the return type.

Use `-> impl Fn(i32) -> i32` when you return *one specific* closure type (static dispatch, inlined). Use `-> Box<dyn Fn(i32) -> i32>` when you return *different closure types* from different code paths (dynamic dispatch, heap-allocated).

## How It Works in Rust

```rust
// impl Fn: zero-cost, compiler resolves the concrete type
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n    // n must be MOVED — it lives on make_adder's stack
}
let add5 = make_adder(5);
println!("{}", add5(10)); // 15
println!("{}", add5(20)); // 25  — reusable

// Box<dyn Fn>: required when returning DIFFERENT closure types
fn make_transformer(op: &str) -> Box<dyn Fn(i32) -> i32> {
    match op {
        "double" => Box::new(|x| x * 2),   // different closure types
        "square" => Box::new(|x| x * x),   // per branch — needs Box
        "negate" => Box::new(|x| -x),
        _        => Box::new(|x| x),
    }
}

// Closure capturing an owned String (move required)
fn make_greeter(greeting: String) -> impl Fn(&str) -> String {
    move |name| format!("{}, {}!", greeting, name)
    //          ^-- greeting moved into closure; closure owns it
}
let greet = make_greeter("Hello".to_string());
println!("{}", greet("Rust"));  // "Hello, Rust!"
println!("{}", greet("world")); // still works — closure owns the String

// Why move? The stack frame of make_greeter is gone after the call.
// The closure must OWN the data to keep it alive.
```

## What This Unlocks

- **Configuration factories** — generate pre-configured validators, formatters, or handlers with captured settings.
- **Currying and partial application** — `make_adder(5)` is partial application; chain them for currying.
- **Plugin systems** — return different closure implementations based on runtime configuration, all behind a uniform `Box<dyn Fn>` interface.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Return a function | `let f () = fun x -> x + 1` | `fn f() -> impl Fn(i32) -> i32` |
| Return type spelling | Inferred function type | `impl Fn(...)` or `Box<dyn Fn(...)>` |
| Captured variable lifetime | GC managed | Must `move` into closure to outlive fn |
| Different types per branch | Easy — structural typing | Requires `Box<dyn Fn>` |
| Dynamic dispatch | Always (all functions boxed) | Opt-in: `Box<dyn Fn>` with vtable |
