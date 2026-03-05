# OCaml vs Rust: dyn Trait — Dynamic Dispatch

## Side-by-Side Code

### OCaml — First-class modules as open polymorphism

```ocaml
module type Shape = sig
  val area : unit -> float
  val name : unit -> string
end

let total_area (shapes : (module Shape) list) =
  List.fold_left (fun acc (module S : Shape) -> acc +. S.area ()) 0.0 shapes

let circle r : (module Shape) = (module struct
  let area () = Float.pi *. r *. r
  let name () = "circle"
end)

let rect w h : (module Shape) = (module struct
  let area () = w *. h
  let name () = "rectangle"
end)
```

### Rust — dyn Trait (dynamic dispatch, open set)

```rust
pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

pub fn total_area_dyn(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// Heterogeneous Vec — the canonical use case
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Rect { width: 3.0, height: 4.0 }),
];
```

### Rust — Enum dispatch (closed set, no heap)

```rust
pub enum AnyShape {
    Circle(Circle),
    Rect(Rect),
    Triangle(Triangle),
}

impl AnyShape {
    pub fn area(&self) -> f64 {
        match self {
            AnyShape::Circle(c) => c.area(),
            AnyShape::Rect(r) => r.area(),
            AnyShape::Triangle(t) => t.area(),
        }
    }
}

pub fn total_area_enum(shapes: &[AnyShape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}
```

### Rust — impl Trait / generics (static dispatch, zero-cost, homogeneous)

```rust
pub fn total_area_static<S: Shape>(shapes: &[S]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Trait/module type | `module type Shape` | `trait Shape` |
| Dynamic polymorphism | `(module Shape) list` | `Vec<Box<dyn Shape>>` |
| Fat pointer | anonymous (first-class module) | `Box<dyn Shape>` = (data ptr, vtable ptr) |
| Static polymorphism | `'a list` with concrete module | `&[S] where S: Shape` |
| Closed-set dispatch | N/A (use ADT variants) | `enum AnyShape { Circle(..), Rect(..) }` |

## Key Insights

1. **OCaml first-class modules ≈ Rust `dyn Trait`**: Both let you package a value with its method table at runtime and store mixed types in a list/vec. OCaml's mechanism is more structural; Rust's is more explicit about heap allocation and pointer indirection.

2. **`Box<dyn Shape>` is a fat pointer**: It carries two machine words — one to the heap data, one to a vtable of function pointers. Every method call goes through the vtable at runtime. The cost is real but usually negligible compared to what the methods do.

3. **Enum dispatch beats `dyn Trait` for closed sets**: When you own every variant, an enum + `match` lets the compiler inline everything, skip heap allocation, and enforce exhaustiveness. `dyn Trait` wins when the set of types is open (plugins, user-provided types, type-erased collections built from separate crates).

4. **`impl Trait` / generics win when types are homogeneous**: Monomorphization produces one specialized copy of the function per concrete type — zero overhead, no vtable — but every element of a slice must be the same type. You cannot mix `Circle` and `Rect` in a `&[impl Shape]`.

5. **Object safety**: Not every trait can be used with `dyn`. A trait is object-safe only if its methods take `&self` / `&mut self` (no `Self` in return position, no generic methods). This is enforced at compile time in Rust; OCaml module types have no such restriction.

## When to Use Each Style

**Use `dyn Trait` when:** you need a heterogeneous collection, you're exposing a plugin API, or the concrete types arrive from external crates you don't control.

**Use enum dispatch when:** the set of variants is fixed and you own all of them — you get exhaustiveness checking, no heap allocation, and faster dispatch.

**Use `impl Trait` / generics when:** all elements in a collection are the same concrete type and you want zero-overhead monomorphized code without boxing.
