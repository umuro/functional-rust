📖 **[View on hightechmind.io →](https://hightechmind.io/rust/312-error-downcasting)**

---

# 312: Error Downcasting
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

When errors are stored as `Box<dyn Error>` for flexibility, the concrete type is erased. Downcasting recovers the concrete type at runtime when specific error handling is needed — retrying on network timeouts but propagating authentication errors, for example. This is `downcast_ref::<ConcreteType>()` on a `dyn Error` — the Rust equivalent of `instanceof` checks or `catch (SpecificException e)` in Java/Python.

## Learning Outcomes

- Use `error.downcast_ref::<ConcreteType>()` to attempt runtime type recovery
- Match on `Some(concrete)` vs `None` to handle specific vs unknown error types
- Understand the `'static` lifetime requirement for downcastable error types
- Walk the `source()` chain to downcast errors at any level

## Rust Application

`downcast_ref::<T>()` returns `Option<&T>` — `Some` if the error is of that type, `None` if not:

```rust
fn handle_error(e: &(dyn Error + 'static)) {
    if let Some(parse_err) = e.downcast_ref::<ParseError>() {
        println!("Parse error: '{}'", parse_err.input);
    } else if let Some(net_err) = e.downcast_ref::<NetworkError>() {
        println!("Network error code {}: {}", net_err.code, net_err.message);
    } else {
        println!("Unknown error: {}", e);
    }
}
```

The type must be `'static` — errors containing borrowed references cannot be downcast.

## OCaml Approach

OCaml's exception system uses `match exn with | SpecificException data -> ...` for typed error discrimination. For `result` error values, pattern matching on variant types achieves the same without runtime type checks:

```ocaml
let handle_error = function
  | `Parse input -> Printf.printf "Parse error: '%s'\n" input
  | `Network (code, msg) -> Printf.printf "Network %d: %s\n" code msg
  | e -> Printf.printf "Unknown: %s\n" (to_string e)
```

OCaml's pattern matching on sum types is static and exhaustive — downcasting is unnecessary when using algebraic types.

## Key Differences

1. **Static vs dynamic**: OCaml's variant matching is static and compile-time checked; Rust's `downcast_ref` is a dynamic runtime check.
2. **When needed**: Downcasting is necessary when errors are stored as `dyn Error`; with concrete error enum types, pattern matching suffices in Rust too.
3. **Source chain**: Rust can downcast errors anywhere in the `source()` chain; OCaml exceptions must be explicitly carried through the call stack.
4. **Performance**: `downcast_ref` uses type IDs for O(1) checking; multiple downcasts are faster than multiple pattern matches but require knowing all possible types.

## Exercises

1. Write a function that takes a `Box<dyn Error>` and attempts to downcast it to three different concrete types, logging a type-specific message for each.
2. Walk the full `source()` chain of a nested error, attempting to downcast each level, and return the first level that is a specific `IoError`.
3. Demonstrate that a `dyn Error` without `'static` cannot be downcast, and explain why the `'static` bound is required.
