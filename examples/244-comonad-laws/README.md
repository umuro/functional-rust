[![Functional Rust](https://img.shields.io/badge/functional--rust-examples-blue)](https://hightechmind.io)

# Comonad Laws

## Problem Statement

Just as monads must satisfy left identity, right identity, and associativity, comonads must satisfy three dual laws. These laws ensure that `extract` and `extend` compose predictably, making comonadic abstractions composable and refactorable without hidden surprises. This example formalizes and verifies the comonad laws in Rust using property-based tests on concrete comonad instances.

## Learning Outcomes

- State the three comonad laws precisely: left identity, right identity, and associativity
- Understand each law as the categorical dual of the corresponding monad law
- Write property-based tests that verify the laws hold for Identity, Env, and Store comonads
- Recognize when a candidate `extend` implementation violates a law
- Connect comonad laws to the coherence conditions of comonadic algebras

## Rust Application

The three comonad laws, with verification:

```rust
// Law 1 — Left identity:  extend extract = id
// Law 2 — Right identity: extract . extend f = f
// Law 3 — Associativity:  extend f . extend g = extend (f . extend g)

trait Comonad: Clone {
    type Val: Clone;
    fn extract(&self) -> Self::Val;
    fn extend<B: Clone>(&self, f: impl Fn(&Self) -> B + Clone) -> impl Comonad<Val = B>;
}

// ---- Concrete: Identity comonad ----
#[derive(Clone, Debug, PartialEq)]
struct Identity<A: Clone>(A);

impl<A: Clone + PartialEq + 'static> Identity<A> {
    fn extract(&self) -> A { self.0.clone() }

    fn extend<B: Clone + 'static>(&self, f: impl Fn(&Identity<A>) -> B) -> Identity<B> {
        Identity(f(self))
    }
}

// Verify Law 1: extend extract = id
fn law1_identity<A: Clone + PartialEq + std::fmt::Debug>(w: Identity<A>) -> bool {
    let extended = w.extend(|x| x.extract());
    extended == w
}

// Verify Law 2: extract (extend f w) = f w
fn law2_identity<A: Clone + PartialEq + std::fmt::Debug, B: Clone + PartialEq + std::fmt::Debug>(
    w: Identity<A>,
    f: impl Fn(&Identity<A>) -> B,
) -> bool {
    let f2 = |x: &Identity<A>| f(x);
    w.extend(&f2).extract() == f(&w)
}

// Verify Law 3: extend f (extend g w) = extend (fun x -> f (extend g x)) w
fn law3_identity<
    A: Clone + PartialEq + std::fmt::Debug + 'static,
    B: Clone + PartialEq + std::fmt::Debug + 'static,
    C: Clone + PartialEq + std::fmt::Debug,
>(
    w: Identity<A>,
    g: impl Fn(&Identity<A>) -> B + Clone + 'static,
    f: impl Fn(&Identity<B>) -> C,
) -> bool {
    let g2 = g.clone();
    let lhs = w.extend(g).extend(|x| f(x));
    let rhs = w.extend(move |x| f(&x.extend(g2.clone())));
    lhs == rhs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_law1_left_identity() {
        for val in [1, 42, -7, 0, 100] {
            assert!(law1_identity(Identity(val)), "Law 1 failed for {val}");
        }
    }

    #[test]
    fn test_law2_right_identity() {
        let double = |w: &Identity<i32>| w.extract() * 2;
        for val in [1, 5, 10] {
            assert!(law2_identity(Identity(val), double), "Law 2 failed for {val}");
        }
    }

    #[test]
    fn test_law3_associativity() {
        let g = |w: &Identity<i32>| w.extract() + 1;
        let f = |w: &Identity<i32>| w.extract() * 3;
        for val in [0, 1, 4, 9] {
            assert!(law3_identity(Identity(val), g, f), "Law 3 failed for {val}");
        }
    }

    #[test]
    fn test_env_comonad_laws() {
        // Env<String, i32>: environment is config string, value is integer
        use std::collections::HashMap;
        // extend extract = id
        let env = (String::from("prod"), 42_i32);
        let extract = |(_, v): &(String, i32)| *v;
        let extend_env = |(e, v): &(String, i32), f: &dyn Fn(&(String, i32)) -> i32| {
            (e.clone(), f(&(e.clone(), *v)))
        };
        let extended = extend_env(&env, &|x| extract(x));
        assert_eq!(extended.1, env.1, "Env Law 1 failed");
    }
}
```

The Identity comonad is the simplest witness: every comonad law reduces to a trivial equality, making it ideal for initial testing.

## OCaml Approach

In OCaml the laws are typically stated as signatures in a functor:

```ocaml
module type COMONAD = sig
  type 'a t
  val extract : 'a t -> 'a
  val extend  : ('a t -> 'b) -> 'a t -> 'b t
  (* Laws (as comments, verified by quickcheck) *)
  (* extend extract = Fun.id *)
  (* extract (extend f x) = f x *)
  (* extend f (extend g x) = extend (fun y -> f (extend g y)) x *)
end
```

QuickCheck/qcheck handles property verification; Rust uses proptest or hand-written loops.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Law encoding | unit tests + generics | module signature + qcheck |
| Trait object limits | `impl Fn` not object-safe | first-class polymorphism |
| Verification | compile-time + runtime | mostly runtime via qcheck |
| `PartialEq` constraint | required for equality tests | structural equality built in |
| Laws as types | not enforceable | not enforceable (same limitation) |

Neither language can enforce comonad laws at the type level — they remain runtime-verifiable contracts. This mirrors the situation with monad laws.

## Exercises

1. Implement the laws for the `Store<usize, i32>` comonad using a fixed finite array as the getter.
2. Find an `extend`-like operation that satisfies laws 1 and 2 but not 3 (associativity); explain which law breaks and why.
3. Write a QuickCheck/proptest harness that checks all three laws for `Identity<i32>` with random values.
4. Prove algebraically (on paper) that `duplicate` defined as `extend id` always satisfies the comonad laws if `extend` does.
5. Implement a `ComonadLaws<W>` trait with associated functions that check all three laws given a sample value, and implement it for both `Identity` and `Env`.
