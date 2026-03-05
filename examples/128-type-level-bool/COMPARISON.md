# OCaml vs Rust: Type-Level Booleans

## Side-by-Side Code

### OCaml
```ocaml
(* Phantom types: 'b is never stored — it's only a compile-time label *)
type true_t = True_t
type false_t = False_t

type 'b flag = { _phantom : unit }

let mk_true  : true_t  flag = { _phantom = () }
let mk_false : false_t flag = { _phantom = () }

(* GADT-based type-level bool *)
type _ tbool =
  | TTrue  : true_t  tbool
  | TFalse : false_t tbool
```

### Rust (marker structs + trait)
```rust
use std::marker::PhantomData;

pub struct True;
pub struct False;

pub trait Bool { const VALUE: bool; }
impl Bool for True  { const VALUE: bool = true;  }
impl Bool for False { const VALUE: bool = false; }

// Type-level NOT via associated type
pub trait Not { type Output: Bool; }
impl Not for True  { type Output = False; }
impl Not for False { type Output = True;  }
```

### Rust (builder enforced at compile time)
```rust
pub struct Config<V, L> {
    host: String,
    port: u16,
    _validated: PhantomData<V>,
    _logged:    PhantomData<L>,
}

// execute() only exists on Config<True, True>
impl Config<True, True> {
    pub fn execute(&self) -> String {
        format!("Executing on {}:{}", self.host, self.port)
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Type-level true | `true_t` (phantom param) | `struct True;` |
| Type-level false | `false_t` (phantom param) | `struct False;` |
| Phantom parameter | `'b flag` — `'b` unused in body | `PhantomData<B>` |
| Runtime reflection | Module value `val value : bool` | `trait Bool { const VALUE: bool }` |
| Type-level NOT | Separate type families | `trait Not { type Output: Bool }` |
| Conditional methods | Module functors / GADTs | `impl Config<True, True>` |

## Key Insights

1. **Phantom types share the core idea** — both OCaml and Rust use a type parameter that carries no runtime data; the "value" exists only in the type system.

2. **Rust uses zero-sized structs; OCaml uses type aliases** — `struct True;` compiles to nothing at runtime, exactly like OCaml's `type true_t = True_t`. Both are erased before execution.

3. **Associated types replace GADT witnesses** — OCaml's GADTs (`type _ tbool`) prove relationships between type indices at match sites. Rust's `trait Not { type Output }` encodes the same logic as a compiler-verified type mapping.

4. **`impl` specialization enforces preconditions** — defining `execute()` only on `Config<True, True>` means calling it prematurely is a *compile error*, not a *runtime panic*. OCaml achieves this with module signatures that hide the constructor.

5. **`PhantomData` prevents variance surprises** — Rust's ownership model requires declaring how a phantom type is used (owned, borrowed, covariant, etc.). `PhantomData<V>` tells the compiler `Config` is covariant over `V` and owns a notional `V`, which is the correct variance for a state-machine type.

## When to Use Each Style

**Use idiomatic Rust (`Bool` trait + const VALUE) when:** you need to inspect the boolean at runtime (e.g., logging, serialization) while still encoding it as a type.

**Use the builder (`impl Config<True, True>`) when:** you want compile-time enforcement of a multi-step setup protocol with no runtime cost whatsoever — the type itself becomes the proof.
