# Typed Error Hierarchy — Comparison

## Core Insight
Large applications need structured errors. Both languages use nested enums/variants, but Rust's `From` trait eliminates the manual lifting that OCaml requires.

## OCaml Approach
- Nested variant types: `type app_error = Db of db_error | Auth of auth_error`
- Manual lifting at each boundary: `Error (Db e)` / `Error (Auth e)`
- Individual `string_of_*` functions for display
- No standard trait for error composition

## Rust Approach
- Same enum nesting pattern: `enum AppError { Db(DbError), Auth(AuthError) }`
- `From` impls automate lifting via `?`
- `Display` + `Error` traits provide standard formatting
- `source()` method chains subsystem errors

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Hierarchy | Nested variants | Nested enums |
| Lifting | Manual `Error (Db e)` | Automatic via `From` + `?` |
| Display | `string_of_*` functions | `impl Display` |
| Exhaustiveness | Yes (match) | Yes (match) |
| Source chain | Manual | `Error::source()` |
| Boilerplate | Medium (lifting) | Medium (`From` impls) |
