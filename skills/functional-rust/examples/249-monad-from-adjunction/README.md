# 249: Monad from Adjunction

**Difficulty:** 5  **Level:** Master

Every monad arises from an adjunction — this example shows WHY monads have the structure they do.

## The Problem This Solves

Monads are often introduced as "a design pattern for sequencing effects." That explains *what* they do, but not *why* they have exactly `return` and `bind` (or `return` and `join`), and not why the monad laws look the way they do.

The adjunction perspective answers this. It says: every monad is secretly a pair of functors that go back and forth between two categories, and the monad laws are just the triangle identities of that adjunction. Once you see this, you realize the State monad, the List monad, the Maybe monad, and every other monad are all the same structure at different levels of abstraction.

For working programmers, this matters when you're designing new effect systems or embedding DSLs. Knowing the adjunction underlying your monad tells you exactly what the "free" and "forgetful" parts are — which tells you how to split the monad into two simpler pieces (like splitting State into `get`/`put` primitives).

## The Intuition

An **adjunction** `F ⊣ G` between two functors means: "transforming an `A` into a `G(B)` is the same as transforming an `F(A)` into a `B`." They're inverses in a precise sense, mediated by two natural transformations:

- **Unit** `η : A → G(F(A))` — embed `A` into the round-trip `G(F(A))`
- **Counit** `ε : F(G(B)) → B` — collapse the round-trip `F(G(B))` back to `B`

The monad `M = G ∘ F` is the round-trip. Its structure comes entirely from the adjunction:
- `return = η` — the unit of the adjunction
- `join = G(ε_F(-))` — apply the counit "inside" G

**The State monad** comes from the **currying adjunction**:
- `F(A) = (A, S)` — pair A with state S (product functor)
- `G(B) = S → B` — functions from state S (exponential/reader functor)
- `G ∘ F (A) = S → (A, S)` — exactly the State monad!

The unit `η : A → (S → (A, S))` is `return a = \s -> (a, s)` — produce a value without touching the state.
The counit `ε : (S → (A, S), S) → A` is `run = \(computation, s) -> fst (computation s)` — extract the value by running the computation.

**The List monad** comes from the **Free ⊣ Forgetful adjunction**:
- `F(A) = [A]` — the free monoid over A (just a list)
- `G(M) = underlying set of monoid M`
- `G ∘ F (A) = [A]` — the list monad

`return a = [a]` (singleton list = unit of the free monoid).
`join = concat` (flattening = the free monoid operation applied to a list of lists).

## How It Works in Rust

```rust
// State<S, A> = G ∘ F where F(A) = (A,S), G(B) = S -> B
pub struct State<S, A: Clone> {
    run_fn: Rc<dyn Fn(S) -> (A, S)>,
}

impl<S: Clone + 'static, A: Clone + 'static> State<S, A> {
    // Unit of the adjunction: η_A : A -> G(F(A)) = A -> S -> (A, S)
    pub fn return_(a: A) -> Self {
        State::new(move |s| (a.clone(), s))  // value produced, state unchanged
    }

    // Bind derived from the adjunction structure
    pub fn bind<B>(self, f: impl Fn(A) -> State<S, B> + 'static) -> State<S, B> {
        State::new(move |s| {
            let (a, s2) = self.run(s);  // run first computation, get value + new state
            f(a).run(s2)                // thread new state into second computation
        })
    }
}
```

Primitive operations (the decomposition the adjunction provides):
```rust
// get: read the state as the value — η applied to the state
fn get<S: Clone>() -> State<S, S> {
    State::new(|s: S| (s.clone(), s))
}

// put: replace the state — the "forgetful" direction
fn put<S: Clone>(new_s: S) -> State<S, ()> {
    State::new(move |_| ((), new_s.clone()))
}
```

A complete state program — reads, modifies, reads again:
```rust
let program =
    get::<i64>().bind(|initial|       // read current state
    put(initial + 10).bind(move |_|   // modify state
    get::<i64>().bind(move |after|    // read again
    State::return_((initial, after)))));

let ((before, after), final_state) = program.run(5);
// before=5, after=15, final_state=15
```

The List monad from the Free ⊣ Forgetful adjunction:
```rust
fn list_return<A>(a: A) -> Vec<A> { vec![a] }  // singleton = unit
fn list_bind<A, B>(xs: Vec<A>, f: impl Fn(A) -> Vec<B>) -> Vec<B> {
    xs.into_iter().flat_map(f).collect()  // flat_map = bind
}
fn list_join<A>(xss: Vec<Vec<A>>) -> Vec<A> {
    xss.into_iter().flatten().collect()   // flatten = join = monoid concat
}
```

## What This Unlocks

- **Deriving new monads**: identify an adjunction in your problem domain → the round-trip gives you a monad with the right laws for free.
- **Splitting monads into primitives**: the adjunction tells you what `get`/`put` or `ask`/`tell` should be — the two directions of the round-trip.
- **Understanding monad transformers**: `StateT`, `ReaderT`, and friends stack adjunctions — understanding one level helps reason about the whole stack.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| State type | `type ('s, 'a) state = State of ('s -> 'a * 's)` | `struct State<S, A> { run_fn: Rc<dyn Fn(S) -> (A, S)> }` |
| return (unit η) | `let return_ a = State (fun s -> (a, s))` | `State::new(move \|s\| (a.clone(), s))` — requires Clone |
| bind | Pattern match unwraps `State f` | Method that clones `Rc` to share the closure |
| Sharing state fn | GC handles aliasing | `Rc` for single-threaded; `Arc` for multi-threaded |
| Counit ε | `let run (State f) s = f s` | `self.run_fn(s)` — same |
| List monad join | `List.concat` | `flatten()` |
