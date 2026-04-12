📖 **[View on hightechmind.io →](https://hightechmind.io/rust/309-never-type-errors)**

---

# 309: The Never Type (!) in Error Handling
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Some functions never return — they loop forever, panic, or call `std::process::exit()`. Rust's `!` (never type) represents the return type of such functions. It coerces to any type, enabling `panic!()` in match arms that need any type, and it is the error type for infallible operations. `std::convert::Infallible` is the named equivalent for `Result<T, !>` — a result that can only ever be `Ok`. Understanding `!` clarifies why certain patterns compile.

## Learning Outcomes

- Understand `!` (never type) as the return type of functions that never return
- Recognize that `!` coerces to any type, enabling use in any expression context
- Use `std::convert::Infallible` to mark operations that cannot fail
- Understand how `!` enables `unwrap_infallible()` — extraction without panic

## Rust Application

The never type enables diverging functions to appear in any expression:

```rust
// ! return type signals "never returns"
pub fn crash(msg: &str) -> ! { panic!("{}", msg) }

// ! coerces to any type:
pub fn parse_or_crash(s: &str) -> i32 {
    s.parse::<i32>().unwrap_or_else(|e| crash(&format!("fatal: {}", e)))
    // crash() has type !, which coerces to i32 here
}

// Infallible: Result that can only be Ok
pub fn to_uppercase(s: &str) -> Result<String, Infallible> {
    Ok(s.to_uppercase())  // Never fails
}
// Safe to unwrap — the error type is Infallible:
let upper = to_uppercase("hello").unwrap();
```

## OCaml Approach

OCaml's `'a` bottom type (`type 'a t = T of 'a`) doesn't have a direct equivalent. The `raise` function has type `exn -> 'a` — it is polymorphic because it never returns, so it can be used anywhere any type is expected:

```ocaml
(* raise has type exn -> 'a — can appear in any context *)
let parse_or_crash s =
  match int_of_string_opt s with
  | Some n -> n
  | None -> raise (Invalid_argument "not a number")
```

OCaml lacks `Infallible` — there is no way to statically prove a `result` error case is unreachable.

## Key Differences

1. **Type system integration**: Rust's `!` is a first-class type that participates in type checking; OCaml's `raise` works by being polymorphic rather than having a special type.
2. **Infallible conversions**: `Result<T, Infallible>` proves at the type level that a conversion cannot fail — OCaml has no equivalent guarantee.
3. **From impl**: `impl From<Infallible> for Any` enables writing generic code that handles `!` errors uniformly without special cases.
4. **Stabilization**: The `!` type was a long-stabilization feature in Rust — older code uses `std::convert::Infallible` (an alias for `!`) for compatibility.

## Exercises

1. Write a function that uses `!` in a match arm to prove exhaustiveness: a match on a `Result<T, Infallible>` that handles only `Ok`.
2. Implement `From<Infallible> for MyError` to demonstrate that infallible results can be unified with fallible ones in generic code.
3. Create a `NeverFails<T>` type alias for `Result<T, Infallible>` and demonstrate that `unwrap()` on it is always safe.
