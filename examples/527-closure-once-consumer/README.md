📖 **[View on hightechmind.io →](https://hightechmind.io/rust/527-closure-once-consumer)**

---

# FnOnce for Consuming Closures

## Problem Statement

Some operations are inherently one-time: consuming a network connection, sending a channel message, releasing a one-time authentication token, or running a database transaction. Languages without linear types struggle to enforce "call at most once" at compile time, leading to runtime errors or logic bugs. Rust's `FnOnce` trait is the compile-time guarantee that a callable is invoked at most once — the type system physically prevents a second call by consuming the closure on the first. This maps directly to linear/affine types in type theory.

## Learning Outcomes

- How `FnOnce` differs from `Fn` and `FnMut` in terms of call constraints
- Why closures that move non-`Copy` values out of captures are automatically `FnOnce`
- How `with_resource<R, T, F: FnOnce(R) -> T>(resource, f)` implements resource bracketing
- How `OnceAction<F: FnOnce()>` wraps a one-shot action that can be safely called or dropped
- Where `FnOnce` appears in Rust's standard library: `thread::spawn`, `std::fs::File::create`

## Rust Application

`OneTimeToken` is a struct whose `consume(self)` takes ownership. `make_token_consumer(token)` returns `impl FnOnce() -> String` — the closure moves `token` into itself and calls `token.consume()` on invocation, which consumes the closure. Calling the closure a second time is a compile error. `with_resource<R, T, F: FnOnce(R) -> T>` is a bracket pattern: it calls `f(resource)` and returns the result, ensuring the resource is consumed exactly once. `OnceAction` wraps `F: FnOnce()` in `Option<F>` to allow `run` to be safe to call on a `mut self`.

Key patterns:
- `impl FnOnce() -> String` return type — caller cannot call twice
- `move || token.consume()` — moves non-Copy value, making closure `FnOnce`
- `Option<F>` + `take()` pattern for `OnceAction::run` on `&mut self`

## OCaml Approach

OCaml has no `FnOnce` equivalent — all functions can be called multiple times. One-time semantics are enforced by convention or by using a `ref bool` flag that raises an exception on second call:

```ocaml
let make_once f =
  let called = ref false in
  fun () -> if !called then failwith "called twice"
            else (called := true; f ())
```

This is a runtime check, not a compile-time guarantee.

## Key Differences

1. **Compile-time enforcement**: Rust's `FnOnce` makes double-call a compile error; OCaml has no equivalent — double-call protection must be implemented at runtime with a `ref` flag.
2. **Linear resources**: Rust's type system enforces linear use of `FnOnce` closures, aligning with affine/linear type theory; OCaml's GC-managed closures are always multiply-usable.
3. **`Option<F>` workaround**: When `FnOnce` must be stored in a struct and called via `&mut self`, Rust uses `Option::take()` to satisfy the borrow checker; OCaml needs no such workaround.
4. **Standard library integration**: `thread::spawn` takes `F: FnOnce() + Send + 'static` — a fundamental guarantee that the closure runs exactly once on the new thread; OCaml's `Thread.create` has no such type-level contract.

## Exercises

1. **Transaction closure**: Implement `with_transaction<F: FnOnce(Transaction) -> Result<(), String>>(db: &mut Database, f: F) -> Result<(), String>` that commits on `Ok` and rolls back on `Err`.
2. **Once-per-test helper**: Write `run_once_setup(setup: impl FnOnce() -> String)` that stores the result in a `OnceLock<String>` and verifies `setup` can only be called once from the outside.
3. **Deferred drop**: Create `Defer<F: FnOnce()>` that stores a closure and calls it in `Drop::drop`, implementing a scope-guard pattern without unsafe code.
