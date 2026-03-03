# OCaml vs Rust: Result Monad — Error Chaining

## Side-by-Side Code

### OCaml

```ocaml
let ( >>= ) r f = match r with
  | Error _ as e -> e
  | Ok x -> f x

let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error ("Not an integer: " ^ s)

let check_positive n =
  if n > 0 then Ok n else Error "Must be positive"

let check_even n =
  if n mod 2 = 0 then Ok n else Error "Must be even"

let validate s =
  parse_int s >>= check_positive >>= check_even
```

### Rust (idiomatic — and_then chain)

```rust
pub fn parse_int(s: &str) -> Result<i64, String> {
    s.parse::<i64>().map_err(|_| format!("Not an integer: {s}"))
}

pub fn check_positive(n: i64) -> Result<i64, String> {
    if n > 0 { Ok(n) } else { Err("Must be positive".to_string()) }
}

pub fn check_even(n: i64) -> Result<i64, String> {
    if n % 2 == 0 { Ok(n) } else { Err("Must be even".to_string()) }
}

pub fn validate_idiomatic(s: &str) -> Result<i64, String> {
    parse_int(s).and_then(check_positive).and_then(check_even)
}
```

### Rust (? operator — sequential style)

```rust
pub fn validate_question_mark(s: &str) -> Result<i64, String> {
    let n = parse_int(s)?;
    let n = check_positive(n)?;
    check_even(n)
}
```

### Rust (explicit bind — mirrors OCaml's >>= directly)

```rust
fn bind<T, U, E>(r: Result<T, E>, f: impl FnOnce(T) -> Result<U, E>) -> Result<U, E> {
    match r {
        Err(e) => Err(e),
        Ok(x) => f(x),
    }
}

pub fn validate_explicit(s: &str) -> Result<i64, String> {
    bind(bind(parse_int(s), check_positive), check_even)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Result type | `('a, 'b) result` | `Result<T, E>` |
| Bind operator | `val (>>=) : ('a,'e) result -> ('a -> ('b,'e) result) -> ('b,'e) result` | `fn and_then<U, F>(self, f: F) -> Result<U, E>` |
| Validate signature | `val validate : string -> (int, string) result` | `fn validate(s: &str) -> Result<i64, String>` |
| Error conversion | `"Not an integer: " ^ s` (string concat) | `format!("Not an integer: {s}")` + `map_err` |
| Short-circuit | `Error _ as e -> e` in `>>=` | Implicit in `and_then` / early return via `?` |

## Key Insights

1. **`and_then` is `>>=`:** Rust's `Result::and_then` is the stdlib bind combinator — no custom operator needed. It unwraps `Ok` and passes the value to the next function, or passes `Err` through unchanged.

2. **`?` is do-notation sugar:** The `?` operator desugars to a match on `Result` that early-returns `Err`. Sequential `?` usage gives the same left-to-right chaining as `>>=` but looks like ordinary imperative code.

3. **Error type uniformity:** OCaml's polymorphic `result` lets you mix error types freely. Rust requires all steps in a chain to share the same `E` — here `String`. `map_err` is the idiomatic adapter when upstream errors differ.

4. **First failure wins:** In both languages the chain stops at the first `Err`/`Error`. `parse_int "abc"` never reaches `check_positive` or `check_even`; the error message reflects exactly which step failed.

5. **Railway metaphor:** Each validation function is a railway switch — the "success track" (`Ok`) continues forward; the "error track" (`Err`) bypasses all remaining steps and exits at the end. This pattern makes error handling compositional and centralized.

## When to Use Each Style

**Use `and_then` chain when:** The validators are already written as standalone functions and you want a point-free, functional pipeline that reads like a sentence.

**Use `?` operator when:** The validation logic is complex, needs intermediate let-bindings, or benefits from the familiar sequential look — particularly inside a function body with other logic.

**Use explicit `bind` when:** You are teaching the monad concept or need to abstract over multiple monadic types (e.g., building a generic combinator library).
