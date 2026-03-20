📖 **[View on hightechmind.io →](https://hightechmind.io/rust/859-state-monad)**

---

# State Monad

## Problem Statement

Threading state through a sequence of functions without the State monad requires passing the state explicitly as an argument and returning it alongside the result: `fn step(input: T, state: S) -> (R, S)`. This is error-prone and noisy. The State monad encapsulates this threading: `State<S, A>` represents a computation `S -> (A, S)` that reads and modifies state. Computations are composed without explicit state passing — the monad handles threading. This pattern appears in: compiler passes (threading symbol tables), game state machines, configuration accumulation, and embedded DSLs. It makes stateful computation composable and testable while remaining purely functional.

## Learning Outcomes

- Understand `State<S, A>` as a wrapper around `FnOnce(S) -> (A, S)`
- Implement `get()` returning current state, `put(s)` replacing state, `modify(f)` transforming state
- Implement monadic bind: `state.then(|a| next_state)` threading state through both computations
- Use `run_state(initial)` to execute the computation and get `(result, final_state)`
- Recognize the tension with Rust's ownership: `FnOnce` vs `Fn` based on state mutation needs

## Rust Application

```rust
pub struct State<S, A> {
    run: Box<dyn FnOnce(S) -> (A, S)>,
}
impl<S: 'static, A: 'static> State<S, A> {
    pub fn new(f: impl FnOnce(S) -> (A, S) + 'static) -> Self {
        State { run: Box::new(f) }
    }
    pub fn run_state(self, s: S) -> (A, S) { (self.run)(s) }
    pub fn get() -> State<S, S> where S: Clone {
        State::new(|s: S| (s.clone(), s))
    }
    pub fn put(new_s: S) -> State<S, ()> {
        State::new(|_| ((), new_s))
    }
}
```

`Box<dyn FnOnce(S) -> (A, S)>` is necessary because the State monad contains arbitrary computations. `FnOnce` allows the closure to consume captured values — important when state contains non-Clone types. The `'static` bound prevents dangling references in the boxed closure. `get()` clones the state to return both the state value and the unchanged state. `put()` ignores the old state and replaces it. The `run_state(initial)` executes the full stateful computation.

## OCaml Approach

OCaml represents State as `type ('s, 'a) state = State of ('s -> 'a * 's)`. The `run_state (State f) s = f s`. Monadic bind: `let bind (State f) k = State (fun s -> let (a, s') = f s in let State g = k a in g s')`. `get = State (fun s -> (s, s))`, `put s = State (fun _ -> ((), s))`. OCaml's algebraic types make the State monad clean and readable. The `ppx_let` extension provides `let%bind` syntax for threading state.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Type | `Box<dyn FnOnce(S) -> (A, S)>` | `State of ('s -> 'a * 's)` |
| `FnOnce` vs `Fn` | Must choose based on use | `fun s -> ...` (always `Fn`) |
| Bind implementation | Complex with boxing | Clean algebraic unwrap |
| `get` | Requires `S: Clone` | Same (returns clone in pure) |
| Thread safety | `Send + Sync` bounds needed | Not applicable (single-threaded) |
| `'static` bound | Required for boxed closures | Not required |

## Exercises

1. Implement monadic bind for `State<S, A>` and use it to compose `get`, a transform, and `put` into a single computation.
2. Implement a stack using the State monad: `push` and `pop` operations as `State<Vec<T>, Option<T>>` computations.
3. Use the State monad to implement a simple counter that increments and returns the new count at each step.
4. Compare the State monad approach with explicit state threading: implement the same computation both ways.
5. Implement `modify(f: S -> S) -> State<S, ()>` using `get` and `put` and verify it equals `State::new(|s| ((), f(s)))`.
