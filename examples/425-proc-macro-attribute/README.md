📖 **[View on hightechmind.io →](https://hightechmind.io/rust/425-proc-macro-attribute)**

---

# 425: Proc Macro Attribute

## Problem Statement

Attribute macros transform the item they annotate. `#[tokio::main]` rewrites `async fn main()` into a synchronous main that creates a Tokio runtime. `#[actix_web::get("/path")]` registers a handler function with routing metadata. `#[cached]` wraps a function with memoization. These transformations are impossible with derive macros (which only add implementations) or `macro_rules!` (which can't inspect or rewrite existing items). Attribute macros receive both the attribute arguments and the annotated item, enabling full code transformation.

Attribute macros are the mechanism behind framework integration: web routing, async runtime setup, middleware injection, retry logic, and profiling annotations.

## Learning Outcomes

- Understand the attribute proc macro lifecycle: receives `(attr: TokenStream, item: TokenStream)`
- Learn how attribute macros can transform, wrap, or replace the annotated item
- See the difference from derive macros: attribute macros modify existing items, derives add new impls
- Understand how `#[tokio::main]` rewrites async functions using attribute macros
- Learn the `#[proc_macro_attribute]` registration and how arguments are passed

## Rust Application

The `src/lib.rs` demonstrates the conceptual output of attribute macros. A `#[log_calls]` attribute would wrap the function body with entry/exit logging. A `#[retry(3)]` attribute would wrap the body in a retry loop. A `#[cached]` attribute would add a `HashMap` cache around a pure function. Real implementations require a separate proc-macro crate with `syn` for parsing the function signature and `quote!` for generating the wrapper.

## OCaml Approach

OCaml's PPX extensions (`[@attr]` and `[%ext ...]`) serve the attribute macro role. A `[@log_calls]` ppx extension receives the function's AST and can wrap it. `ppxlib`'s `Attribute.declare` creates typed attribute handlers. The `ppx_bench` and `ppx_expect` libraries use this to transform functions with benchmarking and expectation test machinery.

## Key Differences

1. **Arguments**: Rust attribute macros receive arguments as a `TokenStream` and parse them freely; OCaml PPX attributes use a typed declaration system.
2. **Item types**: Rust attribute macros can annotate any item (fn, struct, impl, mod); OCaml PPX extensions are declared for specific AST node types.
3. **Error handling**: Rust macros can call `compile_error!` or return a `TokenStream` with errors; OCaml raises exceptions or uses `Location.error_extensionf`.
4. **Testing**: Rust attribute macro tests use `trybuild` for expected outputs; OCaml uses `ppx_deriving`'s test `expect` tests.

## Exercises

1. **Timing attribute**: Implement `#[timed]` that wraps any function with `let start = Instant::now(); let result = original_body; println!("{}: {:?}", fn_name, start.elapsed()); result`.
2. **Validate input**: Create `#[validate_positive]` for functions taking `i32` that adds an assertion at the start: `assert!(arg > 0, "argument must be positive")`. Handle functions with multiple parameters by validating only the first `i32`.
3. **Deprecated with replacement**: Implement `#[replace_with("new_function_name")]` that emits a deprecation warning `#[deprecated(note = "use new_function_name instead")]` and adds a `log::warn!` call at the start of the function body.
