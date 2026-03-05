# Error Context — Comparison

## Core Insight
Raw errors like "file not found" are useless without knowing which file, in which operation, at which layer. Context wrapping builds a breadcrumb trail.

## OCaml Approach
- Record with `context: string list` accumulates breadcrumbs
- Custom `>>|` operator adds context in pipelines
- No standard library support — each project rolls its own

## Rust Approach
- Wrapper struct with `Vec<String>` context chain
- Extension trait `Context` on `Result` — `.context("msg")?`
- Lazy variant: `.with_context(|| format!(...))?`
- Real-world: `anyhow::Context` trait does exactly this

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Context accumulator | `string list` field | `Vec<String>` field |
| Adding context | Custom `>>|` operator | `.context()` trait method |
| Lazy context | Thunk `fun () -> ...` | Closure `\|\| format!(...)` |
| Standard library | No | No (but `anyhow` is de facto standard) |
| Display format | Manual `String.concat` | Custom `Display` impl |
