📖 **[View on hightechmind.io →](https://hightechmind.io/rust/180-phantom-safety)**

---

# PhantomData for API Safety
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Database connections, file handles, and network sockets have a lifecycle: they must be opened before use and closed after use. Calling query methods on a closed connection causes runtime errors. PhantomData-based typestate encodes the connection state in the type: `Connection<Closed>` and `Connection<Open>` are different types, with query methods available only on `Connection<Open>`. Opening a closed connection returns `Connection<Open>`; closing an open connection returns `Connection<Closed>`.

## Learning Outcomes

- Apply the typestate pattern to a real-world resource lifecycle (database connection)
- Understand how phantom types prevent use-after-close and use-before-open bugs at compile time
- See how the same pattern applies to file handles, sockets, and other OS resources
- Appreciate zero-cost typestate: `PhantomData<State>` adds no runtime overhead

## Rust Application

`struct Connection<State> { host: String, _state: PhantomData<State> }` uses `Closed` and `Open` as zero-sized markers. `Connection<Closed>::new(host)` creates a closed connection. `open(self) -> Connection<Open>` transitions to open (consuming the closed connection). `query(&self) -> &str` is only implemented on `Connection<Open>` — calling it on a `Connection<Closed>` is a compile error. `close(self) -> Connection<Closed>` transitions back to closed.

## OCaml Approach

OCaml's phantom type approach:
```ocaml
type closed = Closed
type open_ = Open
type 'state connection = { host: string }
let open_conn (c: closed connection) : open_ connection = c
let close_conn (c: open_ connection) : closed connection = c
let query (c: open_ connection) : string = "result"
```
This works but does not prevent using the old `closed connection` after calling `open_conn` — OCaml's GC keeps the old value alive, so the programmer can accidentally use it. Rust's move semantics make this impossible.

## Key Differences

1. **Move semantics**: Rust's `open(self)` consumes the closed connection — the old binding cannot be used; OCaml retains the old value in scope.
2. **Compile-time prevention**: Rust: using the old closed connection after `open` is a compile error ("value used after move"); OCaml: same value remains accessible.
3. **Linear types**: Rust's ownership system provides a subset of linear types; true linear type languages (Idris, Linear Haskell) are stricter still.
4. **RAII**: Rust can combine typestate with `Drop` to auto-close on drop; OCaml uses finalizers (unreliable for deterministic resource cleanup).

## Exercises

1. Add an `execute(&mut self, sql: &str) -> Result<(), String>` method on `Connection<Open>` that simulates query execution.
2. Implement `Connection<Open>` → `Connection<InTransaction>` → `Connection<Open>` transitions with `begin_transaction`, `commit`, and `rollback` methods.
3. Combine typestate with `Drop`: auto-close the connection when `Connection<Open>` is dropped, logging a warning.
