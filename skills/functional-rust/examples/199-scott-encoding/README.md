# 199: Scott Encoding — Pattern Matching as Functions

**Difficulty:** 4  **Level:** Expert

Represent algebraic data types as functions — proving that pattern matching is just function application.

## The Problem This Solves

You use `match` every day in Rust. It dispatches on variants, extracts values, handles all cases. It feels like a fundamental language feature — built in, irreducible.

But is `match` actually fundamental? Or is it syntactic sugar for something simpler?

Scott encoding answers: `match` is function application in disguise. Every `enum` variant can be encoded as a function that takes one callback per variant. To "pattern match" on it, you call it with your callbacks. The variant itself decides which callback to invoke — and that IS pattern matching.

The practical implications go beyond the theoretical: Scott encoding is how dependently typed languages (Coq, Agda, Lean) implement elimination of data types. It's how some runtime systems implement pattern matching efficiently. And it shows up in advanced Rust whenever you design types that accept visitor-style callbacks — which is a direct translation of Scott encoding.

Understanding Scott encoding means understanding that `Option<T>` isn't magic. It's just a function that takes two callbacks: one for `None`, one for `Some`. This example exists to show you that pattern matching and function application are the same thing.

## The Intuition

In Rust you write:
```rust
match my_option {
    None      => do_nothing(),
    Some(val) => do_something(val),
}
```

Scott encoding says: what if `my_option` itself was a function that *took* `do_nothing` and `do_something` as arguments?

```
None       = a function that calls k_none and ignores k_some
Some(x)    = a function that calls k_some(x) and ignores k_none
```

Then pattern matching becomes:
```rust
my_option(do_nothing, do_something)
// The OPTION decides which callback to invoke — that's pattern matching!
```

Compare to Church encoding: Church encoding is about *iteration* (apply f N times). Scott encoding is about *case analysis* (which constructor was used?). Scott-encoded data IS its own eliminator.

## How It Works in Rust

**Pure functional Scott option (closure-based, as in OCaml):**
```rust
use std::rc::Rc;

// A Scott-encoded Option<i32> IS a function:
// given (callback for None, callback for Some), call the right one
type PureScottOpt = Rc<dyn Fn(
    Rc<dyn Fn() -> i32>,      // k_none: called if this is None
    Rc<dyn Fn(i32) -> i32>,   // k_some: called with the value if Some
) -> i32>;

fn pure_none() -> PureScottOpt {
    // None: call k_none, ignore k_some
    Rc::new(|k_none: Rc<dyn Fn() -> i32>, _k_some| k_none())
}

fn pure_some(x: i32) -> PureScottOpt {
    // Some(x): call k_some with x, ignore k_none
    Rc::new(move |_k_none, k_some: Rc<dyn Fn(i32) -> i32>| k_some(x))
}

// "Pattern matching" = just call the option:
let opt = pure_some(42);
let result = opt(
    Rc::new(|| -1),          // None branch: return -1
    Rc::new(|x| x * 2),     // Some branch: double the value
);
assert_eq!(result, 84);  // some(42) → 42 * 2 = 84
```

**Practical Scott Option with enum backing (idiomatic Rust):**
```rust
#[derive(Clone)]
enum ScottOption<A: Clone> {
    None,
    Some(A),
}

impl<A: Clone> ScottOption<A> {
    // The Scott eliminator: give it one function per variant
    fn elim<B, FN: Fn() -> B, FS: Fn(A) -> B>(&self, k_none: FN, k_some: FS) -> B {
        match self {
            ScottOption::None    => k_none(),      // call the None callback
            ScottOption::Some(x) => k_some(x.clone()), // call Some callback with value
        }
    }

    // EVERYTHING is defined via elim — no direct matching needed:
    fn is_none(&self) -> bool      { self.elim(|| true, |_| false) }
    fn is_some(&self) -> bool      { self.elim(|| false, |_| true) }
    fn get_or(&self, default: A) -> A { self.elim(|| default.clone(), |x| x) }

    fn map<B: Clone, F: Fn(A) -> B>(&self, f: F) -> ScottOption<B> {
        self.elim(
            || ScottOption::None,
            |x| ScottOption::Some(f(x)),
        )
    }
}
```

**Scott-encoded List:**
```rust
#[derive(Clone)]
enum ScottList<A: Clone> { Nil, Cons(A, Box<ScottList<A>>) }

impl<A: Clone> ScottList<A> {
    fn elim<B, FN: Fn() -> B, FC: Fn(A, &ScottList<A>) -> B>(
        &self, k_nil: FN, k_cons: FC
    ) -> B {
        match self {
            ScottList::Nil           => k_nil(),
            ScottList::Cons(x, xs)  => k_cons(x.clone(), xs),
        }
    }

    // head/tail/isEmpty — all via elim:
    fn head_or(&self, default: A) -> A { self.elim(|| default.clone(), |x, _| x) }
    fn is_empty(&self) -> bool         { self.elim(|| true, |_, _| false) }
}
```

**Scott naturals — `None` = Zero, `Some(prev)` = Successor:**
```rust
// Scott-encoded Nat: Zero is None, Succ(n) is Some(n)
// This proves Option<Nat> and Nat are structurally the same!
type ScottNat = ScottOption<Box<ScottNatCell>>;

fn nat_zero() -> ScottNat { ScottOption::none() }
fn nat_succ(n: ScottNat) -> ScottNat { ScottOption::some(Box::new(ScottNatCell(n))) }

fn nat_to_usize(n: &ScottNat) -> usize {
    match n {
        ScottOption::None         => 0,
        ScottOption::Some(box_n)  => 1 + nat_to_usize(&box_n.0),
    }
}
```

## What This Unlocks

- **Visitor pattern explained**: Every Rust `Visitor` trait is Scott encoding made explicit — you give one method per variant, and the data decides which to call. Once you see this, visitor patterns click immediately.
- **Proof assistants and type theory**: Dependently typed systems (Coq, Lean, Agda) use Scott eliminators as the *definition* of algebraic types. If you ever want to read formal proofs or verification code, Scott encoding is the vocabulary.
- **Zero-overhead dispatch design**: Scott encoding via `elim` compiles to the same code as `match`. But it's more composable — you can pass the eliminator around as a function, parameterize over it, abstract over data types without knowing their internals.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Scott None | `fun k_none _k_some -> k_none ()` | `Rc::new(\|k_none, _k_some\| k_none())` |
| Scott Some x | `fun _k_none k_some -> k_some x` | `Rc::new(move \|_k_none, k_some\| k_some(x))` |
| Higher-rank types | Natural (`'a. ...`) | Not expressible — needs concrete output type |
| Eliminator | Structurally simple | `impl Trait` or concrete function types |
| vs Church encoding | Church = iteration | Scott = case analysis |
| vs Rust `match` | Two ways to say same thing | `elim` compiles to same code as `match` |
