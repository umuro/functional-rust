📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1021-error-propagation-depth)**

---

# 1021-error-propagation-depth — Error Propagation Depth

## Problem Statement

Real applications have deep call stacks: a user request flows through authentication, configuration loading, parsing, validation, and service calls — each of which can fail with a different type of error. Manually handling each error at every call site creates enormous boilerplate and buries the application logic.

Rust's `?` operator enables error propagation across multiple layers with minimal syntax. This example demonstrates a five-level deep pipeline where each layer uses `?` to propagate errors upward, all while preserving type safety and the ability to pattern-match on specific errors at the top level.

## Learning Outcomes

- Compose five layers of fallible functions using `?`
- Design a unified `AppError` enum that wraps errors from all subsystems
- Understand how `From` implementations enable `?` across error type boundaries
- Trace an error from the innermost layer to the outermost handler
- Know the trade-offs between a unified error enum and `Box<dyn Error>`

## Rust Application

`src/lib.rs` defines five functions: `read_config`, `parse_port`, `validate_port`, `check_service`, and `startup`. Each returns `Result<_, AppError>` and uses `?` to propagate failures. The `AppError` enum has variants for each failure mode: `ConfigMissing`, `ParseFailed`, `ValidationFailed`, `ServiceUnavailable`, and `Timeout`. `startup` chains all five with `?` in a readable linear sequence.

The `Display` impl on `AppError` enables human-readable error messages. Pattern matching in tests shows that specific error variants can be inspected at the top level.

## OCaml Approach

OCaml achieves the same effect with `let*` and a unified error type:

```ocaml
let ( let* ) = Result.bind

let startup () =
  let* config = read_config "port" in
  let* port = parse_port config in
  let* _ = validate_port port in
  check_service port
```

Each `let*` short-circuits on `Error`. Unlike Rust, OCaml does not require `From` impls because the error type is unified at the module boundary and OCaml's structural type system handles matching.

## Key Differences

1. **`From` requirement**: Rust's `?` requires `From<SourceError> for AppError`; OCaml's `let*` requires the error type to already be the same.
2. **Error enum exhaustiveness**: Rust's `match` on `AppError` is checked exhaustively at compile time; OCaml pattern matching is also exhaustive but the type system is structurally typed.
3. **Five layers of `?`**: Rust reads left-to-right linearly with `?` at each step; OCaml reads top-to-bottom with `let*` bindings.
4. **Type-level error info**: Rust's typed `AppError` retains the failure category in the type; `Box<dyn Error>` erases it but is easier to compose.

## Exercises

1. Add a sixth layer `log_startup(port: u16) -> Result<(), AppError>` that simulates a logging failure and chain it into `startup`.
2. Refactor the example to use `anyhow::Result` and `.context()` instead of a typed `AppError`. Compare readability and the loss of pattern-matching ability.
3. Write an integration test that calls `startup` with various failing configurations and asserts the specific `AppError` variant returned.
