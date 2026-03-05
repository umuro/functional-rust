# Comparison: Example 181 — Type-Safe Query Builder

## State-Tracked Builder

### OCaml
```ocaml
let select cols (q : (empty_q, 'f, 'w) query) : (has_select, 'f, 'w) query =
  { q with select_clause = Some cols }

let from table (q : (has_select, empty_q, 'w) query) : (has_select, has_from, 'w) query =
  { q with from_clause = Some table }

let where_ cond (q : (has_select, has_from, empty_q) query) =
  { q with where_clause = Some cond }

(* Usage *)
let sql = empty_query |> select "*" |> from "users" |> where_ "age > 18"
```

### Rust
```rust
impl<F, W> Query<NoSelect, F, W> {
    fn select(self, cols: &str) -> Query<HasSelect, F, W> { /* ... */ }
}
impl<W> Query<HasSelect, NoFrom, W> {
    fn from(self, table: &str) -> Query<HasSelect, HasFrom, W> { /* ... */ }
}
impl Query<HasSelect, HasFrom, NoWhere> {
    fn where_(self, cond: &str) -> Query<HasSelect, HasFrom, HasWhere> { /* ... */ }
}

// Usage
let sql = Query::new().select("*").from("users").where_("age > 18").build();
```

## Compile-Time Error

### OCaml
```ocaml
(* Won't compile: from needs has_select *)
let _ = empty_query |> from "users"
(* Error: This expression has type (empty_q, empty_q, empty_q) query
   but expected (has_select, empty_q, 'w) query *)
```

### Rust
```rust
// Won't compile: no from() on NoSelect
Query::new().from("users");
// Error: no method named `from` found for `Query<NoSelect, NoFrom, NoWhere>`
```
