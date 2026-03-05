## Core Insight

Instead of validating at every use site, validate once at construction and encode the invariant in the type system. A `NonEmptyString` can never be empty — no runtime checks needed downstream.

## OCaml Approach
- Private/abstract types in module signatures
- Constructor function returns `option` or `result`
- Module signature hides the raw constructor

## Rust Approach
- Newtype pattern: `struct NonEmptyString(String)` with private field
- Constructor returns `Result`
- No public access to inner value without going through API

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Hide constructor | Module signature | Private field |
| Validate | `create : string -> t option` | `fn new(s) -> Result<Self>` |
| Access | Getter function | `.as_str()` / `.value()` |
| Guarantee | Type-level | Type-level |
