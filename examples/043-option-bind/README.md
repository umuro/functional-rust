📖 **[View on hightechmind.io →](https://hightechmind.io/rust/043-option-bind)**

---

# 043 — Option Bind (and_then)

## Problem Statement

`Option::and_then` (also called "bind" in Haskell/OCaml) is the monadic sequencing operation for `Option`. Where `map(f)` applies `f: T -> U` and wraps in `Some`, `and_then(f)` applies `f: T -> Option<U>` — the function itself decides whether to return `Some` or `None`. This prevents double-wrapping: `map` on an `Option`-returning function produces `Option<Option<U>>`; `and_then` flattens it to `Option<U>`.

`and_then` is the essence of the option monad: "do this, but only if the previous step succeeded". It enables sequential fallible computations without nesting: `find_user(id).and_then(|u| find_account(u.id)).and_then(|a| check_balance(a))` — each step may fail, and failure propagates automatically.

## Learning Outcomes

- Use `opt.and_then(|x| maybe_f(x))` to chain fallible computations
- Understand the difference between `map` and `and_then`: map returns `U`, and_then returns `Option<U>`
- Recognize that `and_then` is equivalent to `map` followed by `flatten`
- Chain multiple `and_then` calls to sequence optional operations
- Use `?` in functions returning `Option<T>` as syntactic sugar for `and_then`

## Rust Application

`opt.and_then(|x| safe_div(x, 2))` applies `safe_div` which returns `Option<i32>`. If `opt` is `None`, the result is `None` without calling `safe_div`. If `safe_div` returns `None` (division by zero), the chain ends there. Chaining: `safe_head(v).and_then(|i| v.get(i as usize).copied())` — use the head value as an index, failing if out of bounds. The `?` operator in `fn f() -> Option<T>` desugars to `and_then` chains.

## OCaml Approach

OCaml's `Option.bind opt f`: `let bind opt f = match opt with None -> None | Some x -> f x`. The `|>` pipe with `Option.bind`: `opt |> Option.bind safe_div`. OCaml 4.08+ provides `Option.bind`. With `ppx_let`: `let* x = find_user id in let* acc = find_account x.id in check_balance acc` — this is the monadic let syntax that makes sequenced options look like sequential code.

## Key Differences

1. **Naming**: Rust calls it `and_then` (method), Haskell `>>=`, OCaml `Option.bind`. All are the same operation: `T -> Option<U>` applied to `Option<T>` producing `Option<U>`.
2. **`?` operator**: Rust's `?` in a function returning `Option` is `and_then` spelled differently — `x?` means "extract from Some, or return None early". OCaml needs `ppx_let` for `let*` syntax.
3. **`map` vs `and_then` choice**: Use `map` when the transformation cannot fail (`|x| x * 2`). Use `and_then` when the transformation is itself fallible (`|x| safe_div(x, 2)`).
4. **`flatten`**: `opt.map(f).flatten()` is equivalent to `opt.and_then(f)`. Rust provides both; use `and_then` directly as it is more efficient.

## Exercises

1. **Parse chain**: Write `parse_and_double(s: &str) -> Option<i32>` that parses a string to an integer and doubles it, returning None if parsing fails. Use `s.parse::<i32>().ok().map(|x| x * 2)`.
2. **Nested lookup**: Write `lookup_nested(outer: &HashMap<i32, HashMap<i32, String>>, k1: i32, k2: i32) -> Option<&String>` using `outer.get(&k1).and_then(|inner| inner.get(&k2))`.
3. **Option pipeline**: Write a function that: (1) finds a user by ID (`Option<User>`), (2) finds their primary address (`Option<Address>`), (3) formats the city name (may be missing). Chain with `and_then`.
