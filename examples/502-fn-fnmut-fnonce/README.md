📖 **[View on hightechmind.io →](https://hightechmind.io/rust/502-fn-fnmut-fnonce)**

---

# Fn, FnMut, FnOnce
**Difficulty:** ⭐  
**Category:** Functional Programming  



The three closure traits form a hierarchy: `FnOnce` (callable once) ⊇ `FnMut` (callable multiple times with mutation) ⊇ `Fn` (callable multiple times, no mutation). Every closure implements at least `FnOnce`; the compiler automatically determines which traits apply.

## Problem Statement

Higher-order functions need to express constraints on how many times and in what way a callable can be invoked. A sort key function must be callable many times without side effects — `Fn`. A stateful counter must be called multiple times with mutation — `FnMut`. A function that consumes its captured data should only be called once — `FnOnce`. Without these distinctions, the type system could not prevent: calling a one-shot closure twice (use-after-move), forgetting to declare mutable access, or passing a mutable closure where immutable sharing is expected.

## Learning Outcomes

- Bound function parameters with `F: Fn()`, `F: FnMut()`, `F: FnOnce()`
- Understand the hierarchy: `Fn: FnMut: FnOnce` (subtype relationship)
- Declare the call variable `mut f` for `FnMut` callers
- Write closures that implement each trait automatically
- Use `dyn Fn`/`dyn FnMut`/`dyn FnOnce` for trait objects

## Rust Application

`Fn` — called multiple times, borrows immutably:

```rust
pub fn call_fn<F: Fn() -> i32>(f: F) -> i32 { f() + f() }
// Usage: call_fn(|| x)  — x borrowed immutably
```

`FnMut` — called multiple times, may mutate:

```rust
pub fn call_fn_mut<F: FnMut() -> i32>(mut f: F) -> i32 { f() + f() }
// Usage: call_fn_mut(|| { counter += 1; counter })
```

`FnOnce` — called exactly once, consumes captures:

```rust
pub fn call_fn_once<F: FnOnce() -> String>(f: F) -> String { f() }
// Usage: call_fn_once(|| s)  — s moved out
```

The hierarchy in action — `Fn` coerces to `FnMut` and `FnOnce`:

```rust
let f: &dyn Fn() -> i32 = &|| x;
let _: &dyn FnMut() -> i32 = f;   // Fn implies FnMut
let _: &dyn FnOnce() -> i32 = f;  // FnMut implies FnOnce
```

## OCaml Approach

OCaml functions are first-class but have no trait hierarchy — all closures are uniformly applicable:

```ocaml
let call_fn (f: unit -> int) = f () + f ()
let call_fn_mut (f: unit -> int) = f () + f ()  (* same signature *)

(* "FnOnce" semantics must be enforced manually *)
let call_once f =
  let called = ref false in
  fun () -> if !called then failwith "called twice"
            else (called := true; f ())
```

OCaml has no type-level distinction between a closure that mutates, one that doesn't, or one that should only be called once.

## Key Differences

1. **Type-level enforcement**: Rust's `FnOnce` prevents double-calling at compile time; OCaml must use runtime guards (`ref bool`) for the same constraint.
2. **`mut f` requirement**: Rust's call site must declare `mut f` for `FnMut` callers; OCaml has no such requirement.
3. **Trait objects**: `Box<dyn Fn()>`, `Box<dyn FnMut()>`, and `Box<dyn FnOnce()>` are distinct types in Rust; OCaml uses a single function type.
4. **Subtyping**: Rust's `Fn: FnMut: FnOnce` subtype coercions happen at compile time via trait coherence; OCaml has no equivalent.

## Exercises

1. **Retry combinator**: Write `fn retry<F: FnMut() -> bool>(mut f: F, n: usize) -> bool` that calls `f` up to `n` times and returns `true` on the first success.
2. **`once` adapter**: Write `fn once<F: FnOnce() -> T, T>(f: F) -> impl FnMut() -> Option<T>` that wraps a `FnOnce` in a `FnMut` that returns `Some(result)` on first call and `None` thereafter.
3. **Trait object cost**: Benchmark `dyn Fn(i32)->i32` (dynamic dispatch via vtable) against `impl Fn(i32)->i32` (monomorphised static dispatch) for 10 million calls using `criterion`.
