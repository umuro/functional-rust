📖 **[View on hightechmind.io →](https://hightechmind.io/rust/187-free-monad-state)**

---

# Free Monad with State
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Combining the free monad pattern with state — a mutable counter, accumulator, or environment — requires threading state through the interpreter. A state-carrying free monad DSL enables programs that read and write state as pure operations, with the actual state management happening in the interpreter. This models how database transactions, accumulating computations, and stateful protocols can be expressed purely and tested without mutable global state.

## Learning Outcomes

- Extend a free monad DSL with `Get` and `Put` state operations
- Implement an interpreter that threads state through computation
- See how stateful programs are expressed as pure trees with state operations at the leaves
- Understand the connection between free monad state and the state monad

## Rust Application

A stateful DSL adds `Get(Box<dyn FnOnce(i32) -> Program<A>>)` and `Put(i32, Box<Program<A>>)` operations. The program `get().bind(|n| put(n + 1))` increments a counter without actually reading or writing any mutable state — it builds a data structure. The interpreter `run_state(program, initial_state: i32) -> (A, i32)` threads state through: `Get(k) => run_state(k(state), state)`, `Put(new_state, cont) => run_state(*cont, new_state)`. The final state is returned alongside the result.

## OCaml Approach

OCaml's state monad and free monad can be combined:
```ocaml
type _ state_op =
  | Get : (int -> 'a state_op) -> 'a state_op
  | Put : int * (unit -> 'a state_op) -> 'a state_op
  | Pure : 'a -> 'a state_op
```
The interpreter `run_state : 'a state_op -> int -> 'a * int` has the same structure. OCaml 5's effect handlers can express this more directly: `effect Get : int` and `effect Put : int -> unit` with an `match_with` handler that carries state.

## Key Differences

1. **State threading**: Both pass state as an accumulator through recursive calls; neither uses mutable state in the interpreter.
2. **OCaml effects**: OCaml 5 effects replace free monad state with a cleaner `effect Get : int` declaration and a single `match_with` handler.
3. **Composability**: Free monad state composes with other DSL operations (Print, ReadLine) by building larger sum types; OCaml effects compose automatically.
4. **Stack depth**: Stateful programs with many operations risk stack overflow; both languages need trampolining for deeply recursive state operations.

## Exercises

1. Add a `Modify(f: Box<dyn Fn(i32) -> i32>, continuation: ...)` operation that applies a function to the state.
2. Implement a `run_state_with_log(program, state)` interpreter that logs each `Get` and `Put` operation.
3. Combine the console DSL (example 185) with state to build a program that counts the number of lines read.
