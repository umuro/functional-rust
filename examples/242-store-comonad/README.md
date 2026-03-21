[![Functional Rust](https://img.shields.io/badge/functional--rust-examples-blue)](https://hightechmind.io)

# Store Comonad
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The Store comonad models a mutable reference into a larger data structure: given a "getter" function and a current focus position, you can read the value at focus, move the focus elsewhere, or derive new stores by transforming the getter. This captures the essence of how lenses work under the hood. In Rust, implementing Store reveals how comonadic structure enables elegant data-access abstractions without mutation.

## Learning Outcomes

- Understand Store as a pair `(s -> a, s)` — a getter and a current index
- Implement `extract`, `extend`, and `duplicate` for Store
- See how Store comonad relates to the `Lens` abstraction
- Explore `seek` (reposition) and `peek` (read at arbitrary position)
- Compare Store comonad with OCaml's equivalent using records and closures

## Rust Application

Store comonad in Rust pairs a shared function with a position value:

```rust
struct Store<S, A> {
    getter: Rc<dyn Fn(S) -> A>,
    pos: S,
}

impl<S: Clone, A> Store<S, A> {
    fn new(getter: impl Fn(S) -> A + 'static, pos: S) -> Self {
        Store { getter: Rc::new(getter), pos }
    }

    // Comonad: extract — run the getter at the current position
    fn extract(&self) -> A {
        (self.getter)(self.pos.clone())
    }

    // Reposition without changing the getter
    fn seek(&self, pos: S) -> Store<S, A> {
        Store { getter: Rc::clone(&self.getter), pos }
    }

    // Read at a different position without moving
    fn peek(&self, pos: S) -> A {
        (self.getter)(pos)
    }
}

impl<S: Clone + 'static, A: 'static> Store<S, A> {
    // Comonad: extend — given w a -> b, produce w b
    fn extend<B>(&self, f: impl Fn(&Store<S, A>) -> B + 'static) -> Store<S, B> {
        let getter = Rc::clone(&self.getter);
        let pos = self.pos.clone();
        let f = Rc::new(f);
        Store::new(
            move |s| {
                let inner = Store { getter: Rc::clone(&getter), pos: s };
                f(&inner)
            },
            pos,
        )
    }

    // Comonad: duplicate — Store s a -> Store s (Store s a)
    fn duplicate(&self) -> Store<S, Store<S, A>> {
        let getter = Rc::clone(&self.getter);
        let pos = self.pos.clone();
        Store::new(
            move |s| Store { getter: Rc::clone(&getter), pos: s },
            pos,
        )
    }
}

// Using Store as a lens-like accessor
fn main() {
    let arr = vec![10, 20, 30, 40, 50];
    let store: Store<usize, i32> = Store::new(
        move |i| arr[i],
        2, // focus at index 2
    );

    println!("At current pos: {}", store.extract()); // 30
    println!("Peek at 0: {}", store.peek(0));          // 10

    // extend: compute neighborhood average
    let averaged = store.extend(|s| {
        let prev = if s.pos > 0 { s.peek(s.pos - 1) } else { s.extract() };
        let curr = s.extract();
        let next = s.peek(s.pos + 1); // assume valid bounds
        (prev + curr + next) / 3
    });
    println!("Averaged at pos 2: {}", averaged.extract()); // (20+30+40)/3 = 30
}
```

The `extend` operation is the key: it takes a "context-aware" function and lifts it into the entire store.

## OCaml Approach

In OCaml, Store is a record with a functional getter:

```ocaml
type ('s, 'a) store = { pos: 's; getter: 's -> 'a }

let extract { pos; getter } = getter pos
let seek s { getter; _ } = { pos = s; getter }
let peek s { getter; _ } = getter s

let extend f w =
  { pos = w.pos
  ; getter = fun s -> f { pos = s; getter = w.getter } }

let duplicate w =
  { pos = w.pos
  ; getter = fun s -> { pos = s; getter = w.getter } }
```

OCaml's polymorphic records make the types cleaner; Rust's `Rc<dyn Fn>` achieves the same but requires explicit sharing.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Getter sharing | `Rc<dyn Fn(S) -> A>` | closure captured in record |
| Clone constraint | `S: Clone` required | structural copy for records |
| extend lifetime | `'static` bounds needed | no lifetime tracking |
| Lens connection | explicit derivation | directly via records |
| Performance | zero-cost with monomorphization | GC-managed closures |

Store comonad is the semantic foundation of the van Laarhoven lens encoding. Lenses are exactly Store coalgebras: `a -> Store b b` with the right coherence laws.

## Exercises

1. Implement `fmap` for `Store<S, A>` — map over the result type without changing the position.
2. Verify the three comonad laws (left identity, right identity, associativity) with unit tests.
3. Build a 2D `Store<(usize, usize), Cell>` that represents a Game of Life grid and apply `extend` to compute one generation step.
4. Implement `experiment`: given a functor of positions `F<S>`, collect `F<A>` results by peeking at each position.
5. Show that `Store s` is equivalent to a coalgebra `a -> s -> a` paired with an `s`, connecting it to the lens definition.
