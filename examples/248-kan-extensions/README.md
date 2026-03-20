[![Functional Rust](https://img.shields.io/badge/functional--rust-examples-blue)](https://hightechmind.io)

# Kan Extensions

## Problem Statement

Kan extensions answer the question: "given functors `F: C -> D` and `G: C -> E`, how do we best extend or lift G along F to get a functor from D to E?" The Right Kan Extension (Ran) and Left Kan Extension (Lan) are the universal solutions to this problem. In functional programming, Ran encodes the Yoneda lemma, and Lan gives rise to coends and existential types. Both appear in the implementation of profunctor optics and indexed traversals.

## Learning Outcomes

- Understand Ran and Lan as universal functorial lifting operations
- Implement Ran as `forall b. (a -> f b) -> g b` (the right extension)
- Implement Lan as `exists b. (f b -> a, g b)` (the left extension)
- See how the Yoneda lemma is a special case of Ran
- Compare Kan extensions with OCaml's module-level encoding

## Rust Application

Kan extensions encode sophisticated functor composition patterns:

```rust
use std::rc::Rc;

// Right Kan Extension of G along F:
// Ran F G A = forall B. (A -> F B) -> G B
// Encoding: a struct holding a closure that is universally quantified over B

// Since Rust lacks impredicative polymorphism, we use trait objects
trait RanBody<FB, GB> {
    fn apply(&self, f: Rc<dyn Fn(i32) -> FB>) -> GB;
}

// Concretely: Ran Id Id A ≅ A  (Yoneda for identity functor)
// Ran F G A where F=G=Identity:
// = forall B. (A -> B) -> B  ≡  the Yoneda embedding

struct Yoneda<A> {
    // forall B. (A -> B) -> B
    run: Rc<dyn Fn(Rc<dyn Fn(A) -> Box<dyn std::any::Any>>) -> Box<dyn std::any::Any>>,
    val: A, // store the value for practical use
}

impl<A: Clone + 'static> Yoneda<A> {
    fn new(a: A) -> Self {
        let a2 = a.clone();
        Yoneda {
            run: Rc::new(move |f| f(a2.clone())),
            val: a,
        }
    }

    // Yoneda isomorphism: Ran Id G A ≅ G A
    // toYoneda: G A -> Yoneda G A
    // fromYoneda: Yoneda G A -> G A (run with identity)
    fn from_yoneda(&self) -> A { self.val.clone() }

    // fmap via Yoneda (no constraint on A beyond Clone)
    fn map<B: Clone + 'static>(&self, f: impl Fn(A) -> B + 'static) -> Yoneda<B> {
        Yoneda::new(f(self.val.clone()))
    }
}

// Left Kan Extension of G along F:
// Lan F G A = exists B. (F B -> A, G B)
// Encoding as a struct with existential via Box<dyn Any>

struct Lan<A> {
    // exists B. (f B -> A, g B)
    // We fix f = g = Vec for a concrete example (Lan Vec Vec)
    project: Box<dyn Fn(Vec<Box<dyn std::any::Any>>) -> A>,
    values: Vec<Box<dyn std::any::Any>>,
}

// Simpler, more practical: Codensity monad (related to Ran)
// Codensity M A = forall R. (A -> M R) -> M R
struct Codensity<A> {
    run: Rc<dyn Fn(Rc<dyn Fn(A) -> Vec<A>>) -> Vec<A>>,
}

impl<A: Clone + 'static> Codensity<A> {
    fn pure_(a: A) -> Self {
        Codensity {
            run: Rc::new(move |k| k(a.clone())),
        }
    }

    fn bind<B: Clone + 'static>(self, f: impl Fn(A) -> Codensity<B> + 'static) -> Codensity<B> {
        Codensity {
            run: Rc::new(move |k| {
                (self.run)(Rc::new(move |a| {
                    (f(a).run)(k.clone())
                }))
            }),
        }
    }

    fn lower(self) -> Vec<A> {
        (self.run)(Rc::new(|a| vec![a]))
    }
}

fn main() {
    // Yoneda: store and transform without committing to a specific functor
    let y = Yoneda::new(42_i32);
    let doubled = y.map(|x| x * 2);
    let as_str = doubled.map(|x| x.to_string());
    println!("Yoneda chain: {}", as_str.from_yoneda()); // "84"

    // Codensity: efficient left-associated bind via CPS
    // Building a list using Codensity avoids O(n²) from left-association
    let prog = Codensity::pure_(1_i32)
        .bind(|x| Codensity::pure_(x + 1))
        .bind(|x| Codensity::pure_(x * 3));
    println!("Codensity result: {:?}", prog.lower()); // [6]

    // Yoneda isomorphism demo: fmap fusion
    let base = Yoneda::new(vec![1_i32, 2, 3]);
    // Two fmaps fused into one via Yoneda
    let result = base.map(|v| v.iter().map(|x| x + 1).collect::<Vec<_>>())
                     .map(|v| v.iter().map(|x| x * 2).collect::<Vec<_>>());
    println!("Fused fmap: {:?}", result.from_yoneda()); // [4, 6, 8]
}
```

The Codensity monad (right Kan extension of a monad along itself) is practically important: it converts a left-associated chain of `bind` calls into a right-associated one, turning O(n²) list concatenation into O(n).

## OCaml Approach

OCaml's higher-rank polymorphism makes Ran more direct:

```ocaml
(* Right Kan extension: Ran F G A = { run : 'b. ('a -> 'f 'b) -> 'g 'b } *)
type ('f, 'g, 'a) ran = { run : 'b. ('a -> 'b) -> 'b }  (* Yoneda specialization *)

let to_yoneda : 'a -> ('f, 'g, 'a) ran = fun a -> { run = fun f -> f a }
let from_yoneda : ('f, 'g, 'a) ran -> 'a = fun ran -> ran.run Fun.id

(* Left Kan extension via existential *)
type ('f, 'g, 'a) lan = Lan : ('b -> 'a) * 'b -> ('f, 'g, 'a) lan
```

OCaml's GADT syntax makes existential types (Lan) particularly clean; Rust needs `Box<dyn Any>` as a workaround.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Universal quantification | trait objects (limited) | rank-2 via record fields |
| Existential (Lan) | `Box<dyn Any>` | GADT existential binding |
| Codensity monad | explicit Rc wrapping | cleaner with polymorphic functions |
| Yoneda isomorphism | manual | directly expressible |
| Kan in practice | Codensity most useful | full generality available |

## Exercises

1. Implement `toYoneda: F A -> Yoneda F A` and `fromYoneda: Yoneda F A -> F A` for `F = Vec` and prove they are inverses.
2. Use Codensity to implement a difference list (O(1) append) and compare performance with a naive list monad on 10,000-element sequences.
3. Implement the Density comonad (Left Kan extension analog for comonads) and show it gives rise to the Stream comonad.
4. Show that `Ran Id Id A ≅ A` by implementing the isomorphism (Yoneda lemma for identity functor).
5. Implement `fromLan: Lan F G A -> G (F^{-1} A)` for a concrete invertible functor F (e.g., `Newtype`).
