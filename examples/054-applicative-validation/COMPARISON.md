## Core Insight

Monadic (bind/and_then) error handling stops at the first error. Applicative validation runs all validations and collects every error. This is critical for form validation UX.

## OCaml Approach
- Define a custom `validated` type: `Valid of 'a | Invalid of 'e list`
- Combine validators by accumulating error lists
- No built-in applicative — manual implementation

## Rust Approach
- Collect errors into `Vec<String>` or custom error enum
- Use iterator + partition or fold to gather all results
- Libraries like `validator` exist, but std approach works fine

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Fail-fast | `Result` + `bind` | `Result` + `?` |
| Collect all | Custom `validated` type | `Vec<Error>` accumulation |
| Combine | Manual applicative | `.iter().filter_map()` |
