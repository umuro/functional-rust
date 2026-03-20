📖 **[View on hightechmind.io →](https://hightechmind.io/rust/927-option-result)**

---

# 927-option-result — Option and Result

## Problem Statement

Null pointer dereferences are the billion-dollar mistake — Tony Hoare's famous regret. Languages like ML, Haskell, Rust, and Swift replace null with explicit optional types: `option` / `Option<T>`. The type system forces the programmer to handle the "not present" case. For errors, exceptions are non-local, untyped, and easy to forget. `Result<T, E>` (or `either` in Haskell) makes errors explicit in the type signature. OCaml has `option` and `result`; Rust has `Option<T>` and `Result<T, E>`. Both provide `map`, `and_then`, `unwrap_or` combinators for composing safe computations.

## Learning Outcomes

- Use `Option<T>` for nullable lookups and safe head/tail operations
- Chain Option values with `.map()` and `.and_then()` (monadic bind)
- Use `Result<T, E>` for fallible operations with typed errors
- Chain Result values with `.map()` and `.and_then()`
- Use the `?` operator for ergonomic error propagation

## Rust Application

`find_first`, `safe_div`, `head`, and `last` return `Option<T>`. `find_and_double` chains `.find().map(|x| x * 2)`. `nth_or_default` uses `.get(n).cloned().unwrap_or(default)`. `parse_int` and `positive` return `Result<i32, String>`. `process` uses `?` for short-circuiting: `let n = parse_int(s)?; let n = positive(n)?; sqrt_safe(n)`. The `?` operator is syntactic sugar for `.ok_or()` + early return on `Err` — equivalent to OCaml's `let*` bind.

## OCaml Approach

OCaml's `option` and `result` types have the same structure. `Option.bind: 'a option -> ('a -> 'b option) -> 'b option` is `and_then`. `Option.map: ('a -> 'b) -> 'a option -> 'b option`. `Result.bind` and `Result.map` are analogous. OCaml 4.08+ adds `let*` operator for monadic bind in `Option` and `Result`: `let* n = parse_int s in let* n = positive n in sqrt_safe n`. OCaml lacks the `?` operator but `let*` provides equivalent ergonomics when used with `Option.syntax` or `Result.syntax`.

## Key Differences

1. **? operator**: Rust's `?` short-circuits on `None`/`Err` with one character; OCaml requires `let*` or explicit `Result.bind` calls.
2. **Error conversion**: Rust `?` automatically converts error types using `From`; OCaml requires explicit `Result.map_error`.
3. **Standard combinators**: Both provide `map`, `bind`/`and_then`, `unwrap_or`, `get_or_else`; names differ slightly.
4. **Exhaustiveness**: Both enforce handling both variants in pattern matching; unwrapping without checking is unsafe in both (Rust `unwrap()` panics, OCaml `Option.get` raises).

## Exercises

1. Implement `safe_chain<T, U, V>(opt: Option<T>, f: impl Fn(T) -> Option<U>, g: impl Fn(U) -> Option<V>) -> Option<V>` using `and_then`.
2. Write `accumulate_errors(validations: Vec<Result<(), String>>) -> Result<(), Vec<String>>` that collects all errors instead of stopping at the first.
3. Implement a small expression evaluator using `Result<f64, String>` for division-by-zero and parse errors, using `?` throughout.
