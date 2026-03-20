📖 **[View on hightechmind.io →](https://hightechmind.io/rust/536-lifetime-static)**

---

# 'static Lifetime

## Problem Statement

`'static` is the longest possible lifetime in Rust — it means "valid for the entire program duration." String literals embedded in the binary are `'static str`. Global constants and statics are `'static`. The `'static` bound on trait objects (`Box<dyn Trait + 'static>`) means the type contains no non-static references — it can be sent across thread boundaries and stored in long-lived data structures. Understanding `'static` is essential for thread-safe data sharing, global configuration, error type design, and trait object storage.

## Learning Outcomes

- What `'static` means: the reference is valid for the entire program's lifetime
- How string literals (`"hello"`) are `&'static str` — embedded in the binary
- How owned types (`String`, `Vec<T>`) satisfy `'static` bounds because they have no borrows
- How `'static` bounds on trait objects (`Box<dyn Error + 'static>`) enable thread-safe error handling
- Where `'static` is required: `thread::spawn`, global statics, `Box<dyn Any + 'static>`

## Rust Application

`APP_NAME: &'static str` and `ERROR_MESSAGES: &[(u16, &str)]` are program-wide statics. `get_error_msg(code)` returns `&'static str` — a reference to program binary data. `get_greeting()` returns `"Hello, World!"` typed as `&'static str`. `make_static_string()` returns a `String` — not `&'static str` — but `String` still satisfies `'static` trait bounds because it owns its data with no borrows. `store_static(s: &'static str)` demonstrates that only `'static` string references can be stored in this position.

Key patterns:
- `static FOO: &str = "..."` — the static variable itself is `'static`
- `fn get() -> &'static str` — returning a reference to program-lifetime data
- `T: 'static` bound: type contains no non-static references (owned types qualify)

## OCaml Approach

OCaml has no `'static` concept. All values are GC-managed and valid as long as any reference exists. Global values are declared with `let` at module scope:

```ocaml
let app_name = "MyApp"            (* module-level, available everywhere *)
let error_messages = [(404, "Not Found"); (500, "Internal Server Error")]
```

The GC ensures these are always valid — there is no concept of a lifetime expiring.

## Key Differences

1. **Binary embedding**: Rust `&'static str` literals are embedded in the binary's read-only data segment; OCaml string literals are GC-allocated heap objects (though the compiler may intern them).
2. **Thread safety**: Rust `'static` bound on `thread::spawn`'s closure ensures no borrowed references cross the thread boundary; OCaml's GC manages cross-thread safety through domain locks in OCaml 5.x.
3. **Owned data and 'static**: In Rust, `String: 'static` because it has no borrows — a subtle but important distinction between "is `'static` data" and "satisfies `'static` bound"; OCaml has no equivalent.
4. **Error types**: Rust `Box<dyn std::error::Error + 'static>` is the standard owned error type; OCaml exceptions are values of type `exn` with no lifetime tracking.

## Exercises

1. **Static dispatch table**: Define a `static OPS: &[(&str, fn(i32) -> i32)]` at module level mapping names to operations, and write a lookup function returning `Option<fn(i32) -> i32>`.
2. **'static bound function**: Write `fn store_in_vec<T: 'static>(v: &mut Vec<Box<dyn std::any::Any>>, item: T)` and verify it rejects types with non-static borrows.
3. **Lazy static config**: Use `std::sync::OnceLock<&'static str>` to store a program-global greeting initialized from a command-line argument on first access.
