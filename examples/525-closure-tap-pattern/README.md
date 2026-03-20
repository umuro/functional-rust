📖 **[View on hightechmind.io →](https://hightechmind.io/rust/525-closure-tap-pattern)**

---

# Tap Pattern for Side Effects
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Data pipelines built with iterator chains or method chaining have a readability problem: inserting debug logging or instrumentation requires breaking the chain into temporary `let` bindings. The tap pattern solves this by injecting a side-effecting function at any point in a chain without disrupting the data flow — the function runs, but the original value passes through unchanged. This pattern appears in JavaScript's `.tap()` in lodash, Ruby's `Object#tap`, Haskell's `(<$)` for constant functors, and is commonly needed when debugging long iterator chains.

## Learning Outcomes

- How `tap<T, F: FnOnce(&T)>(value: T, f: F) -> T` threads a value through a side effect
- How to implement `tap` as an extension trait for ergonomic chaining with dot notation
- The difference between `tap` (immutable peek), `tap_mut` (mutable modification), and `tap_dbg` (debug printing)
- How `tap_if(condition, f)` enables conditional side effects without breaking the chain
- Where tap appears: logging pipelines, metrics instrumentation, test assertions in chains

## Rust Application

`tap<T, F: FnOnce(&T)>(value: T, f: F) -> T` runs `f(&value)` then returns `value`. The `Tap` trait is a blanket impl over all `T: Sized`, providing `.tap()`, `.tap_mut()`, and `.tap_dbg(label)` as methods. `tap_dbg` requires `T: Debug` and prints `"label: {:?}"` to stderr. `tap_if(value, condition, f)` calls `f` only when `condition` is `true`. These combinators compose naturally in iterator chains: `iter.map(...).tap(|x| log(x)).filter(...).collect()`.

Key patterns:
- Blanket `impl<T> Tap for T {}` — zero-cost extension for all types
- `FnOnce(&T)` for immutable peek, `FnOnce(&mut T)` for mutation
- `tap_dbg` gated on `T: Debug` using a where clause

## OCaml Approach

OCaml achieves tap via a simple helper that is idiomatic and common in pipelines using `|>`:

```ocaml
let tap f x = f x; x
let tap_debug label x = Printf.eprintf "%s: %s\n" label (Obj.repr x |> ...); x
(* usage *)
value |> tap (fun x -> log x) |> transform |> tap (fun x -> assert_ok x)
```

## Key Differences

1. **Method vs function**: Rust's `Tap` trait enables `value.tap(f)` dot-notation in method chains; OCaml uses `|>` with `tap f` as a free function — both achieve the same pipeline clarity.
2. **Mutable tap**: Rust `tap_mut` can modify the value in-place before it passes through; OCaml's `tap` with a `ref` or mutable record achieves the same but requires explicit `ref` cells.
3. **Debug constraint**: Rust's `tap_dbg` requires `T: Debug` at compile time; OCaml's `tap_debug` uses `Obj.repr` or format functions at runtime with less type safety.
4. **Blanket impl**: Rust's blanket `impl<T> Tap for T` adds methods to every type in scope; OCaml achieves this through the module system by opening a `Tap` module.

## Exercises

1. **Logged pipeline**: Build a pipeline that reads integers from a slice, doubles them, taps to log each doubled value, filters for values over 10, then collects — all in one method chain.
2. **Metric tap**: Implement `tap_count(counter: &mut usize)` that increments `counter` on each call through the chain — useful for profiling how many items pass a filter.
3. **Conditional mutation**: Use `tap_mut` to normalize strings (trim whitespace, lowercase) inside a pipeline without breaking the chain, then verify the final collected values are normalized.
