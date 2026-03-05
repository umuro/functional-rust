# OCaml vs Rust: Result.bind and Result.map — Error Handling Pipeline

## Side-by-Side Code

### OCaml

```ocaml
let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error ("Not a number: " ^ s)

let check_positive n =
  if n > 0 then Ok n else Error "Must be positive"

let check_range n =
  if n <= 100 then Ok n else Error "Must be <= 100"

let validate s =
  parse_int s
  |> Result.bind check_positive
  |> Result.bind check_range
  |> Result.map (fun n -> n * 2)
```

### Rust (idiomatic — combinator chain)

```rust
pub fn validate(s: &str) -> Result<i64, String> {
    parse_int(s)
        .and_then(check_positive)
        .and_then(check_range)
        .map(|n| n * 2)
}
```

### Rust (functional — `?` operator)

```rust
pub fn validate_question_mark(s: &str) -> Result<i64, String> {
    let n = parse_int(s)?;
    let n = check_positive(n)?;
    let n = check_range(n)?;
    Ok(n * 2)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Fallible function | `val parse_int : string -> (int, string) result` | `fn parse_int(s: &str) -> Result<i64, String>` |
| Bind / flatMap | `Result.bind : ('a, 'e) result -> ('a -> ('b, 'e) result) -> ('b, 'e) result` | `fn and_then<U, F>(self, f: F) -> Result<U, E>` |
| Map (infallible) | `Result.map : ('a -> 'b) -> ('a, 'e) result -> ('b, 'e) result` | `fn map<U, F>(self, f: F) -> Result<U, E>` |
| Pipeline operator | `\|>` (reverse-application) | method chaining (`.`) |
| Early-exit syntax | `let* n = expr in` (OCaml 5 syntax extension) | `let n = expr?;` |

## Key Insights

1. **`Result.bind` = `and_then`:** The OCaml `Result.bind check_positive` and the Rust
   `.and_then(check_positive)` are identical in semantics — both pass the success value
   into the next function and propagate errors unchanged.

2. **`|>` vs method chaining:** OCaml's pipe operator `|>` and Rust's `.method()` chain
   achieve the same left-to-right composition. Neither requires naming intermediate values.

3. **`?` is syntactic sugar for bind:** `let n = expr?` desugars to `expr.and_then(|n| …)`
   (roughly). It is not a try/catch — it is a monadic early return that stays purely in
   the type system. OCaml's `let*` in a `Result.bind`-derived monad does the same.

4. **Error type uniformity:** In Rust, all steps in a `?`-chain must produce the *same*
   error type. OCaml enforces this at the type level too, but conversion is more often
   done implicitly via polymorphic variants. In Rust, use `.map_err()` to adapt errors.

5. **No exceptions:** Both OCaml and Rust treat errors as ordinary values in the return
   type. There is no runtime exception mechanism — callers are forced by the type system
   to handle `Error`/`Err` explicitly, unlike languages where exceptions can be silently
   ignored.

## When to Use Each Style

**Use the `and_then` combinator chain when:** the pipeline is purely functional — each
step is a named function passed by reference, and you want a style that mirrors the OCaml
`|> Result.bind` pattern exactly. Also good when the chain is embedded inside a `map` or
composed with other combinators.

**Use the `?` operator when:** the pipeline has several steps and you need to name
intermediate values, perform additional logic between steps, or the `?`-desugaring makes
the control flow clearer to readers who are not familiar with the combinator style.
Both styles compile to identical machine code; choose based on readability.
