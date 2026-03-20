📖 **[View on hightechmind.io →](https://hightechmind.io/rust/532-lifetime-multiple)**

---

# Multiple Lifetime Parameters
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Most introductory lifetime examples use a single `'a` for all borrows. But real code often has references with genuinely independent lifetimes — a function that reads from one buffer and writes to another, a struct holding a reader and a writer that may live for different durations. Using a single lifetime in these cases would over-constrain the API: callers would need to keep all referenced data alive for the same duration. Multiple independent lifetime parameters express the true dependency relationships and give callers maximum flexibility.

## Learning Outcomes

- Why `first_of<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str` uses two lifetimes instead of one
- How `struct Pair<'a, 'b>` with independent field lifetimes enables flexible use
- How `impl<'a, 'b> Pair<'a, 'b>` methods can return references tied to specific fields
- How `Context<'r, 'w>` models independent reader (`'r`) and writer (`'w`) lifetimes
- When to use lifetime subtyping (`'long: 'short`) to express ordering constraints

## Rust Application

`first_of<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str` has two lifetime parameters — the output is tied only to `'a`, so `y` can have a shorter lifetime without restricting the result. `Pair<'a, 'b>` stores `first: &'a str` and `second: &'b str`; `get_first` returns `&'a str` and `get_second` returns `&'b str` — each method correctly names which field it returns from. `Context<'r, 'w>` separates the read-only (`'r`) and mutable write (`'w`) lifetimes, allowing them to be independently scoped.

Key patterns:
- `<'a, 'b>` — two independent lifetime parameters
- `fn get_first(&self) -> &'a str` — return tied to field lifetime, not `self` lifetime
- `'long: 'short` — lifetime subtyping: `'long` outlives `'short`

## OCaml Approach

OCaml has no lifetime parameters — all borrows are managed by the GC. A record with two string references needs no annotation:

```ocaml
type pair = { first: string; second: string }
let get_first p = p.first   (* no lifetime annotation needed *)
let get_second p = p.second
```

Multiple independent reference lifetimes are a Rust-specific concept; OCaml programs never express or reason about them.

## Key Differences

1. **Lifetime independence**: Rust requires separate `'a` and `'b` when two references can have different scopes; OCaml has no such distinction — the GC ensures both are valid as long as needed.
2. **Return source tracing**: Rust methods that return references must specify which field they return from via the lifetime; OCaml methods return values with no annotation on where they came from.
3. **API flexibility**: Two-lifetime Rust APIs are strictly more flexible for callers than single-lifetime APIs; OCaml APIs are uniformly flexible because all values are GC-managed.
4. **Lifetime subtyping**: Rust `'long: 'short` expresses outlives relationships as a compile-time constraint; OCaml has no equivalent — the runtime guarantee is unconditional.

## Exercises

1. **Three-lifetime function**: Write `fn combine<'a, 'b, 'c>(x: &'a str, y: &'b str, sep: &'c str) -> String` where the output owns its data (not tied to any input lifetime).
2. **Independent reader/writer**: Implement `struct Log<'data, 'label> { entries: &'data [String], prefix: &'label str }` with a method that formats entries using the prefix, returning an owned `String`.
3. **Lifetime subtyping demo**: Write `fn coerce<'long: 'short, 'short>(x: &'long str) -> &'short str { x }` and explain in a comment why this compiles — `'long` can be used where `'short` is expected.
