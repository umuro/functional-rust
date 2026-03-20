📖 **[View on hightechmind.io →](https://hightechmind.io/rust/421-macro-include)**

---

# 421: `include!`, `include_str!`, `include_bytes!`
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Binary assets (images, shaders, SQL queries, configuration files) can be embedded directly into executables. Instead of loading them from disk at runtime (which can fail if the file is missing or the path changes), `include_bytes!` and `include_str!` embed the file content as a compile-time constant. The resulting binary is self-contained: no external files needed, no file-not-found errors at runtime. `include!` includes arbitrary Rust source files, enabling code generation workflows where a `build.rs` produces Rust code that is then `include!`'d.

These macros are used by `rust-embed`, WASM binary bundlers, shader compilers, and any application embedding resources (game assets, localization strings, TLS certificates).

## Learning Outcomes

- Understand how `include_bytes!` embeds a file as `&'static [u8]` at compile time
- Learn how `include_str!` embeds a UTF-8 file as `&'static str`
- See how `include!` includes generated Rust source code from `build.rs` workflows
- Understand how Cargo tracks file dependencies via `println!("cargo:rerun-if-changed=...")`
- Learn the trade-off: larger binary vs. zero runtime file I/O errors

## Rust Application

In `src/lib.rs`, `include_bytes!("../Cargo.toml")` embeds the Cargo.toml as a `&[u8]` byte slice. The `include_sql!` macro uses `concat!` to build SQL query strings at compile time. The path in `include_bytes!` is relative to the source file's location, resolved by the compiler. For `include_str!`, the file must be valid UTF-8. Cargo automatically rebuilds when included files change.

## OCaml Approach

OCaml achieves file embedding through the `dune` build system's `(embed_file ...)` rule or through the `sedlex` and `menhir` grammar inclusion patterns. The `Crunch` library converts files into OCaml modules. For inline SQL, `ppx_rapper` generates type-safe queries from SQL strings in OCaml source. OCaml has no built-in equivalent of `include_bytes!` — file embedding requires build system configuration.

## Key Differences

1. **Built-in**: Rust's `include_bytes!` and `include_str!` are language primitives; OCaml requires build system configuration or external libraries.
2. **Path resolution**: Rust resolves paths relative to the source file; OCaml's approaches resolve relative to the dune build directory.
3. **Rebuild tracking**: Cargo automatically re-builds when included files change; OCaml requires explicit `(deps ...)` in dune rules.
4. **Code inclusion**: Rust's `include!` includes arbitrary Rust code; OCaml's `load_path` and dynamic loading serve similar purposes but differently.

## Exercises

1. **Embedded config**: Use `include_str!` to embed a TOML config file into your binary. Parse it at startup using the `toml` crate and expose configuration values as constants. Show that the binary works without the config file being present at runtime.
2. **Shader embedding**: Embed a GLSL vertex and fragment shader as `include_str!` constants. Create a `ShaderSource { vert: &'static str, frag: &'static str }` and verify the strings contain valid GLSL keywords.
3. **Code generation**: Write a `build.rs` that reads a `data/keywords.txt` file and generates `src/generated.rs` containing `pub const KEYWORDS: &[&str] = &[...];`. Use `include!("../src/generated.rs")` to include the generated code.
