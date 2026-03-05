# OCaml vs Rust: Clone and Copy

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml: all values are implicitly shared/copied by the GC.
   No distinction between cheap stack copy and expensive heap copy. *)

type point = { x : float; y : float }

let translate p dx dy = { x = p.x +. dx; y = p.y +. dy }

let () =
  let origin = { x = 0.0; y = 0.0 } in
  let moved = translate origin 1.0 2.0 in
  (* origin is still valid — GC manages both records implicitly *)
  assert (origin.x = 0.0);
  assert (moved.x = 1.0);

  (* Strings: structural sharing, no explicit clone needed *)
  let s1 = "hello" in
  let s2 = s1 ^ " world" in   (* creates new string, GC handles old *)
  assert (s1 = "hello");
  assert (s2 = "hello world");
  print_endline "ok"
```

### Rust (Copy — stack types, implicit duplication)
```rust
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point { pub x: f64, pub y: f64 }

impl Point {
    pub fn translate(self, dx: f64, dy: f64) -> Self {
        Self { x: self.x + dx, y: self.y + dy }
    }
}

fn copy_demo() {
    let origin = Point { x: 0.0, y: 0.0 };
    let moved = origin.translate(1.0, 2.0); // origin copied implicitly
    assert_eq!(origin, Point { x: 0.0, y: 0.0 }); // still valid
    assert_eq!(moved.x, 1.0);

    let x: i32 = 42;
    let y = x; // silent bitwise copy — both x and y are valid
    assert_eq!(x, y);
}
```

### Rust (Clone — heap types, explicit deep copy)
```rust
fn clone_demo() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // explicit — you see the heap allocation cost
    assert_eq!(s1, s2);  // s1 still valid, s2 is a new allocation

    let v1 = vec![1, 2, 3];
    let v2 = v1.clone(); // another explicit deep copy
    assert_eq!(v1, v2);
}
```

### Rust (Clone-only struct — owns heap data)
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct NamedPoint {
    pub label: String, // owns heap data → cannot be Copy
    pub x: f64,
    pub y: f64,
}

impl NamedPoint {
    pub fn translate(&self, dx: f64, dy: f64) -> Self {
        Self {
            label: self.label.clone(), // must clone the String explicitly
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Cheap copy | Implicit (GC handles everything) | `Copy` trait — bitwise, silent |
| Expensive copy | Implicit (GC allocates new heap object) | `Clone` trait — explicit `.clone()` call |
| Stack-only struct | `type point = { x: float; y: float }` | `#[derive(Copy, Clone)] struct Point` |
| Heap-owning struct | `type named = { label: string; x: float }` | `#[derive(Clone)] struct NamedPoint { label: String, .. }` |
| String copy | `let s2 = s1` (shared/copied implicitly) | `let s2 = s1.clone()` (explicit heap allocation) |
| Function taking struct | `let f p = ...` (implicit copy or share) | `fn f(p: Point)` (moved or copied, depending on `Copy`) |

## Key Insights

1. **Visibility of cost**: In OCaml the GC silently manages all copies and sharing — you never see the cost. In Rust, `Copy` (cheap, stack-only) is invisible while `Clone` (potentially expensive, heap-allocating) must be written explicitly, making costs visible at the call site.

2. **`Copy` is a subset of `Clone`**: A `Copy` type must also implement `Clone`; `Clone` is the general interface, `Copy` is the optimized silent variant. If a type contains any non-`Copy` field (like `String`), it cannot be `Copy`, only `Clone`.

3. **Move semantics vs copy semantics**: Types without `Copy` are *moved* on assignment — the original binding becomes invalid. Types with `Copy` are silently duplicated. OCaml has neither concept because the GC tracks all references.

4. **Design signal**: When you see `.clone()` in a Rust code review, it is a deliberate signal — something heap-heavy is being duplicated. In OCaml you never get that signal, which can hide performance problems.

5. **Struct composition determines trait availability**: Adding a single `String` field to an otherwise stack-only struct removes the ability to derive `Copy`. This makes the memory model explicit in the type definition itself, not just at use sites.

## When to Use Each Style

**Derive `Copy` when:** the struct holds only stack-sized, trivially-copyable fields (`i32`, `f64`, `bool`, arrays of `Copy`, etc.) and implicit duplication is semantically correct (e.g., coordinate types, small value objects).

**Derive only `Clone` when:** the struct owns heap data (`String`, `Vec`, `Box`, etc.) and copies must be explicit to make their cost visible to the reader.
