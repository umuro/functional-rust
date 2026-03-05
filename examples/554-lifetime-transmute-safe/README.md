📖 **[View on hightechmind.io →](https://hightechmind.io/rust/554-lifetime-transmute-safe)**

---

# 554: Safe Transmute with Lifetimes

**Difficulty:** 5  **Level:** Advanced

When and how to reason about `std::mem::transmute` on lifetime parameters — and why the safe alternatives almost always suffice.

## The Problem This Solves

Rust's lifetime system prevents dangling references at compile time. But occasionally — in allocators, arena-based data structures, FFI wrappers, and self-referential patterns — you have a reference whose validity *you* can prove but the compiler cannot. The instinct is to reach for `transmute` to change the lifetime annotation and silence the borrow checker.

This is extremely dangerous. A transmuted lifetime tells the compiler "trust me, this reference lives long enough." If you are wrong, the result is a use-after-free — undefined behaviour that the borrow checker can no longer catch. Understanding *why* it is dangerous, *when* it might be justified, and what safe alternatives exist is essential for writing low-level Rust.

The key insight is that in almost every real-world case, there is a safe alternative: `from_raw_parts` with documented SAFETY invariants, `Pin` for self-referential structs, `Arc` for shared ownership, or simply restructuring ownership to let the borrow checker see the relationship.

## The Intuition

`transmute<T, U>(x: T) -> U` reinterprets the bits of `x` as type `U` with zero machine code. When `T = &'short Foo` and `U = &'long Foo`, the bits are identical (it's just a pointer), but the lifetime annotation changes. The pointer itself is not affected — only the compiler's bookkeeping is.

The danger: the compiler now assumes the reference is valid for `'long`. If the underlying data is dropped before `'long` ends, you get a dangling pointer. The borrow checker trusted you and stopped watching.

Safe patterns express the same relationship through proper API: `slice::from_raw_parts` (SAFETY comment required), `from_raw_parts_mut`, or by returning `&self.field` (where the compiler infers the lifetime from `self`).

## How It Works in Rust

**The dangerous pattern — do not use without proven invariants:**
```rust
unsafe fn extend_lifetime_unsafe<'short, 'long, T>(r: &'short T) -> &'long T {
    std::mem::transmute(r)
}
```

**Safe alternative — split_at with documented SAFETY:**
```rust
fn safe_split(data: &[i32], mid: usize) -> (&[i32], &[i32]) {
    assert!(mid <= data.len());
    unsafe {
        let ptr = data.as_ptr();
        // SAFETY: ptr..ptr+mid and ptr+mid..ptr+len are non-overlapping
        // slices within the same allocation, valid for lifetime of `data`
        let left  = std::slice::from_raw_parts(ptr, mid);
        let right = std::slice::from_raw_parts(ptr.add(mid), data.len() - mid);
        (left, right)
    }
}
```
This is exactly what `slice::split_at` does internally. The difference: the SAFETY comment documents the invariant you proved.

**Numeric type punning — use the safe API:**
```rust
let bits: u32 = 1.5f32.to_bits();  // safe, explicit
let back: f32 = f32::from_bits(bits);
```
`to_bits`/`from_bits` are the safe equivalents of `transmute::<f32, u32>`. Prefer them always.

**Struct owning its data — no transmute needed:**
```rust
struct OwnedData { data: Vec<u8> }
impl OwnedData {
    fn as_slice(&self) -> &[u8] { &self.data }  // lifetime tied to &self automatically
}
```
The borrow checker infers `fn as_slice<'a>(&'a self) -> &'a [u8]` — no unsafe required.

## What This Unlocks

- **Custom allocators and arenas** — understanding when `transmute` of lifetimes is genuinely necessary (rarely) and how to document invariants.
- **FFI boundary work** — converting raw pointers from C into safe Rust references with the correct lifetime.
- **Recognizing safe alternatives** — reach for `from_raw_parts`, `Pin`, `Arc`, or restructured ownership before writing `transmute`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Memory safety | GC prevents dangling refs | Borrow checker + lifetime annotations |
| Type punning | `Obj.magic` (escape hatch, no types) | `transmute` (escape hatch, types must have same layout) |
| Safe split | `Array.sub` (copies) | `slice::split_at` (zero-copy, safe) |
| Lifetime extension | N/A (GC tracks roots) | `transmute` or `from_raw_parts` + SAFETY doc |
