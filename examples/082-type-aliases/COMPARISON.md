## Core Insight

Type aliases give shorter names to complex types. They're transparent — the compiler treats them as identical to the original. Useful for Result types, complex generics, and documentation.

## OCaml Approach
- `type 'a my_result = ('a, error) result`
- Aliases are fully transparent
- Can also use `type t = int` for simple aliases

## Rust Approach
- `type Result<T> = std::result::Result<T, MyError>;`
- Common in library APIs (`io::Result<T>`)
- No new type created — just a name

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Syntax | `type alias = original` | `type Alias = Original;` |
| Transparent | Yes | Yes |
| Generic | `type 'a t = ...` | `type T<A> = ...` |
| New type? | No | No |
