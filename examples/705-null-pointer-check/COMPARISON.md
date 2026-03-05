# OCaml vs Rust: Null Pointer Handling with NonNull<T>

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml has no nulls — every value is non-null by construction.
   Absence is modelled with option, not null pointers. *)
let make_nonnull (x : 'a) : 'a = x          (* identity: all values are non-null *)
let wrap_nullable (x : 'a option) : 'a option = x

let () =
  let nn = make_nonnull 42 in
  Printf.printf "NonNull value: %d\n" nn;
  Printf.printf "Some: %s\n"
    (match wrap_nullable (Some 99) with Some v -> string_of_int v | None -> "null");
  Printf.printf "None: %s\n"
    (match wrap_nullable None with Some v -> string_of_int v | None -> "null");
  assert (make_nonnull 0 = 0);
  print_endline "ok"
```

### Rust (idiomatic — Option<NonNull<T>> at pointer size)
```rust
use std::ptr::NonNull;
use std::mem::size_of;

// NonNull::new turns a raw pointer into Option<NonNull<T>>,
// forcing null-check at the boundary rather than at dereference.
pub fn wrap_nullable<T>(ptr: *mut T) -> Option<NonNull<T>> {
    NonNull::new(ptr)   // None if null, Some(nn) if non-null
}

// Option<NonNull<T>> is exactly pointer-sized (null-pointer optimisation).
pub fn option_nonnull_is_pointer_sized<T>() -> bool {
    size_of::<Option<NonNull<T>>>() == size_of::<*mut T>()
}
```

### Rust (functional — NonNull-based linked list)
```rust
pub fn build_list<T: Copy>(values: &[T]) -> Option<NonNull<Node<T>>> {
    let mut head = None;
    for &v in values.iter().rev() {
        let node = Box::new(Node { value: v, next: head });
        // SAFETY: Box::into_raw is never null.
        head = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
    }
    head
}

pub fn collect_list<T: Copy>(mut cursor: Option<NonNull<Node<T>>>) -> Vec<T> {
    let mut out = Vec::new();
    while let Some(ptr) = cursor {
        // SAFETY: every pointer came from Box::into_raw and is still live.
        let node = unsafe { ptr.as_ref() };
        out.push(node.value);
        cursor = node.next;
    }
    out
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Non-null value | every `'a` value (no nulls exist) | `NonNull<T>` — `*mut T` guaranteed non-null |
| Nullable pointer | `'a option` | `Option<NonNull<T>>` |
| Null check | pattern match on `option` | `NonNull::new` returns `Option<NonNull<T>>` |
| Memory size of nullable ptr | always pointer-size (boxed rep) | `size_of::<Option<NonNull<T>>>() == size_of::<*mut T>()` |
| Dereference | direct (GC-managed) | `unsafe { nn.as_ref() }` — explicit unsafe block |

## Key Insights

1. **No nulls in OCaml**: every OCaml value is non-null by definition; the language achieves the same safety guarantee that `NonNull<T>` adds back to Rust's unsafe pointer world.
2. **Null-pointer optimisation**: `Option<NonNull<T>>` is encoded as a single machine word — `None` maps to the null address, `Some(nn)` to the pointer itself — exactly matching a C nullable pointer in size and ABI.
3. **Safety boundary at construction**: `NonNull::new` is the only safe entry point; it returns `Option<NonNull<T>>` so null-checking is forced at the FFI or allocation boundary rather than silently deferred to dereference time.
4. **Manual memory discipline**: OCaml's GC reclaims nodes automatically; the Rust version requires an explicit `free_list` walk using `Box::from_raw` to avoid leaking heap nodes — the borrow checker doesn't help inside `unsafe`.
5. **`unsafe` is scoped and documented**: each `unsafe` block carries a `// SAFETY:` comment explaining why the invariant holds, making audits tractable even when the code is low-level.

## When to Use Each Style

**Use `NonNull<T>` when:** writing custom allocators, intrusive data structures, or wrapping C FFI that returns nullable pointers — anywhere you need raw-pointer performance with a compiler-enforced non-null invariant and zero-cost `Option` encoding.

**Use `Option<Box<T>>` or `Option<&T>` when:** you don't need raw pointers at all; prefer safe references and let the borrow checker handle lifetime and aliasing instead of maintaining manual `unsafe` invariants.
