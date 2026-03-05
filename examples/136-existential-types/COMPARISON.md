# OCaml vs Rust: Existential Types

## Side-by-Side Code

### OCaml — first-class module packing

```ocaml
module type SHOWABLE = sig
  type t
  val value : t
  val show  : t -> string
end

let pack_showable (type a) (show : a -> string) (value : a) : (module SHOWABLE) =
  (module struct
    type t = a
    let value = value
    let show  = show
  end)

let show_it (m : (module SHOWABLE)) =
  let module M = (val m) in M.show M.value
```

### OCaml — closure record (lighter weight)

```ocaml
type showable = { show : unit -> string }

let make_showable show value = { show = fun () -> show value }
```

### Rust — `impl Trait` (static existential, zero-cost)

```rust
pub trait Showable { fn show(&self) -> String; }

// Caller sees `impl Showable`; concrete type `Counter` is hidden at the API boundary.
pub fn make_counter(n: u32) -> impl Showable { Counter(n) }
```

### Rust — `Box<dyn Trait>` (dynamic existential, heterogeneous collections)

```rust
// Erase the concrete type at runtime; enable Vec<Box<dyn Showable>>.
pub fn pack(value: impl Showable + 'static) -> Box<dyn Showable> { Box::new(value) }
pub fn show_all(items: &[Box<dyn Showable>]) -> Vec<String> {
    items.iter().map(|i| i.show()).collect()
}
```

### Rust — closure-based erasure (mirrors OCaml's record approach)

```rust
pub struct ShowClosure { show_fn: Box<dyn Fn() -> String> }

impl ShowClosure {
    pub fn new<T: 'static>(value: T, show_fn: impl Fn(&T) -> String + 'static) -> Self {
        ShowClosure { show_fn: Box::new(move || show_fn(&value)) }
    }
    pub fn show(&self) -> String { (self.show_fn)() }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Existential return | `(module SHOWABLE)` | `impl Showable` |
| Runtime-erased value | `{ show : unit -> string }` | `Box<dyn Showable>` |
| Closure erasure | `make_showable : ('a -> string) -> 'a -> showable` | `ShowClosure::new<T>(T, Fn(&T)->String)` |
| Heterogeneous list | `any_list` GADT | `Vec<Box<dyn Showable>>` |
| Interface declaration | `module type SHOWABLE` | `trait Showable` |

## Key Insights

1. **Two flavours of existential**: Rust offers `impl Trait` (static, compiler-resolved, zero overhead) and `Box<dyn Trait>` (dynamic, vtable dispatch). OCaml's first-class modules sit between these: they carry type information but erase the concrete identity from callers.

2. **Static vs dynamic dispatch**: `impl Trait` in return position is monomorphised by the compiler — no allocation, no vtable. `Box<dyn Trait>` pays one allocation and one indirection per call. OCaml's `(module SHOWABLE)` is dynamically dispatched like `dyn Trait`.

3. **Closure erasure is identical in spirit**: OCaml's `{ show : unit -> string }` and Rust's `Box<dyn Fn() -> String>` both capture a value and erase its type by closing over it. The only difference is syntactic.

4. **`impl Trait` forces a single concrete type per function**: if you need to return different concrete types at runtime, you must use `Box<dyn Trait>`. OCaml's first-class modules always allow heterogeneous unpacking because the module itself carries the type index.

5. **Lifetime annotation `'static`**: Rust requires `T: 'static` when boxing a trait object that might outlive the call frame. OCaml's GC-managed heap makes this bookkeeping invisible, though the same guarantee is implicitly enforced at runtime.

## When to Use Each Style

**Use `impl Trait` when:** the function always returns the same concrete type and you just want to hide it from the public API — zero-cost abstraction.

**Use `Box<dyn Trait>` when:** you need a heterogeneous collection, a function that returns different types depending on runtime state, or you're building a plugin/registry system.

**Use closure erasure (`ShowClosure`) when:** you want to bundle a value with custom behaviour at the point of construction — closest to OCaml's record-based existential and the most flexible approach.
