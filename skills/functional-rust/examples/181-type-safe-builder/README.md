# 181: Type-Safe SQL-Like Query Builder

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Use phantom type states to enforce that a query builder is used in the right order — `SELECT` before `FROM`, `FROM` before `WHERE` — turning protocol violations into compile-time errors.

## The Problem This Solves

Builder APIs have ordering requirements. A SQL query needs `SELECT` before `FROM`. An HTTP request needs a URL before headers. A test fixture needs setup before assertions. The typical builder pattern accepts calls in any order and validates at `.build()` time — which means errors surface only at runtime, possibly in production, possibly intermittently when the code path that constructs a malformed query is hit.

The subtler issue: a runtime-validated builder forces every consumer to handle `Result<Query, BuildError>`. Half your callers know the query is valid — they're using the builder correctly — but they still have to write `.unwrap()` or propagate an error they didn't cause. The type system has let them down by treating correct usage and incorrect usage identically.

Type-state builders solve this by making each stage of the builder a distinct type. After calling `.select(...)` you hold a `Query<HasSelect, NoFrom, NoWhere>`. After `.from(...)` you hold a `Query<HasSelect, HasFrom, NoWhere>`. The `.build()` method only exists when all required stages are complete — and it returns `Query` directly, not `Result<Query, Error>`, because correctness is guaranteed.

## The Intuition

A passport application form has required sections. The clerk won't stamp "received" until you've filled in name, date of birth, *and* nationality. Each section you complete moves you forward in a process. You can't go to the "photo attached" step without the basics done first.

In Rust, each completed stage is a type parameter. `PhantomData<(HasSelect, HasFrom, NoWhere)>` is a zero-sized triple that records exactly what you've done. The compiler reads it. When you call `.build()`, the bound says `S: IsReady` — and `IsReady` is only implemented for the state where all required fields are set.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// Stage marker types — zero-sized
struct NoSelect;  struct HasSelect;
struct NoFrom;    struct HasFrom;
struct NoWhere;   struct HasWhere;

struct QueryBuilder<S, F, W> {
    select_clause: Option<String>,
    from_clause:   Option<String>,
    where_clause:  Option<String>,
    _state: PhantomData<(S, F, W)>,
}

// Start: no stages set
impl QueryBuilder<NoSelect, NoFrom, NoWhere> {
    fn new() -> Self {
        QueryBuilder {
            select_clause: None,
            from_clause: None,
            where_clause: None,
            _state: PhantomData,
        }
    }
}

// select() transitions NoSelect -> HasSelect
impl<F, W> QueryBuilder<NoSelect, F, W> {
    fn select(self, cols: &str) -> QueryBuilder<HasSelect, F, W> {
        QueryBuilder {
            select_clause: Some(cols.to_string()),
            from_clause: self.from_clause,
            where_clause: self.where_clause,
            _state: PhantomData,
        }
    }
}

// from() requires HasSelect (you can't have FROM without SELECT)
impl<W> QueryBuilder<HasSelect, NoFrom, W> {
    fn from(self, table: &str) -> QueryBuilder<HasSelect, HasFrom, W> {
        QueryBuilder {
            select_clause: self.select_clause,
            from_clause: Some(table.to_string()),
            where_clause: self.where_clause,
            _state: PhantomData,
        }
    }
}

// build() only exists when both SELECT and FROM are set — returns String, not Result
impl<W> QueryBuilder<HasSelect, HasFrom, W> {
    fn build(self) -> String {
        let mut sql = format!("SELECT {} FROM {}",
            self.select_clause.unwrap(),
            self.from_clause.unwrap());
        if let Some(w) = self.where_clause {
            sql.push_str(&format!(" WHERE {}", w));
        }
        sql
    }
}

// Correct usage — builds cleanly with no Result:
let sql = QueryBuilder::new()
    .select("id, name")
    .from("users")
    .build(); // returns String directly

// This fails to compile:
// QueryBuilder::new().from("users").build();
//                    ^^^^  error: method `from` not found in `QueryBuilder<NoSelect, ...>`
```

## What This Unlocks

- **HTTP client builders** — enforce that a URL is set before headers, headers before body, body before `.send()` — no more `MissingUrl` errors at runtime.
- **Configuration builders** — required fields become type-state stages; `Config::build()` is infallible because the type guarantees completeness.
- **State machine APIs** — authentication flows, checkout funnels, multi-step wizards — each step advances the type, making it impossible to skip required stages.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| State encoding | Phantom type parameters on a record, e.g. `type ('s, 'f, 'w) builder` | `PhantomData<(S, F, W)>` triple, or separate marker traits |
| Method gating | Functions accept only specific phantom type combinations | `impl` blocks scoped to specific type parameter combinations |
| Style | Pipeline `\|>` with typed intermediate values | Method chaining; each call returns a new typed builder |
| Error quality | "Type mismatch" with phantom types in error message | "method not found in `QueryBuilder<NoSelect, ...>`" — very readable |
| Zero-cost | Yes | Yes — PhantomData is erased entirely at runtime |
