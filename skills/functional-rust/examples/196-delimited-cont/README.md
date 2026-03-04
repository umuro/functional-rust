# 196: Delimited Continuations

**Difficulty:** ⭐⭐⭐⭐⭐  **Level:** Expert

Capture "the rest of the computation up to a delimiter" as a reusable function — the mind-bending abstraction that underlies async/await, generators, and nondeterministic search.

## The Problem This Solves

Normal exceptions let you jump up the call stack and throw away everything below the catch point. That's useful, but sometimes you want to jump up *and come back*. Imagine a generator: `yield 5` pauses execution, hands `5` to the caller, and when the caller asks for more, execution *resumes from exactly that point*. Or nondeterminism: `choose [1, 2, 3]` runs "the rest of the computation" three times, once for each choice. Both of these require capturing "the continuation" — the rest of the program from the current point — as a callable function.

**Delimited continuations** are that mechanism. `reset` marks a boundary (the "delimiter"). `shift` captures everything between the current point and the nearest `reset` as a function `k`, then passes `k` to a user-provided function that can call `k` zero, one, or many times. Call it zero times: abort (like an exception). Call it once: transform (like a monad). Call it many times: nondeterminism, generators, backtracking.

`async/await` in every language is implemented with delimited continuations internally. `await` is `shift`: it captures the continuation (rest of the async function), passes it to the runtime, and the runtime calls it when the future resolves. Generators (`yield`) work the same way. Delimited continuations are the unified foundation.

## The Intuition

Imagine a branching choose-your-own-adventure book. `reset` is the spine of the book. `shift` (via `choose`) is a page that says "go to page 42 if you chose A, go to page 97 if you chose B." Each branch *continues reading the book from the same point*. The "continuation" `k` is like a photocopier: it copies the rest of the book from the current page, so you can hand a copy to each branch. Both branches continue from the same place, independently.

In code, `choose [1, 2, 3]` captures `k` = "everything that comes after the choose call up to the `reset`." It calls `k(1)`, `k(2)`, `k(3)`, and collects all results. This is exactly the list monad — which is why the list monad and delimited continuations produce identical behavior for nondeterminism.

## How It Works in Rust

```rust
// Approach 1: List monad — direct translation, most idiomatic

fn nd_bind<A, B, F: Fn(A) -> Vec<B>>(xs: Vec<A>, f: F) -> Vec<B> {
    xs.into_iter().flat_map(f).collect()  // flat_map IS the nondeterminism handler
}

fn choose<A: Clone>(xs: Vec<A>) -> Vec<A> { xs }  // "perform shift"
fn nd_guard(cond: bool) -> Vec<()> { if cond { vec![()] } else { vec![] } }

// Find Pythagorean triples — each choose runs "the rest" for every value:
let triples = nd_bind(choose((1..=10).collect()), |a| {
    nd_bind(choose((a..=10).collect()), move |b| {
        nd_bind(choose((b..=10).collect()), move |c| {
            nd_bind(nd_guard(a*a + b*b == c*c), move |_| vec![(a,b,c)])
        })
    })
});
// [(3,4,5), (6,8,10)] — all solutions, automatically explored

// Approach 2: CPS (continuation-passing style) shift/reset
// Closer to the formal definition; useful for understanding the semantics.

use std::rc::Rc;
type Cont<A, R> = Rc<dyn Fn(A) -> R>;
type DelimComp<A, R> = Rc<dyn Fn(Cont<A, R>) -> R>;

// reset(m): run m with the identity continuation (no transformation)
fn dc_reset<A: Clone + 'static>(m: DelimComp<A, A>) -> A {
    m(Rc::new(|x| x))  // "return yourself" = the delimiter
}

// shift(g): g receives the current continuation k
fn dc_shift<A: 'static, R: 'static, G: Fn(Cont<A, R>) -> R + 'static>(g: G) -> DelimComp<A, R> {
    Rc::new(move |k: Cont<A, R>| g(k))  // hand k to g; g decides what to do with it
}

// Nondeterminism via CPS: choose_cps calls k once per value, collects results
fn choose_cps<A: Clone + 'static>(xs: Vec<A>) -> DelimComp<A, Vec<A>> {
    Rc::new(move |k: Cont<A, Vec<A>>| {
        xs.iter().flat_map(|x| k(x.clone())).collect()
        //                     ^^^^^^^^^^^^ run continuation with each choice
    })
}

// The deep insight: choose_cps IS dc_shift with g = |k| xs.iter().flat_map(k).collect()
// And nd_reset_cps IS dc_reset with answer type Vec<A>
// The list monad and CPS shift/reset compute the same thing — proven here.

fn nd_reset_cps<A: Clone + 'static>(m: DelimComp<A, Vec<A>>) -> Vec<A> {
    m(Rc::new(|x| vec![x]))  // base case: wrap single result in vec
}

let results = nd_reset_cps(dc_bind(
    choose_cps(vec![1_i32, 2, 3]),
    Rc::new(|x| dc_bind(
        choose_cps(vec![10_i32, 20]),
        Rc::new(move |y| nd_return_cps(x + y)),
    )),
));
// [11, 21, 12, 22, 13, 23] — all sums

// Approach 3: Permutations — classic nondeterminism application
fn perms(list: Vec<i32>) -> Vec<Vec<i32>> {
    if list.is_empty() { return vec![vec![]]; }
    nd_bind(list.clone(), |x| {
        let rest: Vec<i32> = list.iter().filter(|&&y| y != x).cloned().collect();
        nd_bind(perms(rest), move |mut tail| {
            tail.insert(0, x);
            vec![tail]
        })
    })
}
// perms(vec![1,2,3]) → 6 permutations
```

## What This Unlocks

- **Understanding async/await** — `await` is `shift`: capture the rest of the async function as a continuation, register it with the runtime, resume when the future is ready. Every `async` function compiles to a state machine encoding exactly this.
- **Prolog-style backtracking** — constraint solvers, type unification, regex matching, SAT solvers all use nondeterminism with pruning via `nd_guard`; delimited continuations are the clean semantic model.
- **Effect handler implementation** — OCaml 5's effect system is implemented using delimited continuations under the hood. Understanding shift/reset explains why `continue k value` works the way it does.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| reset | `match f () with \| v -> v \| effect (Shift k) handler -> handler k` | `dc_reset(m)` = `m(Rc::new(\|x\| x))` in CPS; or implicit in list monad |
| shift | `shift f = perform (Shift f)` — captures continuation natively | `dc_shift(g)` = `Rc::new(\|k\| g(k))` in CPS; `choose` = `flat_map` in list monad |
| Resuming `k` multiple times | Native — OCaml continuations are heap-allocated, can be called N times | `Rc<dyn Fn(A) -> R>` (cloneable, multi-use); `Box<dyn FnOnce>` can't be cloned |
| Nondeterminism equivalent | `choose xs = shift (fun k -> List.concat_map k xs)` | `choose(xs) = xs; nd_bind = flat_map` — list monad is the Rust idiom |
| Use in production | OCaml 5 effects and continuations are production-ready | CPS is academic; use iterators/rayon for real nondeterminism; use tokio for continuations |
