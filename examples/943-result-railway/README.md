[result-railway on hightechmind.io](https://hightechmind.io/posts/functional-rust/result-railway)

---

## Problem Statement

Implement railway-oriented programming with Rust's `Result` type. Chain fallible operations using `and_then` (monadic bind) and the `?` operator so that the first error short-circuits the rest of the pipeline. Build a concrete pipeline: parse a string to integer, validate it is positive, then compute its square root. Compare the combinator-based approach with the `?`-operator style.

## Learning Outcomes

- Use `Result<T, E>` as a railway: `Ok` stays on the happy path, `Err` short-circuits to the error track
- Chain fallible operations with `and_then` — equivalent to OCaml's `>>=` on `result`
- Apply the `?` operator for ergonomic short-circuiting inside functions that return `Result`
- Map over success values with `.map()` without touching the error type
- Understand that `and_then(f)` and `let n = x?; f(n)` are semantically equivalent

## Rust Application

```rust
pub fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|_| format!("not an integer: {:?}", s))
}

pub fn positive(x: i32) -> Result<i32, String> {
    if x > 0 { Ok(x) } else { Err(format!("{} is not positive", x)) }
}

pub fn sqrt_safe(x: i32) -> Result<f64, String> {
    positive(x).map(|n| (n as f64).sqrt())
}

// Combinator style — mirrors OCaml's >>= chain
pub fn process_bind(s: &str) -> Result<f64, String> {
    parse_int(s).and_then(positive).and_then(sqrt_safe)
}

// ? operator style — idiomatic Rust
pub fn process(s: &str) -> Result<f64, String> {
    let n = parse_int(s)?;
    let n = positive(n)?;
    let result = sqrt_safe(n)?;
    Ok(result)
}
```

Both `process_bind` and `process` are semantically identical. The `?` operator desugars to `match result { Ok(v) => v, Err(e) => return Err(e.into()) }`, giving imperative-looking code with monadic semantics.

`map_err` converts the `ParseIntError` into a `String` so that the entire pipeline has a uniform error type. When different stages produce different error types, define an enum or use `Box<dyn Error>`.

## OCaml Approach

```ocaml
let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (Printf.sprintf "not an integer: %s" s)

let positive x =
  if x > 0 then Ok x
  else Error (Printf.sprintf "%d is not positive" x)

let sqrt_safe x =
  Result.bind (positive x) (fun n -> Ok (sqrt (float_of_int n)))

(* Bind chain using >>= *)
let ( >>= ) = Result.bind

let process s =
  parse_int s >>= positive >>= sqrt_safe

(* With let* (OCaml 4.08+, requires result.ml ppx or Result.bind) *)
let process_letstar s =
  let* n = parse_int s in
  let* n = positive n in
  Ok (sqrt (float_of_int n))
```

OCaml's `Result.bind` is the direct equivalent of Rust's `and_then`. The `>>=` operator alias makes pipelines read like Haskell do-notation. The `let*` syntax (monadic bind sugar) is the modern OCaml style.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Short-circuit syntax | `?` operator — lightweight, reads like imperative code | `>>=` or `let*` — explicit monadic syntax |
| Combinator | `and_then` | `Result.bind` |
| Map success | `.map(f)` | `Result.map f` |
| Error type | Must be uniform or use `Box<dyn Error>`/`anyhow` | Polymorphic type variable `'e` |
| Interoperability | `?` converts via `From` trait | Manual adaptation needed |

The `?` operator is Rust's answer to the verbosity of explicit `match` on every fallible call. It retains the type-safety of `Result` while reading almost as cleanly as exception-based code.

## Exercises

1. Add a `divide(x, y)` step that returns `Err` when `y == 0` and chain it into the pipeline.
2. Define a custom error enum `ProcessError { Parse, NotPositive, Overflow }` and rewrite the pipeline with it.
3. Use `map_err` to convert `ProcessError` into a human-readable `String` at the boundary.
4. Implement `process_all(inputs: &[&str]) -> Vec<Result<f64, String>>` and then use `partition` to separate successes from failures.
5. Compare `and_then` chain vs `?` operator in terms of generated code using `cargo expand` or by reading the desugared output.
