**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐⭐⭐  

[profunctor-intro on hightechmind.io](https://hightechmind.io/posts/functional-rust/profunctor-intro)

---

## Problem Statement

Introduce profunctors — abstractions that are covariant in their output type and contravariant in their input type. Implement a concrete `Mapper<A, B>` struct (wrapping `A -> B`) with `dimap`, `lmap` (contramap input), and `rmap` (covariant map output). Also implement `Star<A, B>` (wrapping `A -> Option<B>`) to show the same profunctor pattern in a richer context.

## Learning Outcomes

- Understand the profunctor interface: `dimap :: (C -> A) -> (B -> D) -> p A B -> p C D`
- Recognize that `dimap f g p = g ∘ p ∘ f` — pre-compose input adapter, post-compose output adapter
- Implement `lmap` as `dimap f id` (contramap — adapt only the input)
- Implement `rmap` as `dimap id g` (covariant map — adapt only the output)
- Understand why Rust cannot express a generic `Profunctor` trait (no HKT) and how to work around it with concrete types

## Rust Application

```rust
pub struct Mapper<A, B> {
    f: Box<dyn Fn(A) -> B>,
}

impl<A: 'static, B: 'static> Mapper<A, B> {
    pub fn new<F: Fn(A) -> B + 'static>(f: F) -> Self {
        Mapper { f: Box::new(f) }
    }

    pub fn apply(&self, a: A) -> B { (self.f)(a) }

    /// dimap: pre-compose with `pre`, post-compose with `post`
    /// dimap f g p = post ∘ p ∘ pre
    pub fn dimap<C: 'static, D: 'static>(
        self,
        pre: impl Fn(C) -> A + 'static,
        post: impl Fn(B) -> D + 'static,
    ) -> Mapper<C, D> {
        Mapper::new(move |c| post((self.f)(pre(c))))
    }

    /// lmap: adapt only input (contramap)
    pub fn lmap<C: 'static>(self, pre: impl Fn(C) -> A + 'static) -> Mapper<C, B> {
        Mapper::new(move |c| (self.f)(pre(c)))
    }

    /// rmap: adapt only output (covariant map)
    pub fn rmap<D: 'static>(self, post: impl Fn(B) -> D + 'static) -> Mapper<A, D> {
        Mapper::new(move |a| post((self.f)(a)))
    }
}
```

`dimap` takes the existing function `p: A -> B` and wires adapters around it: `C -pre-> A -p-> B -post-> D`, yielding `C -> D`. This is function composition written outside-in.

`lmap` specializes `dimap` to only adapt input: useful when adding a preprocessing step without changing the output type. `rmap` specializes to post-processing only.

`Box<dyn Fn(A) -> B>` is required because the returned `Mapper<C, D>` captures closures of different types in each `dimap` call — type erasure lets them live in the same struct field.

## OCaml Approach

```ocaml
(* Haskell: class Profunctor p where
     dimap :: (c -> a) -> (b -> d) -> p a b -> p c d
   OCaml can express this with modules and functors *)

module type PROFUNCTOR = sig
  type ('a, 'b) t
  val dimap : ('c -> 'a) -> ('b -> 'd) -> ('a, 'b) t -> ('c, 'd) t
end

(* Functions are the canonical profunctor *)
module FnProfunctor : PROFUNCTOR with type ('a, 'b) t = 'a -> 'b = struct
  type ('a, 'b) t = 'a -> 'b
  let dimap pre post f = fun c -> post (f (pre c))
end

(* Using it *)
let double_string =
  FnProfunctor.dimap
    int_of_string      (* pre: string -> int *)
    string_of_int      (* post: int -> string *)
    (fun n -> n * 2)   (* core: int -> int *)
(* double_string "21" = "42" *)
```

OCaml modules and functors allow a generic `PROFUNCTOR` interface. The implementation is more composable than Rust's concrete type approach, but requires more boilerplate for module instantiation.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Generic profunctor trait | Not possible without HKT | `module type PROFUNCTOR` with type parameter |
| Concrete implementation | `Mapper<A,B>` with methods | Module implementing the signature |
| Type erasure | `Box<dyn Fn>` | Not needed — closures are GC values |
| Composition | Method chaining: `.lmap(...).rmap(...)` | Function application: `dimap pre post f` |
| Laws | Tested with concrete values | Provable from module abstraction |

Profunctors generalize the idea of "things that can be mapped on both ends." They appear in optics (lenses/prisms), arrow composition, and parser combinators. The concrete `Mapper` is the simplest example.

## Exercises

1. Verify the profunctor identity law: `dimap id id p = p` — applying both identity functions leaves the mapper unchanged.
2. Verify the composition law: `dimap (f ∘ g) (h ∘ k) = dimap g h ∘ dimap f k`.
3. Implement `Star` fully: `Star<A, B>` wrapping `Fn(A) -> Option<B>` with its own `dimap`.
4. Implement a `Costar<A, B>` wrapping `Fn(Vec<A>) -> B>` (works over the input collection).
5. Build a data validation pipeline using `Mapper` where `lmap` converts raw string input and `rmap` formats the validated output.
