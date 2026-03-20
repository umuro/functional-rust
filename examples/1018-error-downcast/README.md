📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1018-error-downcast)**

---

# 1018-error-downcast — Error Downcast

## Problem Statement

When errors are type-erased as `Box<dyn Error>` or `Arc<dyn Error>`, you lose the ability to pattern-match on specific error types. This is the trade-off of dynamic dispatch: flexibility at the cost of type information. Downcasting recovers the concrete type at runtime, using the `Any` mechanism under the hood.

This pattern appears wherever error types cross API boundaries: plugin systems, dynamic library interfaces, and functions returning `Box<dyn Error>` for flexibility. The `downcast_ref` and `downcast` methods are Rust's equivalent of Java's `instanceof` check plus cast.

## Learning Outcomes

- Understand how `Box<dyn Error>` erases the concrete error type
- Use `downcast_ref::<ConcreteError>()` to recover a reference to the concrete type
- Use `downcast::<ConcreteError>()` to take ownership of the concrete type from a `Box`
- Walk the `Error::source()` chain to find errors nested inside wrappers
- Know the limitations: downcasting requires `'static` bounds on the error type

## Rust Application

`src/lib.rs` defines `DatabaseError`, `AuthError`, and `NetworkError`, each implementing `std::error::Error`. Functions return `Box<dyn Error>` (type-erased). `classify_error` uses `downcast_ref` to identify the concrete type without taking ownership. `recover_from_error` uses `downcast::<DatabaseError>()` on a `Box<dyn Error + 'static>` to take ownership and extract the inner value.

The `'static` bound on `downcast_ref` and `downcast` is necessary because `Any`'s type ID mechanism requires knowing the concrete type at compile time.

## OCaml Approach

OCaml exceptions carry typed payloads and can be matched directly without downcasting:

```ocaml
exception Database_error of string
exception Auth_error of string

let classify exn =
  match exn with
  | Database_error msg -> "database: " ^ msg
  | Auth_error msg -> "auth: " ^ msg
  | _ -> "unknown"
```

When using `Or_error`, the `Error.to_exn` and `Error.of_exn` functions bridge between exceptions and the `Error.t` type. There is no downcasting because exceptions are always typed at the match site.

## Key Differences

1. **Type erasure**: Rust explicitly erases types with `dyn Trait`; OCaml exceptions are always fully typed and matchable.
2. **Runtime overhead**: `downcast_ref` performs a single type-ID comparison (essentially free); OCaml match compilation is similar in cost.
3. **`'static` bound**: Rust downcasting requires the error type to be `'static`; OCaml has no equivalent restriction.
4. **Safety**: Rust downcasting cannot produce unsound code — a failed downcast returns `None`/`Err`; OCaml pattern matching is always exhaustive.

## Exercises

1. Write a function that walks the `Error::source()` chain recursively and returns a `Vec<&str>` of all error messages from root to leaf.
2. Implement a `try_recover<E: Error + 'static>(err: Box<dyn Error>) -> Result<(), E>` generic function that downcasts and returns the specific error type if it matches.
3. Add a `WrappedError` struct that wraps another `Box<dyn Error>` and implements `Error::source()`. Show that downcasting into the inner error still works via source-chain walking.
