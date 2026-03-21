📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1012-never-type)**

---

# 1012-never-type — The Never Type (!)
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Some computations never return a value: they panic, loop forever, or exit the process. In type theory, this is the "bottom" type — a type with no inhabitants. Rust makes this explicit with the `!` type (pronounced "never"). Functions returning `!` can appear in any expression position because they are coerced to any type, which allows them to unify with arbitrary branches in `match` expressions.

The never type also enables safe encoding of impossible states. `std::convert::Infallible` is an uninhabited enum (effectively `!`) used as the error type in infallible `Result`s.

## Learning Outcomes

- Understand what the `!` type means and why it is called the bottom type
- Write diverging functions annotated with `-> !`
- Use `!` in match arms with `unreachable!()` and `panic!()` for exhaustiveness
- Handle `Result<T, Infallible>` by matching on an empty enum
- Appreciate how `!` coerces to any type in expression context

## Rust Application

`src/lib.rs` shows three patterns. `diverge_panic()` and `diverge_loop()` return `!`, demonstrating that diverging functions satisfy any return type requirement. `handle_infallible` matches a `Result<i64, Infallible>`: the `Err` arm uses an empty match `match e {}` — valid because `Infallible` has no constructors so the arm is unreachable. `classify` uses `unreachable!()` in the wildcard arm; since `unreachable!()` returns `!`, it unifies with the `String` return type of other arms.

The `always_succeeds` function illustrates that `Result<i64, Infallible>` is a common return type for functions that implement a fallible interface but cannot actually fail.

## OCaml Approach

OCaml has no explicit `!` type in surface syntax, but the concept exists as the `'a` type variable in functions like `failwith : string -> 'a`. An uninhabited type can be encoded as an empty variant:

```ocaml
type never = |  (* empty variant — no constructors *)

let handle_never (r : (int, never) result) =
  match r with
  | Ok n -> n
  | Error e -> match e with  (* exhaustive because never has no cases *)
```

The `Error` arm is compiled away since it can never be entered.

## Key Differences

1. **Named bottom type**: Rust has `!` as a first-class type; OCaml encodes it as an empty variant or uses the polymorphic `'a` return.
2. **Standard library support**: Rust's `std::convert::Infallible` is the canonical uninhabited type; OCaml lacks a standard equivalent.
3. **Match exhaustiveness**: Both compilers understand that an empty type requires no match arms, but Rust expresses this with `match e {}` syntax.
4. **Coercion**: Rust automatically coerces `!` to any type in expression position; OCaml's `'a` return type is universally polymorphic which achieves the same effect.

## Exercises

1. Write a function `safe_index(v: &[i32], i: usize) -> i32` that panics with a custom message if `i` is out of bounds. Annotate a helper that builds the panic message as `-> !`.
2. Implement a `Result<u32, Infallible>` producer that parses a hardcoded string known to always be valid. Use `handle_infallible` to unwrap it without `unwrap()`.
3. Add a new `ParseOrNever` enum variant `Computed(i64)` and add a match arm in a function that covers all variants — demonstrating exhaustive matching.
