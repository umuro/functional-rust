# OCaml vs Rust: Type-Level Natural Numbers

## Side-by-Side Code

### OCaml (GADT approach)

```ocaml
type zero = Zero_t
type 'n succ = Succ_t

type _ nat =
  | Zero : zero nat
  | Succ : 'n nat -> 'n succ nat

let rec to_int : type n. n nat -> int = function
  | Zero -> 0
  | Succ n -> 1 + to_int n

(* Length-indexed list — 'n encodes the length *)
type (_, _) vec =
  | Nil  : ('a, zero nat) vec
  | Cons : 'a * ('a, 'n nat) vec -> ('a, 'n succ nat) vec
```

### Rust (marker-type approach)

```rust
use std::marker::PhantomData;

pub struct Zero;
pub struct Succ<N>(PhantomData<N>);

pub trait Nat { const VALUE: usize; }
impl Nat for Zero { const VALUE: usize = 0; }
impl<N: Nat> Nat for Succ<N> { const VALUE: usize = N::VALUE + 1; }

pub type One   = Succ<Zero>;
pub type Two   = Succ<One>;
pub type Three = Succ<Two>;
```

### Rust (length-indexed vector — `pop` only on non-empty)

```rust
pub struct TypeVec<T, N: Nat> {
    data: Vec<T>,
    _len: PhantomData<N>,
}

// push is available for any N — returns Succ<N>
impl<T, N: Nat> TypeVec<T, N> {
    pub fn push(mut self, value: T) -> TypeVec<T, Succ<N>> { … }
}

// pop is ONLY available for Succ<N> — popping Zero is a compile error
impl<T, N: Nat> TypeVec<T, Succ<N>> {
    pub fn pop(mut self) -> (TypeVec<T, N>, T) { … }
}
```

### Rust (type-level addition)

```rust
pub trait Add<B: Nat>: Nat { type Sum: Nat; }

impl<B: Nat> Add<B> for Zero       { type Sum = B; }
impl<A: Nat + Add<B>, B: Nat> Add<B> for Succ<A> {
    type Sum = Succ<<A as Add<B>>::Sum>;
}

// 2 + 3 = 5, verified at compile time:
assert_eq!(<Two as Add<Three>>::Sum::VALUE, 5);
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Zero | `type zero = Zero_t` | `pub struct Zero;` |
| Successor | `type 'n succ = Succ_t` | `pub struct Succ<N>(PhantomData<N>);` |
| Nat reflection | `val to_int : 'n nat -> int` | `trait Nat { const VALUE: usize; }` |
| Length-indexed vec | `('a, 'n nat) vec` | `TypeVec<T, N: Nat>` |
| Type-level addition | type-level via GADTs | `trait Add<B>: Nat { type Sum: Nat; }` |
| Non-empty constraint | encoded in GADT constructor | `impl<T,N:Nat> TypeVec<T,Succ<N>>` |

## Key Insights

1. **GADTs vs marker structs**: OCaml uses GADTs to relate value-level constructors
   (`Zero`, `Succ`) to type-level witnesses, giving a single unified term that
   carries both the value and its type index.  Rust instead uses zero-sized marker
   structs (`Zero`, `Succ<N>`) — the type *is* the witness, and `PhantomData`
   ensures the struct is zero-cost.

2. **Trait-based reflection**: OCaml reads off the Peano number with a
   pattern-matching function (`to_int`).  Rust uses a trait constant `Nat::VALUE`,
   which the compiler evaluates at compile time; no runtime dispatch occurs.

3. **Method-level safety**: OCaml enforces non-emptiness through exhaustiveness of
   the GADT pattern — the `Nil` case just isn't present in the function.  Rust
   achieves the same by scoping the `pop`, `first`, and `last` methods to
   `impl<T,N:Nat> TypeVec<T,Succ<N>>`: calling them on `TypeVec<T,Zero>` is a
   hard compile error because the method literally doesn't exist for that type.

4. **Associated-type arithmetic**: Type-level addition in OCaml requires a GADT
   proof term that the compiler carries through.  In Rust, associated types on a
   trait (`type Sum: Nat`) let the compiler compute `Add::Sum` entirely at the
   type level — no value is constructed, only types are resolved.

5. **Zero cost at runtime**: Both approaches impose zero overhead.  Rust's marker
   structs and `PhantomData` are erased entirely; `TypeVec<T,N>` is
   representation-equivalent to `Vec<T>` — the length type parameter exists only
   during compilation.

## When to Use Each Style

**Use idiomatic Rust (marker structs + trait constants)** when you need
compile-time length guarantees without a dependently-typed language — e.g., safe
API contracts for fixed-size buffers, cryptographic keys, matrix dimensions, or
protocol framing fields where the wrong size must be a build error, not a panic.

**Use the recursive/GADT style (closer to OCaml)** when you are porting dependently
typed OCaml/Haskell code and want to preserve the structural correspondence, or
when you need type-level arithmetic (addition, comparison) that benefits from the
inductive structure being explicit in the trait implementations.
