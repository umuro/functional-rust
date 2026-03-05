# OCaml vs Rust: `MaybeUninit` — Safe Uninitialized Memory

## Side-by-Side Code

### OCaml (deferred initialisation via option/variant)

```ocaml
type 'a maybe_uninit = Uninit | Init of 'a

let write _mu v = Init v

let assume_init = function
  | Init v -> v
  | Uninit -> failwith "assume_init called on uninitialised value!"

let () =
  let slot : int maybe_uninit ref = ref Uninit in
  slot := write !slot 42;
  let value = assume_init !slot in
  Printf.printf "Initialised value: %d\n" value
(* Output: Initialised value: 42 *)
```

### Rust — idiomatic `MaybeUninit<T>` (single value)

```rust
use std::mem::MaybeUninit;

fn fill_value(out: &mut MaybeUninit<u32>, x: u32) {
    out.write(x * 2);
}

fn single_value_demo() -> u32 {
    let mut slot = MaybeUninit::<u32>::uninit();
    fill_value(&mut slot, 21);
    // SAFETY: fill_value unconditionally calls .write()
    unsafe { slot.assume_init() }
}
```

### Rust — fixed-size array built element-by-element

```rust
use std::mem::MaybeUninit;

fn build_array<const N: usize>(f: impl Fn(usize) -> u32) -> [u32; N] {
    let mut arr: [MaybeUninit<u32>; N] =
        unsafe { MaybeUninit::uninit().assume_init() };

    for (i, slot) in arr.iter_mut().enumerate() {
        slot.write(f(i));
    }

    // SAFETY: every element written by loop above
    unsafe { std::mem::transmute_copy(&arr) }
}
```

### Rust — partial buffer with tracked initialisation

```rust
pub struct PartialBuf<T, const CAP: usize> {
    data: [MaybeUninit<T>; CAP],
    len: usize,
}

impl<T, const CAP: usize> PartialBuf<T, CAP> {
    pub fn push(&mut self, value: T) -> bool {
        if self.len >= CAP { return false; }
        self.data[self.len].write(value);
        self.len += 1;
        true
    }
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            &*(std::ptr::slice_from_raw_parts(
                self.data.as_ptr().cast::<T>(), self.len))
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Uninitialised slot | `'a maybe_uninit = Uninit \| Init of 'a` | `MaybeUninit<T>` |
| Write into slot | `write : 'a maybe_uninit -> 'a -> 'a maybe_uninit` | `MaybeUninit::write(&mut self, T) -> &mut T` |
| Assert initialised | `assume_init : 'a maybe_uninit -> 'a` (raises on Uninit) | `unsafe fn assume_init(self) -> T` (UB if uninit) |
| Fixed array uninit | `Array.make n (Obj.magic 0)` (unsafe) | `[MaybeUninit<T>; N]` (safe to create) |
| Partial buffer | `option array + length` | `PartialBuf<T, CAP>` struct |

## Key Insights

1. **OCaml always initialises heap values** — its GC requires valid pointers everywhere, so
   a real `MaybeUninit` concept doesn't exist. The closest idiom is `option`, which adds
   a tag-word overhead (`Some`/`None`). Rust's `MaybeUninit<T>` has *zero* overhead: it
   is exactly `sizeof(T)` bytes with no extra discriminant.

2. **The unsafe boundary is explicit** — in OCaml the runtime hides unsafety behind `Obj.magic`.
   In Rust, `MaybeUninit` keeps all allocation safe; only `.assume_init()` is `unsafe`,
   forcing the programmer to justify the invariant at exactly the right place.

3. **`MaybeUninit` enables the "C output parameter" pattern** — functions like `fill_value`
   that take `&mut MaybeUninit<T>` mirror C APIs (`int out;  foo(&out);`) without requiring
   `Default` or zeroing the buffer first.

4. **Array initialisation without `Default`** — `[T; N]` in Rust requires `T: Default` for
   safe construction. `[MaybeUninit<T>; N]` has no such constraint, making element-by-element
   initialisation possible for any `T` (useful for non-`Default` types like `File` or `TcpStream`).

5. **`Drop` must be explicit** — because Rust cannot know which `MaybeUninit` slots hold live
   values, the `PartialBuf::drop` implementation must manually call `assume_init_drop()` on
   each initialised slot. OCaml's GC handles this automatically via its always-valid invariant.

## When to Use Each Style

**Use `MaybeUninit` when:** you are writing FFI glue, a custom allocator, a fixed-capacity
buffer that avoids `Default`, or code that must avoid zero-initialising a large array before
overwriting every element.

**Use `Option<T>` (the OCaml style) when:** you are writing safe, high-level Rust and the
overhead of the `Some`/`None` tag is acceptable — it is simpler to reason about and requires
no `unsafe` at all.
