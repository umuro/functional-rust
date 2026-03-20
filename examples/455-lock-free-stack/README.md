📖 **[View on hightechmind.io →](https://hightechmind.io/rust/455-lock-free-stack)**

---

# 455: Lock-Free Stack
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A mutex-based stack serializes all push/pop operations — a contention bottleneck under high concurrency. A lock-free stack uses `compare_and_swap` on the head pointer: push allocates a new node, sets its next to the current head, then CAS-swaps the head; pop reads the head, follows next, then CAS-swaps the head to next. Retrying on CAS failure makes it correct without locks. Treiber's lock-free stack (1986) is the canonical algorithm and remains relevant in high-frequency trading, lock-free allocators, and concurrent data structures research.

Lock-free stacks appear in memory allocators (free list), lock-free work queues, concurrent GC systems, and any high-throughput concurrent LIFO structure.

## Learning Outcomes

- Understand Treiber's lock-free stack algorithm using CAS on the head pointer
- Learn how `AtomicPtr<Node<T>>` stores the stack head with atomic pointer swaps
- See how the push CAS loop handles concurrent modifications correctly
- Understand the ABA problem: node reuse can fool CAS even with correct values
- Learn why unsafe Rust is needed for raw pointer manipulation in lock-free structures

## Rust Application

The implementation uses `AtomicPtr<Node<T>>` as the head pointer. `push` allocates a `Box::new(Node { val, next: null })`, sets `next` to the current head via CAS loop, and on success the node is in the stack. `pop` reads the head, follows `next`, and CAS-swaps head to `next`. Both operations retry on CAS failure. The implementation requires `unsafe` for raw pointer dereferencing.

## OCaml Approach

OCaml's lock-free stack for OCaml 5.x uses `Atomic.t` for the head pointer combined with OCaml's GC handling memory. The GC eliminates ABA problems for pointer-based structures since allocated nodes are never reused at the same address while still referenced. A lock-free stack: `let push s v = let n = { v; next = Atomic.get s } in while not (Atomic.compare_and_set s n.next n) do n.next <- Atomic.get s done`.

## Key Differences

1. **Memory safety**: Rust requires `unsafe` for pointer manipulation; OCaml's GC provides automatic memory management for lock-free structures.
2. **ABA problem**: Rust's lock-free stack is susceptible to ABA (need hazard pointers or epoch GC); OCaml's GC prevents ABA for pointer-based structures.
3. **Type safety**: Rust's `AtomicPtr<T>` is typed; OCaml's `Atomic.t` holds any value.
4. **Unsafe boundary**: Rust isolates unsafe code; OCaml's lock-free code is safe but depends on GC memory model guarantees.

## Exercises

1. **Correct stack test**: Write tests for the lock-free stack with 8 concurrent producer threads pushing 1000 items each, and 8 consumer threads popping until empty. Verify no items are lost or duplicated.
2. **ABA demonstration**: Devise a scenario where the ABA problem could manifest in the current implementation. Propose a fix using an epoch counter packed with the pointer (if on a 64-bit platform with spare pointer bits).
3. **Lock-free queue**: Extend the lock-free stack concept to implement a lock-free FIFO queue (Michael-Scott queue). The key difference: maintain both head and tail pointers, each protected by separate CAS operations.
