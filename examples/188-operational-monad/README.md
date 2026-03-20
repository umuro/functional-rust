📖 **[View on hightechmind.io →](https://hightechmind.io/rust/188-operational-monad)**

---

# Operational Monad
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The operational monad is a variant of the free monad that separates the description of primitive instructions from the composition mechanism (bind/sequence). Instead of encoding both instructions and sequencing in one recursive type, the operational monad has two layers: `Instruction<A>` (user-defined operations) and `Program<A>` (sequencing). This reduces boilerplate compared to free monads and makes adding new instructions simpler — only the instruction type changes.

## Learning Outcomes

- Understand the operational monad as an alternative to free monads
- Learn how `Program<A>` wraps instructions with a universal bind mechanism
- See how the two-layer design simplifies adding new instruction types
- Understand the trade-offs between free monads and the operational monad in Rust

## Rust Application

`enum Instruction<A> { ReadLine(Box<dyn FnOnce(String) -> Program<A>>), Print(String) }` defines the operations. `enum Program<A> { Done(A), Instruction(Instruction<A>, Box<dyn FnOnce(A) -> Program<A>>) }` provides the sequencing layer. New instructions are added only to `Instruction`; the `Program` wrapper stays unchanged. The interpreter pattern-matches on `Program`: `Done(a) => a`, `Instruction(instr, k) => { let result = execute(instr); run(k(result)) }`.

## OCaml Approach

OCaml's operational monad uses GADTs for the instruction type:
```ocaml
type _ instruction =
  | ReadLine : string instruction
  | Print : string -> unit instruction
type _ program =
  | Done : 'a -> 'a program
  | Instr : 'a instruction * ('a -> 'b program) -> 'b program
```
The GADT instruction type ensures that `ReadLine : string instruction` — the instruction's result type is tracked. Rust's version uses boxed closures to simulate this.

## Key Differences

1. **Result type tracking**: OCaml's GADT `'a instruction` encodes the result type per instruction; Rust's `Instruction<A>` simulates this with continuations.
2. **Two layers**: Both separate instruction definitions from sequencing; the operational monad's two-type design is explicit where free monads combine both in one recursive type.
3. **Adding instructions**: Adding a new instruction requires only a new variant in `Instruction`; the `Program` wrapper and interpreter are unaffected (open-closed principle).
4. **Efficiency**: The operational monad has the same theoretical complexity as free monads; both build heap-allocated trees proportional to program size.

## Exercises

1. Add a `Sleep(duration_ms: u64)` instruction and implement both a real (actual sleep) and mock (instant) interpreter for it.
2. Implement instruction fusion: detect and merge consecutive `Print` operations into a single buffered write.
3. Write a `to_io_actions(program: Program<A>) -> Vec<IoAction>` function that serializes a program to a list of actions without executing them.
