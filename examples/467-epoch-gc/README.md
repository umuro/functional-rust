📖 **[View on hightechmind.io →](https://hightechmind.io/rust/467-epoch-gc)**

---

# Epoch-Based Garbage Collection
**Difficulty:** ⭐  
**Category:** Functional Programming  


A memory reclamation scheme for concurrent data structures that defers deallocation until no thread holds a reference to the object being freed.

## Problem Statement

Lock-free data structures remove nodes without a lock, but cannot immediately `free` the memory: another thread may still be traversing the node. Reference counting (like `Arc`) adds atomic overhead to every access. **Epoch-based reclamation (EBR)**, introduced by Fraser (2004) and popularised by the `crossbeam-epoch` crate, solves this by grouping time into epochs. A thread "pins" its current epoch before reading, then unpins after. Memory retired in epoch E is only freed when every thread has advanced past E, guaranteeing no live references remain.

## Learning Outcomes

- Understand why safe deallocation in lock-free code requires more than a simple `free`
- Model the epoch counter with `AtomicU64` and Acquire/Release ordering
- Track pinned epochs to compute the safe-to-free threshold
- Implement `retire` (mark for deferred free) and `collect` (advance epoch and reclaim)
- Distinguish EBR from hazard pointers and reference counting

## Rust Application

`EpochMgr` holds three fields:

```rust
struct EpochMgr {
    epoch:   AtomicU64,                      // global epoch
    retired: Mutex<VecDeque<(u64, String)>>, // deferred frees tagged by epoch
    pinned:  Mutex<Vec<u64>>,                // epochs currently held by live threads
}
```

`pin()` reads the current epoch and appends it to `pinned`; `unpin()` removes it. `retire(desc)` records the current epoch alongside the object description. `collect()` increments the global epoch, finds the minimum pinned epoch (`min_active`), and frees all retired objects from epochs earlier than `min_active - 1` — safely beyond any live reference.

## OCaml Approach

OCaml's garbage collector handles memory automatically, so EBR is not required in pure OCaml code. For C bindings or `Bigarray`-allocated memory, manual tracking is necessary. The `Domainslib` library for Multicore OCaml uses domain-local state to track safe points, analogous to pin/unpin:

```ocaml
(* Conceptual OCaml EBR sketch *)
let epoch = Atomic.make 0
let pinned = Domain.DLS.new_key (fun () -> ref (-1))

let pin () =
  let e = Atomic.get epoch in
  !(Domain.DLS.get pinned) <- e; e

let unpin () =
  !(Domain.DLS.get pinned) <- -1
```

## Key Differences

1. **Memory model**: Rust requires explicit `AtomicU64` with ordering annotations; OCaml's `Atomic` module provides sequentially consistent operations by default, hiding the ordering complexity.
2. **Manual vs. automatic GC**: Rust demands explicit EBR because the borrow checker does not track concurrent pointer lifetimes across thread boundaries; OCaml's GC handles ordinary heap objects automatically.
3. **Mutex necessity**: Rust uses `Mutex<VecDeque>` for the retire list because there is no GC-assisted write barrier; OCaml would use a domain-local list to avoid cross-domain locking.
4. **`unsafe` boundary**: Rust's actual lock-free structures require `unsafe` for raw pointer dereference; OCaml's type system prevents raw pointer arithmetic entirely.

## Exercises

1. **Thread-local pinning**: Replace `Mutex<Vec<u64>>` with `thread_local!` storage so `pin`/`unpin` do not contend on a shared mutex.
2. **Batch retirement**: Collect a thread-local retire list and only flush to the global queue on `unpin`, reducing mutex acquisitions.
3. **Integration**: Build a lock-free singly linked list that uses `EpochMgr` to retire removed nodes, verifying with `Miri` or `loom` that no use-after-free occurs.
