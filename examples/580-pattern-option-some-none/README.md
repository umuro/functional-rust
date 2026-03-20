📖 **[View on hightechmind.io →](https://hightechmind.io/rust/580-pattern-option-some-none)**

---

# Option Pattern Matching

## Problem Statement

`Option<T>` is Rust's type-safe replacement for null pointers. Tony Hoare called null his "billion-dollar mistake" — null references cause crashes, undefined behavior, and security vulnerabilities in C, C++, Java, and JavaScript. Rust's `Option<T>` forces explicit handling of the absent case at compile time. Every `Option` must be pattern-matched or explicitly unwrapped — the compiler prevents accessing the inner value without handling `None`. OCaml's `option` type predates Rust's `Option` and solves the same problem.

## Learning Outcomes

- How `match`, `if let`, `?`, `map`, `and_then`, `unwrap_or` handle `Option`
- Why `Option<T>` prevents null pointer errors at compile time
- How `safe_div` and `safe_sqrt` express partial functions as `Option` return types
- How `map` and `and_then` compose `Option` values without nested `match`
- Where `Option` replaces sentinel values: parsing, lookup, optional fields

## Rust Application

`safe_div(a, b) -> Option<i32>` returns `None` for division by zero, `Some(a/b)` otherwise. `safe_sqrt` similarly guards negative inputs. Pattern matching: `match opt { Some(n) => n * 2, None => 0 }`. Combinators: `opt.map(|n| n * 2)`, `opt.and_then(|n| if n > 0 { Some(n) } else { None })`, `opt.unwrap_or(default)`. The `?` operator in a `fn -> Option<T>` returns `None` early if the inner `Option` is `None`.

Key patterns:
- `match opt { Some(v) => ..., None => ... }` — explicit handling
- `opt.map(|v| transform(v))` — transform inner value
- `opt.and_then(|v| another_option(v))` — chain fallible operations
- `opt?` — propagate `None` with early return

## OCaml Approach

OCaml's `option` type predates Rust's `Option`:

```ocaml
let safe_div a b = if b = 0 then None else Some (a / b)
let safe_sqrt x = if x < 0.0 then None else Some (sqrt x)
let map f = function None -> None | Some x -> Some (f x)
let and_then f = function None -> None | Some x -> f x
```

## Key Differences

1. **Historical priority**: OCaml's `option` type and `None`/`Some` constructors predate Rust — Rust adopted and refined the pattern.
2. **`?` operator**: Rust has `?` for early return on `None`; OCaml uses `Option.bind` or monadic `let*` syntax.
3. **Null safety**: Both eliminate null pointer errors at compile time — Rust additionally prevents using an `Option` without checking it.
4. **Combinators**: Rust's `Option` has many methods (`map`, `and_then`, `filter`, `or_else`, etc.); OCaml's `Option` module has equivalent functions.

## Exercises

1. **Safe lookup chain**: Write `fn get_street(db: &DB, user_id: u32) -> Option<&str>` using `and_then` to chain: lookup user → get address → get street, returning `None` at any missing step.
2. **Option arithmetic**: Implement `fn add_opts(a: Option<i32>, b: Option<i32>) -> Option<i32>` using `and_then` or `zip` to return `None` if either input is `None`.
3. **Parse pipeline**: Write `fn parse_and_double(s: &str) -> Option<i32>` using `s.parse::<i32>().ok()` followed by `.map(|n| n * 2)` — no `match` or `if let` needed.
