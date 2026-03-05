# OCaml vs Rust: Struct of Arrays vs Array of Structs

## Side-by-Side Code

### OCaml (AoS — sum x-coordinates)
```ocaml
type particle = { x: float; y: float; z: float; mass: float }

let sum_x_aos particles =
  Array.fold_left (fun acc p -> acc +. p.x) 0.0 particles
```

### OCaml (SoA — sum x-coordinates)
```ocaml
type particles_soa = { xs: float array; ys: float array; zs: float array; masses: float array }

let sum_x_soa soa =
  Array.fold_left (+.) 0.0 soa.xs
```

### Rust (AoS — idiomatic)
```rust
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParticleAoS { pub x: f32, pub y: f32, pub z: f32, pub mass: f32 }

pub fn sum_x_aos(particles: &[ParticleAoS]) -> f32 {
    particles.iter().map(|p| p.x).sum()
}
```

### Rust (SoA — idiomatic)
```rust
#[derive(Debug, Default)]
pub struct ParticlesSoA {
    pub xs: Vec<f32>, pub ys: Vec<f32>, pub zs: Vec<f32>, pub masses: Vec<f32>,
}

pub fn sum_x_soa(soa: &ParticlesSoA) -> f32 {
    soa.xs.iter().sum()
}
```

### Rust (gravity update — SoA with zip)
```rust
pub fn apply_gravity_soa(soa: &mut ParticlesSoA, dt: f32) {
    soa.ys
        .iter_mut()
        .zip(soa.masses.iter())
        .for_each(|(y, &m)| *y -= 9.81 * m * dt);
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| AoS element | `type particle = { x: float; ... }` | `struct ParticleAoS { x: f32, ... }` |
| SoA container | `type particles_soa = { xs: float array; ... }` | `struct ParticlesSoA { xs: Vec<f32>, ... }` |
| Sum function | `val sum_x_soa : particles_soa -> float` | `fn sum_x_soa(soa: &ParticlesSoA) -> f32` |
| Mutation | `Array.iteri` + index | `.iter_mut().zip(...)` |
| Borrowing | N/A (GC) | `&[T]` for read, `&mut [T]` for write |

## Memory Layout

```
AoS: [x0|y0|z0|m0|x1|y1|z1|m1|x2|y2|z2|m2|...]
      ^^^^                                       ← only this used when summing x
      cache line brings y0,z0,m0 for free — wasted

SoA: [x0|x1|x2|...xN] [y0|y1|...yN] [z0|...] [m0|...]
     ^^^^^^^^^^^^^^^^^                          ← entire array used when summing x
      cache line is 100% useful
```

## Key Insights

1. **Cache line utilisation**: AoS loads an entire struct (16 bytes here) per iteration but uses only 4 bytes (`x`); SoA accesses a tightly packed `f32` array — 16 values per 64-byte cache line vs. 4 in AoS. This is a 4× difference in effective cache bandwidth for single-field workloads.

2. **Iterator composition**: Rust's `.iter_mut().zip(other.iter())` lets the gravity update operate on two SoA columns simultaneously without allocating — the same pattern OCaml expresses with `Array.iteri`. Both compile to a simple sequential loop over contiguous memory.

3. **Ownership encoding**: Rust encodes the AoS→SoA boundary at the type level. A function taking `&[ParticleAoS]` signals AoS semantics; one taking `&ParticlesSoA` signals SoA semantics. OCaml uses structural types but relies on programmer discipline for the same distinction.

4. **Mutable aliasing safety**: `apply_gravity_soa` takes `&mut ParticlesSoA` and simultaneously borrows `ys` mutably and `masses` immutably via `zip`. Rust enforces at compile time that no aliasing occurs — OCaml's GC permits this but provides no static guarantee.

5. **Layout control**: Rust gives precise control over struct layout via `#[repr(C)]`, padding, and alignment. OCaml floats are boxed by default (except in float-only records), so AoS in OCaml already suffers an additional indirection overhead that Rust avoids entirely.

## When to Use Each Style

**Use AoS when:** entities are usually processed whole (e.g. serialising a single particle to JSON, passing one particle to a physics callback), or the collection is small enough to fit in L1 cache regardless of layout.

**Use SoA when:** hot loops process one or two fields across thousands of entities (physics simulations, particle systems, SIMD vectorisation). The layout change alone routinely yields 2–10× throughput improvements without algorithmic changes. SIMD intrinsics require SoA because they load a vector register from contiguous memory of a single type.
