# OCaml vs Rust: Higher-Kinded Types Simulation

## Side-by-Side Code

### OCaml — natural HKTs via module system

```ocaml
module type FUNCTOR = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
end

module ListFunctor : FUNCTOR with type 'a t = 'a list = struct
  type 'a t = 'a list
  let map = List.map
end

module OptionFunctor : FUNCTOR with type 'a t = 'a option = struct
  type 'a t = 'a option
  let map f = function None -> None | Some x -> Some (f x)
end

(* Generic algorithm over any FUNCTOR — works for both implementations *)
let double_all (type a) (module F : FUNCTOR with type 'a t = a list) xs =
  F.map (fun x -> x * 2) xs
```

### Rust (idiomatic HKT simulation via GATs)

```rust
// Step 1: defunctionalize the type constructor into a marker + GAT
pub trait HKT {
    type Applied<T>;
}

pub struct OptionHKT;
impl HKT for OptionHKT {
    type Applied<T> = Option<T>;
}

pub struct VecHKT;
impl HKT for VecHKT {
    type Applied<T> = Vec<T>;
}

// Step 2: Functor built on HKT
pub trait Functor: HKT {
    fn fmap<A, B>(fa: Self::Applied<A>, f: impl Fn(A) -> B) -> Self::Applied<B>;
}

impl Functor for OptionHKT {
    fn fmap<A, B>(fa: Option<A>, f: impl Fn(A) -> B) -> Option<B> { fa.map(f) }
}

impl Functor for VecHKT {
    fn fmap<A, B>(fa: Vec<A>, f: impl Fn(A) -> B) -> Vec<B> {
        fa.into_iter().map(f).collect()
    }
}

// Step 3: Generic algorithm — one function works for Option AND Vec
pub fn double_all<F>(fa: F::Applied<i32>) -> F::Applied<i32>
where
    F: Functor,
    F::Applied<i32>: Sized,
{
    F::fmap(fa, |x| x * 2)
}
```

### Rust (Monad — bind chains computations generically)

```rust
pub trait Monad: Functor {
    fn pure<A>(a: A) -> Self::Applied<A>;
    fn bind<A, B>(ma: Self::Applied<A>, f: impl Fn(A) -> Self::Applied<B>) -> Self::Applied<B>;
}

impl Monad for OptionHKT {
    fn pure<A>(a: A) -> Option<A> { Some(a) }
    fn bind<A, B>(ma: Option<A>, f: impl Fn(A) -> Option<B>) -> Option<B> { ma.and_then(f) }
}

impl Monad for VecHKT {
    fn pure<A>(a: A) -> Vec<A> { vec![a] }
    fn bind<A, B>(ma: Vec<A>, f: impl Fn(A) -> Vec<B>) -> Vec<B> {
        ma.into_iter().flat_map(f).collect()
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Type constructor | `'a t` (abstract type variable in module sig) | `type Applied<T>` (GAT on `HKT` trait) |
| Functor map | `val map : ('a -> 'b) -> 'a t -> 'b t` | `fn fmap<A,B>(fa: Self::Applied<A>, f: impl Fn(A)->B) -> Self::Applied<B>` |
| Generic functor fn | `(module F : FUNCTOR) -> 'a F.t -> 'b F.t` | `fn foo<F: Functor>(fa: F::Applied<A>) -> F::Applied<B>` |
| Monad bind | `val bind : 'a t -> ('a -> 'b t) -> 'b t` | `fn bind<A,B>(ma: Self::Applied<A>, f: impl Fn(A)->Self::Applied<B>) -> Self::Applied<B>` |
| Container HKT marker | Module identity (implicit) | Explicit marker struct (`OptionHKT`, `VecHKT`) |

## Key Insights

1. **Native vs. simulated**: OCaml supports HKTs natively through its module system — a `FUNCTOR` module signature is literally a type-level function from `'a` to `'a t`. Rust's type system has no type-level functions, so we simulate them with a *defunctionalization* trick: a concrete marker type (`OptionHKT`) paired with a GAT (`type Applied<T>`) plays the role of the OCaml module.

2. **GATs (Generic Associated Types) are the key enabler**: Stable since Rust 1.65, `type Applied<T>` inside a trait lets us express "`F` applied to `T`" — which is exactly the `'a t` in OCaml's module signature. Without GATs this simulation required unsafe pointer casts or was impossible.

3. **Boilerplate cost**: OCaml lets you pass module values at runtime (`(module ListFunctor)`), keeping the site callpoint clean. Rust requires an explicit type parameter at every generic call site (`double_all::<OptionHKT>(Some(3))`). The trade-off is compile-time monomorphisation (zero runtime cost) at the price of more syntactic noise.

4. **Monad expressiveness parity**: `Option::and_then`, `Vec::flat_map`, `Result::and_then` are all the same monadic `bind` — once the HKT simulation is in place, Rust can express this uniformly in `pipeline::<OptionHKT>` or `pipeline::<VecHKT>` without duplicating the algorithm.

5. **`PhantomData` for parameterised markers**: When the HKT marker needs to carry a type parameter (e.g., `ResultHKT<E>` fixes the error type), Rust requires `PhantomData<E>` to inform the type checker — OCaml has no equivalent burden because module types carry all variance information implicitly.

## When to Use Each Style

**Use the HKT simulation when:** you have multiple containers (`Option`, `Vec`, `Result`, custom types) and want a single generic algorithm — e.g., a generic `traverse`, `sequence`, or `fmap`-based pipeline that should work over all of them without copy-pasting.

**Use concrete `impl` blocks when:** you only need the algorithm for one or two containers, or the GAT-based trait bounds become so complex that they obscure the intent. Rust's monomorphisation means there is no runtime overhead either way, so the decision is purely about code organisation and maintainability.
