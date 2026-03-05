# OCaml vs Rust: Move Semantics

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml: GC manages memory — values are shared freely *)
let use_string s =
  String.length s

let () =
  let greeting = "Hello, ownership!" in
  let len1 = use_string greeting in
  let len2 = use_string greeting in  (* perfectly fine — no move *)
  assert (len1 = len2)
```

### Rust (idiomatic — borrow to avoid move)
```rust
pub fn borrow_string(s: &str) -> usize {
    s.len()
}

fn main() {
    let greeting = String::from("Hello, ownership!");
    let len1 = borrow_string(&greeting);
    let len2 = borrow_string(&greeting); // fine — borrowed, not moved
    assert_eq!(len1, len2);
}
```

### Rust (ownership transfer — move semantics)
```rust
pub fn consume_string(s: String) -> usize {
    s.len()
    // s is dropped here — memory freed immediately
}

fn main() {
    let greeting = String::from("Hello, ownership!");
    let len = consume_string(greeting);
    // greeting is GONE — compiler rejects any further use
    assert_eq!(len, 17);
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| String type | `string` (immutable, GC-managed) | `String` (owned, heap) / `&str` (borrowed slice) |
| Passing a string | `val f : string -> int` — always shared | `fn f(s: String)` moves; `fn f(s: &str)` borrows |
| Ownership | implicit — GC tracks all refs | explicit — one owner, tracked statically |
| Memory reclaim | GC pause, non-deterministic | deterministic drop at end of owner scope |
| Copy semantics | all values implicitly shareable | only `Copy` types copy; others move |

## Key Insights

1. **No GC required:** Rust's ownership model gives the compiler enough information to insert `free` calls automatically — at exactly the right point, with zero runtime overhead.

2. **Move = compile-time transfer:** When you pass a `String` to a function in Rust, the compiler treats the original binding as dead. Any subsequent use is a compile error, not a runtime crash.

3. **Borrow is the idiomatic escape hatch:** Most functions that only *read* data should take `&str` or `&T` — this lets callers keep ownership while the function borrows temporarily.

4. **OCaml's GC is the difference:** In OCaml every value is heap-allocated and reference-counted/traced. You can alias freely because the GC ensures the memory lives as long as any reference exists. Rust instead enforces a single owner so it can use stack discipline for memory management.

5. **Copy types opt out of move:** Primitives like `i32`, `bool`, `char` implement `Copy` — assignment duplicates them bitwise, so the original remains valid. `String` and structs containing heap data do not implement `Copy` by default.

## When to Use Each Style

**Use move (owned `String` / `T`):** When the function needs to store, transform, or return the value — it takes full responsibility for the data's lifetime.

**Use borrow (`&str` / `&T`):** When the function only reads or inspects the value and the caller should retain ownership after the call. This is the default choice for most function parameters.
