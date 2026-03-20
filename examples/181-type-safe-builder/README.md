📖 **[View on hightechmind.io →](https://hightechmind.io/rust/181-type-safe-builder)**

---

# Type-Safe SQL-like Query Builder
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A SQL query builder that allows calling `.where_()` before `.from()`, or calling `.build()` without a `SELECT` clause, is a footgun — the error appears at runtime when the query fails, not at the API call site. Type-safe builders use typestate to enforce calling order at compile time: `build()` is only available when all required clauses have been provided. This pattern appears in HTTP clients (`reqwest`), ORMs (`diesel`), and configuration APIs.

## Learning Outcomes

- Apply typestate to a builder pattern enforcing required steps in a specific order
- Use multiple phantom type parameters (`S`, `F`, `W`) to track independent requirements
- See how consuming `self` in each builder method enforces a linear construction flow
- Understand the practical impact: "call build without FROM" is a compile error, not a runtime error

## Rust Application

`Query<S, F, W>` uses three phantom types: `S` (select status: `NoSelect` | `HasSelect`), `F` (from status), `W` (where status). Each setter method consumes `self` and returns a `Query` with the updated phantom: `select(self) -> Query<HasSelect, F, W>`. `build()` is implemented only on `Query<HasSelect, HasFrom, _>` — calling it without SELECT or FROM is a compile error. Partial builders can be stored and continued: `let q = Query::new().select("id")` has type `Query<HasSelect, NoFrom, NoWhere>`.

## OCaml Approach

OCaml's phantom type approach:
```ocaml
type ('s, 'f) query = { select_: string option; from_: string option }
let select q s = { q with select_ = Some s }
let from q t = { q with from_ = Some t }
let build : (has_select, has_from) query -> string = fun q -> ...
```
State transitions via phantom types work similarly in OCaml, but without move semantics — the old query value remains accessible after each builder step. Rust's consuming transitions are stricter.

## Key Differences

1. **Multiple state dimensions**: Multiple phantom parameters independently track each required clause — orthogonal state dimensions; OCaml achieves this similarly.
2. **Move on step**: Rust consumes the builder at each step (preventing accidental use of intermediate states); OCaml retains old values.
3. **Optional clauses**: The `W` phantom allows `build()` with or without a `WHERE` clause — optional requirements are captured by accepting any `W`.
4. **Error messages**: Rust's compile error "method `build` not found" is clear; adding `#[doc]` to the phantom-gated method improves discoverability.

## Exercises

1. Add an `order_by` clause as an optional phantom state `HasOrderBy | NoOrderBy`.
2. Implement `limit(self, n: u32) -> Query<S, F, W>` (doesn't change type state — always optional).
3. Add a validation step in `build()` that checks for SQL injection patterns in string fields.
