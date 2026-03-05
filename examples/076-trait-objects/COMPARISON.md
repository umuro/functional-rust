## Core Insight

Trait objects (`dyn Trait`) enable runtime polymorphism. The compiler generates a vtable for method dispatch. This is Rust's equivalent of OCaml's first-class modules or object system.

## OCaml Approach
- Object system with structural subtyping
- First-class modules for ad-hoc polymorphism
- No explicit vtable — runtime dispatch via method lookup

## Rust Approach
- `dyn Trait` behind a pointer (`Box<dyn Trait>`, `&dyn Trait`)
- Vtable-based dispatch (two-pointer fat pointer)
- Object safety rules: no generics, no `Self` in return position

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Dynamic dispatch | Objects / first-class modules | `dyn Trait` |
| Pointer type | Implicit (GC) | `Box<dyn T>` / `&dyn T` |
| Type erasure | Yes | Yes (via vtable) |
| Overhead | Method lookup | Fat pointer + vtable |
