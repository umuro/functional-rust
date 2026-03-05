# OCaml vs Rust: Soundness, Undefined Behaviour, and Safety Invariants

## Side-by-Side Code

### OCaml — SortedList via module abstraction

```ocaml
(* OCaml enforces invariants through the module system.
   The concrete type is hidden; only the signature is visible. *)
module SortedList : sig
  type t
  val empty  : t
  val insert : int -> t -> t
  val to_list : t -> int list
end = struct
  type t = int list   (* invariant: always sorted ascending *)

  let empty = []

  let rec insert x = function
    | []               -> [x]
    | h :: tl when x <= h -> x :: h :: tl
    | h :: tl          -> h :: insert x tl

  let to_list xs = xs
end
```

### Rust (idiomatic) — SortedVec via private field

```rust
pub struct SortedVec<T: Ord>(Vec<T>);  // private inner Vec

impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self { Self(Vec::new()) }

    pub fn insert(&mut self, val: T) {
        let pos = self.0.partition_point(|x| x <= &val);
        self.0.insert(pos, val);
    }

    pub fn as_slice(&self) -> &[T] { &self.0 }
    pub fn contains(&self, val: &T) -> bool { self.0.binary_search(val).is_ok() }
}
```

### Rust (unsafe with SAFETY comment) — non-overlapping mutable splits

```rust
pub fn split_at_mut_demo<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    assert!(mid <= slice.len());
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    // SAFETY:
    //   • ptr is valid for len elements (live &mut [T]).
    //   • mid <= len is checked above, so both ranges are in-bounds.
    //   • [ptr, ptr+mid) and [ptr+mid, ptr+len) are disjoint — no aliasing.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Sorted collection | `SortedList.t` (abstract module type) | `SortedVec<T: Ord>` (private field) |
| Non-empty guarantee | OCaml has no built-in; use `hd` (raises) | `NonEmpty<T>` (type-level proof) |
| Optional result | `'a option` | `Option<T>` |
| Unsafe operation | N/A — OCaml has no `unsafe` | `unsafe fn` / `unsafe {}` + SAFETY comment |
| Checked wrapper | Signature hiding provides safety | `pub fn checked_index` wraps `unsafe fn raw_index` |

## Key Insights

1. **Invariants are maintained by access control.** OCaml uses opaque module types;
   Rust uses private fields. In both languages, callers can only reach data through
   the methods you expose, so the invariant is safe if every method preserves it.

2. **`unsafe` transfers the proof obligation.** When you write `unsafe { ... }` in
   Rust you assert "I have verified the preconditions the compiler cannot check."
   The `// SAFETY:` comment documents *what* you verified and *why* it holds — it
   is the human-readable proof accompanying the machine's trust.

3. **Soundness is a global property, not a local one.** Code that is correct in
   isolation can still be *unsound*: a private `unsafe impl Send` on a non-Send
   type allows safe callers to move it across threads, producing data races from
   entirely safe code. You must reason about all possible calling sequences.

4. **The borrow checker cannot prove disjointness across a single pointer.** The
   canonical example is `split_at_mut`: Rust's type system would reject two
   `&mut` references derived from the same slice, even if they are non-overlapping.
   `unsafe` lets us express the invariant the compiler cannot — with a SAFETY
   comment as the contract.

5. **OCaml's GC eliminates whole classes of UB.** Use-after-free, dangling
   pointers, and out-of-bounds memory access are impossible in OCaml because the
   runtime tracks object lifetimes. Rust achieves the same safety statically at
   compile time through the borrow checker — and lets you opt out with `unsafe`
   when the pattern is provably correct but inexpressible in the type system.

## When to Use Each Style

**Use encapsulation (private field / opaque module)** when the invariant can be
maintained purely through controlled construction and mutation — no pointer
arithmetic or aliasing involved.

**Use `unsafe` with a SAFETY comment** when you need to express a property the
compiler cannot verify: custom aliasing rules, pointer arithmetic, raw FFI,
custom memory layouts, or performance-critical algorithms that require temporary
invariant violation.

**Never expose raw pointers in a public API** without a checked, safe wrapper.
The unsafe code's caller should be your own module, not downstream users.
