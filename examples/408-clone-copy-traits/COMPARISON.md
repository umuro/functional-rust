# OCaml vs Rust: Clone and Copy Traits

## Side-by-Side Code

### OCaml — GC handles all copying
```ocaml
(* All values can be copied implicitly *)
let x = 42 in
let y = x in  (* Both x and y are valid *)
Printf.printf "%d %d\n" x y

(* Explicit copy for mutable data *)
let s1 = "hello" in
let s2 = String.copy s1 in
(* s1 and s2 are independent *)
```

### Rust — Copy vs Clone distinction
```rust
// Copy: implicit, bitwise, for small stack types
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

let p1 = Point { x: 1, y: 2 };
let p2 = p1;  // Copy: p1 still valid
println!("{} {}", p1.x, p2.x);

// Clone: explicit, for heap-allocated types
let s1 = String::from("hello");
let s2 = s1.clone();  // Must be explicit
// s1 and s2 are independent
```

---

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Default | GC-managed sharing | Move semantics |
| Implicit copy | All values (via GC) | Only `Copy` types |
| Explicit copy | `String.copy`, etc. | `.clone()` |
| Stack types | Same as heap | `Copy` — implicit bitwise |
| Heap types | Same as stack | `Clone` — explicit, may allocate |
| Performance | GC overhead | Zero-cost for Copy, explicit cost for Clone |

---

## Copy Requirements

A type can be `Copy` only if:
1. All fields are `Copy`
2. It doesn't implement `Drop`
3. It's entirely stack-based (no heap pointers)

```rust
// Can be Copy
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

// Cannot be Copy (contains String → heap)
#[derive(Clone)]
struct Named { name: String, value: i32 }

// Cannot be Copy (has Drop)
struct Resource { /* ... */ }
impl Drop for Resource { fn drop(&mut self) { } }
```

---

## Copy vs Clone in Practice

```rust
// Copy: use after assignment
let v1 = Vector2D { x: 1.0, y: 2.0 };
let v2 = v1;  // Copy
let v3 = v1;  // Still valid!
assert_eq!(v1, v2);

// Clone: explicit call required
let dna1 = DNA::new("ATCG");
let dna2 = dna1.clone();  // Explicit
dna1.sequence;  // Still valid

// Move (no Copy, no Clone)
let s1 = String::from("hello");
let s2 = s1;  // Moved
// s1.len();  // Error: use of moved value
```

---

## 5 Takeaways

1. **OCaml's GC makes all values implicitly copyable.**
   No distinction between Copy and Clone.

2. **Rust's Copy is implicit and zero-cost.**
   Bitwise copy for small, stack-only types.

3. **Rust's Clone is explicit and may allocate.**
   Call `.clone()` to duplicate heap data.

4. **Copy requires no Drop implementation.**
   Types with destructors cannot be Copy.

5. **Default in Rust is move, not copy.**
   Assignment transfers ownership unless Copy is implemented.
