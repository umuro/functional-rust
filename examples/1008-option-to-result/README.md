📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1008-option-to-result)**

---

# 1008-option-to-result — Option to Result

## Problem Statement

`Option<T>` represents the presence or absence of a value, while `Result<T, E>` represents success or a specific failure reason. Real programs often start with an `Option` lookup — for example, reading a key from a map — and need to convert it into a `Result` that can carry an error message and propagate through the `?` operator.

Rust provides two conversion methods: `ok_or` (eager, always constructs the error value) and `ok_or_else` (lazy, only constructs the error if `None` is encountered). This distinction matters for performance when the error value is expensive to build.

## Learning Outcomes

- Understand when to use `Option` versus `Result` for absent values
- Convert `Option<T>` to `Result<T, E>` using `ok_or` and `ok_or_else`
- Chain `Option`-to-`Result` conversions inside `?`-based pipelines
- Recognise the cost difference between eager and lazy error construction
- Go the other direction: `Result::ok()` to collapse `Result` back to `Option`

## Rust Application

`src/lib.rs` uses a `HashMap<String, (String, u32)>` as a user store. `find_user_eager` calls `.ok_or(format!(...))` — the format string allocates even on the happy path. `find_user_lazy` calls `.ok_or_else(|| format!(...))` — the closure runs only when the value is `None`. `find_and_validate` chains both conversion and a subsequent `and_then` validation step, showing how `Option`-to-`Result` conversion fits naturally into a larger pipeline.

## OCaml Approach

OCaml has no `ok_or` method on `Option`, but the pattern is one line:

```ocaml
let option_to_result opt msg =
  match opt with
  | Some v -> Ok v
  | None -> Error msg
```

Libraries like `Base` provide `Option.value_exn` and `Option.to_or_error`. The lazy variant is expressed with a thunk: `None -> Error (msg ())`.

## Key Differences

1. **Built-in conversion methods**: Rust has `ok_or` / `ok_or_else` as inherent methods; OCaml requires manual matching or a library helper.
2. **Lazy vs eager**: Both languages can express lazy error construction, but Rust makes the distinction explicit with two differently named methods.
3. **`?` integration**: Rust's `?` can be applied directly to `Result` after conversion; OCaml's equivalent (`let*`) requires the result to already be in the monad.
4. **Null safety**: Neither language has null; both use sum types for optional values, ensuring exhaustiveness at compile time.

## Exercises

1. Add a `find_user_by_email` function that searches the map by email value instead of key, returning `Option<&str>` converted to `Result`.
2. Write a function that chains `find_and_validate` for multiple names, collecting all successful results into a `Vec`.
3. Implement the reverse direction: a function that takes a `Result<User, String>` and maps it to `Option<User>`, discarding the error reason.
