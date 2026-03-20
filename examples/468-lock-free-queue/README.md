📖 **[View on hightechmind.io →](https://hightechmind.io/rust/468-lock-free-queue)**

---

# Lock-Free Queue
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


A concurrent FIFO queue that uses atomic compare-and-swap operations instead of locks, implementing the Michael-Scott algorithm.

## Problem Statement

A mutex-protected queue serialises every enqueue and dequeue. In high-throughput systems — network packet processing, work-stealing schedulers, disruptor-pattern event buses — the lock becomes the bottleneck. Michael and Scott (1996) published a non-blocking queue algorithm using only CAS operations on head and tail pointers. It is the basis for Java's `ConcurrentLinkedQueue`, the `crossbeam-queue` crate, and the LMAX Disruptor.

## Learning Outcomes

- Understand the Michael-Scott two-pointer queue invariant (sentinel node, tail may lag)
- Use `AtomicPtr` with Acquire/Release ordering for safe cross-thread pointer sharing
- Implement CAS-based enqueue with tail advancement
- Handle the dequeue loop: empty check, lagging tail fix, head advance
- Write safe `Drop` that drains remaining nodes without leaking memory

## Rust Application

The queue maintains a sentinel (dummy) node so head and tail always point to valid heap memory:

```rust
pub struct Queue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}
```

Enqueue allocates a new node via `Box::into_raw`, then CAS-links it onto the tail's `next` pointer. A second CAS advances `tail`; if it fails, another thread already helped. Dequeue reads `head.next`, attempts to CAS `head` forward, and extracts the value with `ptr::read` before dropping the old head node via `Box::from_raw`. The `unsafe impl Send/Sync` blocks are safe because the algorithm's CAS protocol establishes the required happens-before edges.

## OCaml Approach

Multicore OCaml's `Saturn` library provides `Saturn.Queue` (Michael-Scott) and `Saturn.Single_prod_single_cons_queue`. In pure OCaml the idiom is:

```ocaml
(* Using Saturn *)
let q = Saturn.Queue.create ()
let () = Saturn.Queue.push q 42
let v  = Saturn.Queue.pop_opt q  (* None | Some x *)
```

Without Saturn, functional OCaml uses immutable persistent queues (two-list Okasaki queue) which are naturally thread-safe for reads but require atomic references for multi-producer use.

## Key Differences

1. **Explicit `unsafe`**: Rust requires `unsafe` blocks around raw pointer dereferences and `Box::from_raw`; OCaml's GC tracks all pointers and prohibits raw arithmetic entirely.
2. **Memory reclamation**: The example uses immediate `Box::from_raw(h)` to free dequeued nodes; in production, epoch-based reclamation (example 467) is needed to prevent use-after-free under concurrent dequeue.
3. **ABA risk**: Rust's simplified CAS on a raw pointer is susceptible to the ABA problem; OCaml's GC guarantees unique addresses for live objects, mitigating ABA for GC-managed nodes.
4. **`Send`/`Sync` proofs**: Rust demands explicit `unsafe impl Send/Sync`; OCaml's type system does not distinguish shared vs. owned data at the type level.

## Exercises

1. **Epoch integration**: Replace immediate node deallocation with `EpochMgr::retire` from example 467 to eliminate the use-after-free hazard.
2. **Bounded queue**: Add an `AtomicUsize` length counter; block `enqueue` when capacity is reached using a `Condvar`, turning the queue into a bounded channel.
3. **Benchmark**: Compare throughput of this lock-free queue against `Mutex<VecDeque>` under 4 producer / 4 consumer threads using `criterion`.
