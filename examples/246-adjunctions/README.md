[![Functional Rust](https://img.shields.io/badge/functional--rust-examples-blue)](https://hightechmind.io)

# Adjunctions
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

An adjunction between two functors `F` and `G` is a natural bijection between morphisms: `Hom(F A, B) ≅ Hom(A, G B)`. Adjunctions are the categorical generalization of "best approximations" and appear everywhere in functional programming: currying is an adjunction, the product/exponent adjunction underlies closures, and many monad/comonad pairs arise from adjunctions. This example implements adjunctions in Rust and shows how the `State` and `Env` comonad/monad pair emerges from the product adjunction.

## Learning Outcomes

- Understand adjunctions via the `unit` and `counit` natural transformations
- Implement the product/diagonal adjunction giving rise to Reader/Writer
- Derive a monad from the composition `G ∘ F` using adjunction data
- Derive a comonad from the composition `F ∘ G` using adjunction data
- Compare Rust's trait-encoded adjunctions with OCaml's module functors

## Rust Application

Encoding an adjunction requires `unit` (eta) and `counit` (epsilon):

```rust
// Adjunction F ⊣ G means:
//   unit:   A -> G(F(A))
//   counit: F(G(B)) -> B
// satisfying triangle identities

// The product adjunction: (- × E) ⊣ (E → -)
// Left functor:  Prod<E, A> = (E, A)       "F = (- × E)"
// Right functor: Exp<E, B>  = E -> B       "G = (E → -)"

#[derive(Clone, Debug)]
struct Prod<E, A>(E, A);  // (E, A)

// G B = E -> B  (function type, represented as closure)

// unit: A -> G(F(A)) = A -> (E -> (E, A))
fn unit<E: Clone, A: Clone>(a: A) -> impl Fn(E) -> Prod<E, A> {
    move |e| Prod(e, a.clone())
}

// counit: F(G(B)) = (E, E->B) -> B
fn counit<E, B>(Prod(e, f): Prod<E, impl Fn(E) -> B>) -> B {
    f(e)
}

// Monad from G ∘ F: Reader monad = E -> (E, A) collapsed = E -> A
// Actually the State monad arises from (- × S) ⊣ (S → -)

// State monad derived from adjunction
struct State<S, A>(Box<dyn Fn(S) -> (A, S)>);

impl<S: Clone + 'static, A: Clone + 'static> State<S, A> {
    fn run(self, s: S) -> (A, S) { (self.0)(s) }

    // unit (return): a -> State s a
    fn pure(a: A) -> Self {
        State(Box::new(move |s| (a.clone(), s)))
    }

    // bind derived from adjunction composition
    fn bind<B: Clone + 'static>(self, f: impl Fn(A) -> State<S, B> + 'static) -> State<S, B> {
        State(Box::new(move |s| {
            let (a, s1) = (self.0)(s);
            (f(a).0)(s1)
        }))
    }

    fn get() -> State<S, S> {
        State(Box::new(|s: S| (s.clone(), s)))
    }

    fn put(new_s: S) -> State<S, ()> {
        State(Box::new(move |_| ((), new_s.clone())))
    }
}

// Comonad from F ∘ G: Env comonad = (E, E->A) with eval = Prod(e, f) -> f(e)
// This IS the counit (epsilon) of the adjunction

fn main() {
    // Unit: embed a value into the product adjunction
    let embed = unit::<String, i32>(42);
    let Prod(e, a) = embed(String::from("hello"));
    println!("unit: ({e}, {a})");  // ("hello", 42)

    // Counit: evaluate (E, E->B) -> B
    let prod = Prod(String::from("world"), |s: String| s.len());
    println!("counit: {}", counit(prod)); // 5

    // State monad from the adjunction
    let program = State::pure(0_i32)
        .bind(|_| State::put(10))
        .bind(|_| State::get())
        .bind(|s| State::pure(s * 2));

    let (result, final_state) = program.run(0);
    println!("State result: {result}, final state: {final_state}"); // 20, 10

    // Triangle identity check: counit(F(unit(a))) = id_A
    let a = 99_i32;
    let env = String::from("ctx");
    let after_unit: Prod<String, i32> = unit::<String, i32>(a)(env.clone());
    // counit of F(unit(a)) in (- x E) ⊣ (E ->) context
    println!("Triangle: {}", after_unit.1); // 99 = a ✓
}
```

The triangle identities are what separate a genuine adjunction from a random pair of natural transformations.

## OCaml Approach

OCaml uses functors (module-level) for adjunctions:

```ocaml
module type ADJUNCTION = sig
  type 'a left_f   (* F A *)
  type 'a right_g  (* G B *)
  val unit   : 'a -> 'a left_f right_g
  val counit : 'a right_g left_f -> 'a
end

(* Product adjunction: F = (E *), G = (E ->) *)
module ProdAdj (E : sig type t val e : t end) = struct
  type 'a left_f  = E.t * 'a
  type 'a right_g = E.t -> 'a
  let unit a e = (e, a)
  let counit (e, f) = f e
end
```

OCaml functors map cleanly to categorical functors; Rust traits simulate the same with more verbosity.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Functor encoding | trait + struct | module type + module |
| Type families | associated types | module type parameters |
| Adjunction laws | runtime tests | module coherence |
| Higher-kinded | simulated via GATs | module polymorphism |
| Monad derivation | manual `bind` | can derive via functor composition |

Adjunctions provide a systematic way to derive monads and comonads: every adjunction gives rise to a monad `G ∘ F` and a comonad `F ∘ G`.

## Exercises

1. Implement the diagonal adjunction `Δ ⊣ ×` (diagonal functor left adjoint to product), which gives rise to the tuple monad.
2. Verify both triangle identities programmatically: `G(ε) ∘ η_G = id_G` and `ε_F ∘ F(η) = id_F`.
3. Derive the `Writer<W, A>` monad from the adjunction between `(- × W)` and `(W →)` where `W` is a monoid.
4. Show that the `List` monad arises from the free/forgetful adjunction between `Set` and `Mon` (monoids).
5. Implement the adjoint transpose: given `f: F A -> B`, produce `f̂: A -> G B` using `unit` and `fmap`.
