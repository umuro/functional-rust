# 421: include! and include_str!

**Difficulty:** 2  **Level:** Intermediate

Embed file contents as string or byte literals at compile time — SQL queries, HTML templates, Wasm bytecode — with zero runtime I/O.

## The Problem This Solves

Hard-coding multi-line strings inline makes code noisy and hard to maintain. Keeping them in separate files is cleaner, but reading files at runtime adds failure modes: the file might be missing, the path might be wrong, the read might fail. You also lose the performance of having the data baked into the binary.

For assets that never change between builds — SQL schemas, config templates, embedded certificates — you want the simplicity of a separate file combined with the reliability of a compile-time constant. You also want the IDE to recognise the string as a literal so it participates in dead-code and const-folding optimisations.

`include_str!` and `include_bytes!` solve exactly this: the compiler reads the file during compilation and replaces the macro call with a `&'static str` or `&'static [u8]` literal. If the file is missing, the build fails — not the running program.

## The Intuition

Think of `include_str!("query.sql")` as copy-paste that the compiler does for you at build time. The SQL file stays readable and editable on disk; the binary acts as if you typed the string inline. The path is relative to the source file, so it travels with the crate.

`include_bytes!` does the same for binary files: Wasm modules, compiled shaders, TLS certificates, small images. The result is a `&'static [u8]` — zero heap allocation, zero runtime loading.

## How It Works in Rust

```rust
// In a real project these reference actual files:
// const QUERY: &str = include_str!("queries/users.sql");
// static WASM: &[u8] = include_bytes!("module.wasm");

// Path is relative to the current source file, not the crate root.
// If the file is missing, you get a COMPILE ERROR, not a runtime panic.

const SQL_QUERY: &str =
    "SELECT id, name, email FROM users WHERE active = true ORDER BY name";

// include_bytes! gives a byte array — good for binary data
static EMBEDDED_DATA: &[u8] = &[0x52, 0x75, 0x73, 0x74]; // "Rust"

// Use the constant anywhere — it's just a &'static str
fn render(template: &str, title: &str) -> String {
    template.replace("{{title}}", title)
}
```

**Key mechanics:**
- `include_str!` — UTF-8 text → `&'static str`
- `include_bytes!` — raw bytes → `&'static [u8]`
- `include!` — includes a Rust source file (for code generation output)
- The `CARGO_MANIFEST_DIR` env var is handy when build scripts generate the included file

## What This Unlocks

- **Embedded SQL / GraphQL schemas** — keep queries in `.sql` files, use them as constants at zero runtime cost.
- **Embedded HTML / config templates** — ship a self-contained binary that renders pages without touching the filesystem.
- **Binary asset embedding** — Wasm modules, compiled shaders, certificates embedded directly in the binary; no installation step required.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Embed file at compile time | Not built-in; use `ppx` or manual `read_file` at startup | `include_str!` / `include_bytes!` — standard macros, no deps |
| Missing file | Runtime error (if read at startup) | Compile error |
| Result type | `string` (heap) loaded at runtime | `&'static str` / `&'static [u8]` — baked into binary |
| Binary files | Read with `Bytes.create` at runtime | `include_bytes!` → `&'static [u8]` |
