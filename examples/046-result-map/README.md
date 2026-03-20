📖 **[View on hightechmind.io →](https://hightechmind.io/rust/046-result-map)**

---

# 046 — Result Map

## Problem Statement

`Result::map` transforms the success value inside `Ok` while leaving `Err` unchanged — the functor operation for `Result`. It is the equivalent of `Option::map` but for the two-channel success/failure type. Together with `map_err`, it enables transformation of both channels: `map` for the success value, `map_err` for the error value.

This pattern is essential for adapting between different result types in a pipeline. A library returns `Result<i32, LibError>`; your code needs `Result<String, AppError>`. Use `result.map(|n| n.to_string()).map_err(AppError::from_lib)` to adapt both sides without unwrapping.

## Learning Outcomes

- Use `result.map(|v| transform(v))` to transform the `Ok` value
- Use `result.map_err(|e| convert(e))` to transform the `Err` value
- Chain `map` calls for sequential transformations on the success path
- Use `map` to add context: `result.map(|v| (v, metadata))`
- Understand that `map` is equivalent to `and_then(|v| Ok(transform(v)))`

## Rust Application

`result.map(|x| x * 2)` doubles the inner value on success. `result.map(|x| x.to_string())` converts types. `result.map_err(|e| format!("error: {}", e))` converts error types. Chaining: `parse_int(s).map(|n| n * 2).map(|n| n.to_string())`. The error type must be consistent through a `map` chain — use `map_err` to normalize before chaining `map` calls.

## OCaml Approach

OCaml's `Result.map f r`: `let map f = function Ok x -> Ok (f x) | Error e -> Error e`. `Result.map_error f r` maps the error. Pipe style: `parse_int s |> Result.map (fun n -> n * 2) |> Result.map string_of_int`. OCaml 4.08+ provides `Result.map` and `Result.map_error`. Earlier: use pattern matching or define them yourself.

## Key Differences

1. **`map_err` naming**: Rust: `map_err`. OCaml: `Result.map_error`. The same operation — transforms the `Err`/`Error` branch.
2. **Consuming vs borrowing**: Rust's `result.map(f)` consumes the result by value. Use `result.as_ref().map(f)` to map over `Result<&T, &E>` without consuming. OCaml's GC handles this transparently.
3. **Type inference**: Both infer the output type of the mapped function. Rust requires explicit `map_err` call to change error types; OCaml's structural typing can sometimes infer error type changes without explicit conversion.
4. **`map` vs `and_then`**: `map` cannot fail — the function returns `U`, not `Result<U, E>`. If the transformation can fail, use `and_then` instead.

## Exercises

1. **Double map**: Write `transform(r: Result<i32, String>) -> Result<String, String>` that doubles the int and converts to string on success, and prepends "Error: " to error messages. Use both `map` and `map_err`.
2. **Normalization pipeline**: Given a `Result<&str, IoError>`, write a chain that trims whitespace, parses as float, multiplies by 100, and rounds to int. Each step uses `map`.
3. **Split channels**: Given `Vec<Result<i32, String>>`, use `partition` to split into `(Vec<i32>, Vec<String>)` of successes and errors. This is the `partition_result` pattern.
