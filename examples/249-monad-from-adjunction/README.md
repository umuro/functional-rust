[![Functional Rust](https://img.shields.io/badge/functional--rust-examples-blue)](https://hightechmind.io)

# Monad from Adjunction
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Every adjunction `F ⊣ G` gives rise to a monad `G ∘ F` with unit `η` and multiplication `G(ε_F)`, and a comonad `F ∘ G` with counit `ε` and comultiplication `F(η_G)`. This is the deepest connection between adjunctions, monads, and comonads. The State monad arises from the product/exponential adjunction; the List monad from the free/forgetful adjunction. This example derives the State monad and its dual the Env comonad from first principles using the product adjunction.

## Learning Outcomes

- Derive the State monad as `G ∘ F` where `F = (- × S)` and `G = (S → -)`
- Implement `return` as the adjunction unit `η` and `join` as `G(ε_F)`
- Derive the Env comonad as `F ∘ G` from the same adjunction
- Verify that the monad laws follow from the adjunction coherence conditions
- Compare this systematic derivation with OCaml's more direct categorical encoding

## Rust Application

Deriving State monad and Env comonad from the product adjunction:

```rust
// The product adjunction: F ⊣ G where
//   F A = (A, S)   ("left functor" = product with S)
//   G B = S -> B   ("right functor" = exponential from S)
//
// Unit    η_A: A -> G(F(A)) = A -> (S -> (A, S))
//              η_A(a)(s) = (a, s)
//
// Counit  ε_B: F(G(B)) = ((S -> B), S) -> B
//              ε_B(f, s) = f(s)

// --- Derived monad M = G ∘ F ---
// M A = G(F(A)) = S -> (A, S)   <--- this IS State!

struct State<S, A>(Box<dyn Fn(S) -> (A, S)>);

impl<S: Clone + 'static, A: 'static> State<S, A> {
    // return = η_A = adjunction unit
    fn ret(a: A) -> Self where A: Clone {
        State(Box::new(move |s| (a.clone(), s)))
    }

    // join = G(ε_{F A}):
    // join: M(M A) = (S -> (S -> (A, S), S)) -> (S -> (A, S))
    // join(mma)(s) = let (ma, s') = mma(s) in ma(s')
    fn join(mma: State<S, State<S, A>>) -> State<S, A> {
        State(Box::new(move |s| {
            let (inner_state, s1) = (mma.0)(s);
            (inner_state.0)(s1)
        }))
    }

    // bind derived from join + fmap
    fn bind<B: 'static>(self, f: impl Fn(A) -> State<S, B> + 'static) -> State<S, B> {
        State(Box::new(move |s| {
            let (a, s1) = (self.0)(s);
            (f(a).0)(s1)
        }))
    }

    fn run(self, s: S) -> (A, S) { (self.0)(s) }
    fn eval(self, s: S) -> A where S: 'static { self.run(s).0 }
    fn exec(self, s: S) -> S where A: 'static { self.run(s).1 }

    fn get() -> State<S, S> {
        State(Box::new(|s: S| (s.clone(), s)))
    }

    fn put(new_s: S) -> State<S, ()> {
        State(Box::new(move |_| ((), new_s.clone())))
    }

    fn modify(f: impl Fn(S) -> S + 'static) -> State<S, ()> {
        State(Box::new(move |s| ((), f(s))))
    }
}

// --- Derived comonad W = F ∘ G ---
// W B = F(G(B)) = ((S -> B), S)   <--- this IS Env!

#[derive(Clone)]
struct Env<S, A> {
    getter: std::rc::Rc<dyn Fn(S) -> A>,
    pos: S,
}

impl<S: Clone + 'static, A: Clone + 'static> Env<S, A> {
    // extract = ε_B = adjunction counit: (S->B, S) -> B
    fn extract(&self) -> A { (self.getter)(self.pos.clone()) }

    // duplicate = F(η_{G B}): W B -> W(W B)
    fn duplicate(&self) -> Env<S, Env<S, A>> {
        let getter = self.getter.clone();
        let pos = self.pos.clone();
        Env {
            getter: std::rc::Rc::new(move |s| Env {
                getter: getter.clone(),
                pos: s,
            }),
            pos,
        }
    }

    fn extend<B: Clone + 'static>(&self, f: impl Fn(&Env<S, A>) -> B + 'static) -> Env<S, B> {
        let getter = self.getter.clone();
        let pos = self.pos.clone();
        Env {
            getter: std::rc::Rc::new(move |s| {
                let local = Env { getter: getter.clone(), pos: s };
                f(&local)
            }),
            pos,
        }
    }
}

fn main() {
    // State monad: stack calculator derived from adjunction
    let program: State<Vec<i32>, i32> = State::get()
        .bind(|_| State::put(vec![]))
        .bind(|_| State::modify(|mut s| { s.push(10); s }))
        .bind(|_| State::modify(|mut s| { s.push(20); s }))
        .bind(|_| State::get().bind(|stack| {
            let sum: i32 = stack.iter().sum();
            State::ret(sum)
        }));

    let (result, final_stack) = program.run(vec![]);
    println!("Sum: {result}, stack: {:?}", final_stack); // 30, [10, 20]

    // Monad law: left identity  return a >>= f  =  f a
    let f = |x: i32| State::ret(x * 2);
    let lhs = State::<Vec<i32>, i32>::ret(5).bind(f).eval(vec![]);
    let rhs = State::<Vec<i32>, i32>::ret(10).eval(vec![]);
    println!("Left identity: lhs={lhs}, rhs={rhs}"); // both 10

    // Env comonad: configuration scorer derived from adjunction
    let cfg_fn: std::rc::Rc<dyn Fn(f64) -> String> =
        std::rc::Rc::new(|threshold: f64| {
            if threshold > 0.5 { "HIGH".to_string() } else { "LOW".to_string() }
        });
    let env = Env { getter: cfg_fn, pos: 0.7_f64 };
    println!("Env extract: {}", env.extract()); // "HIGH"

    let sized = env.extend(|e| format!("{} (at {:.1})", e.extract(), e.pos));
    println!("Env extended: {}", sized.extract()); // "HIGH (at 0.7)"
}
```

The monad laws for State follow directly from the adjunction coherence conditions (triangle identities), making the derivation proof-relevant.

## OCaml Approach

OCaml's module system makes the adjunction derivation structurally cleaner:

```ocaml
module type ADJUNCTION = sig
  type s
  (* F A = (A * s), G B = s -> B *)
  val unit   : 'a -> s -> ('a * s)   (* eta *)
  val counit : (s -> 'b) * s -> 'b   (* epsilon *)
end

(* Derived monad: G o F A = s -> (A * s) *)
let return_ a s = (a, s)
let join mma s = let (ma, s') = mma s in ma s'
let bind ma f s = let (a, s') = ma s in f a s'

(* Derived comonad: F o G B = (s -> B) * s *)
let extract (f, s) = f s
let duplicate (f, s) = (fun s' -> (f, s'), s)
```

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Adjunction encoding | traits + structs | module signatures |
| `join` derivation | manual `Box` wrapping | direct function application |
| Monad laws | require `S: Clone + 'static` | implicit polymorphism |
| Law verification | runtime tests | qcheck property tests |
| Generality | product adjunction only | full module parametricity |

This derivation demonstrates that State and Env are not ad-hoc constructions but arise necessarily and uniquely from the product adjunction, with no choices involved.

## Exercises

1. Derive the `Writer<W>` monad from the adjunction `(- × W) ⊣ (W →)` where W is a monoid; implement `tell`, `listen`, and `pass`.
2. Verify the monad laws for the derived State monad using property tests: left identity, right identity, and associativity.
3. Derive the `Reader<R>` monad from the diagonal/product adjunction and implement `ask` and `local`.
4. Implement the monad morphism from `State<S, A>` to `Reader<S, (A, S)>` (which is just the function type) and show it respects `return` and `bind`.
5. Show that the unit `η` and counit `ε` of the adjunction satisfy the triangle identities computationally by writing tests that check `G(ε) ∘ ηG = idG` and `ε_F ∘ F(η) = idF`.
