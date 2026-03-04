# 559: Region Inference Basics

**Difficulty:** 4  **Level:** Intermediate-Advanced

How Rust's borrow checker infers the minimal scope for each borrow — and why Non-Lexical Lifetimes (NLL) made programs that were always correct finally compile.

## The Problem This Solves

Early Rust tied borrows to lexical scopes: a borrow started at `let r = &x;` and ended at the closing `}`. This was simple to implement but too conservative — it rejected programs that were obviously safe, like using a reference and then mutating the original after the last use of the reference.

Non-Lexical Lifetimes (stabilized in Rust 2018) changed this. The borrow checker now infers the *minimal* region — the span from first use to last use — rather than the lexical block. This is "region inference": computing the smallest region that satisfies all the constraints, then checking for conflicts.

Understanding NLL explains why certain Rust code works that you might expect to fail, and helps you reason about where to restructure code when you hit borrow errors.

## The Intuition

Think of a region as a highlighted range in your source code. The borrow checker infers regions by asking: where is this reference first used? Where is it last used? The region is that span. Two regions conflict if they overlap *and* one is a mutable borrow.

Before NLL, regions were approximated by block boundaries (too large). With NLL, regions shrink to actual usage. A borrow can "end" mid-block the moment the reference is last used, even if the variable is still in scope.

## How It Works in Rust

**NLL in action — mutation after last reference use:**
```rust
let mut v = vec![1, 2, 3];
{
    let sum: i32 = v.iter().sum();  // borrow of v starts here
    println!("sum: {}", sum);       // borrow ends at last use of sum/v
}
v.push(4);  // safe: borrow ended at `println!`, before this line
```
Pre-NLL: this would fail (the block containing `sum` still open). NLL: borrow ends at `println!`.

**Region tied to a specific scope:**
```rust
let x = 5i32;
let r1;
{
    let y = 10i32;
    r1 = &x;       // region of r1: here to the final println!
    let r2 = &y;   // region of r2: here to the println! inside this block
    println!("r2: {}", r2);
    // r2's region ends here — y can be dropped
}
println!("r1: {}", r1);  // valid: r1 borrows x, which still lives
```

**Struct lifetimes — explicit but inferred at call site:**
```rust
struct View<'a> { data: &'a [i32] }

impl<'a> View<'a> {
    fn new(data: &'a [i32]) -> Self { View { data } }
    fn sum(&self) -> i32 { self.data.iter().sum() }
}

let data = vec![10, 20, 30];
let view = View::new(&data);  // 'a inferred = region of `data`
println!("{}", view.sum());
// view dropped here — 'a ends — data can be moved/dropped
```

**Two independent borrows — non-overlapping regions:**
```rust
let a = vec![1, 2, 3];
let b = vec![4, 5, 6];
let sum_a: i32 = a.iter().sum();  // borrow of a: just this line
let sum_b: i32 = b.iter().sum();  // borrow of b: just this line
// Both borrows ended immediately (sum_a, sum_b are i32, not refs)
println!("{} {}", sum_a, sum_b);
```
The regions of the two borrows don't overlap with any mutable use, so this compiles without explicit annotation.

## What This Unlocks

- **Confident borrow error diagnosis** — "this borrow lasts until X" in error messages is the region; now you know what X means.
- **Restructuring to satisfy the borrow checker** — NLL means the fix is often to move the last use of a reference earlier, not to restructure ownership.
- **Understanding lifetime annotations** — explicit `'a` tells the compiler the *minimum* region for a borrow; the checker infers the actual region within that constraint.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Reference validity tracking | GC (runtime) | Regions inferred at compile time |
| Borrow scope | N/A | From first use to last use (NLL) |
| Lexical vs non-lexical | N/A | NLL: borrow ends at last use, not at `}` |
| Struct lifetime | GC handles | `'a` on struct field ties lifetime to call site |
