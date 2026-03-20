# Struct of Arrays (SoA) Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  


> **Functional Rust** · [hightechmind.io](https://hightechmind.io)

## Problem Statement

Modern CPUs execute instructions faster than memory can supply data. The bottleneck in
data-intensive loops is not computation but cache misses: loading a 64-byte cache line
that contains only 4 bytes of useful data wastes 94% of bandwidth. The Array of Structs
(AoS) layout—the default in most languages—interleaves every field of every record,
forcing the CPU to load fields it does not need. Struct of Arrays (SoA) separates each
field into its own contiguous array, enabling full cache-line utilization when iterating
over a single field.

The pattern matters whenever a hot loop reads or writes only a subset of fields: physics
simulations reading positions to compute forces, renderers reading vertex positions to
cull geometry, databases scanning a single column, or game engines querying entity
health bars. In all these cases AoS pays a memory bandwidth tax proportional to the
number of unused fields.

## Learning Outcomes

- Understand how CPU cache lines and spatial locality affect measured throughput
- Implement both AoS and SoA layouts and benchmark the difference
- Use `#[repr(C)]` and field alignment to reason about memory layouts
- Apply the SoA pattern in iterators that remain safe and ergonomic
- Recognize when SoA hurts (random access, frequent full-struct reads)

## Rust Application

Rust makes SoA straightforward because ownership rules prevent dangling cross-array
references and the lack of hidden indirection keeps layouts predictable.

```rust
// AoS — interleaved, cache-unfriendly for single-field scans
struct ParticleAoS {
    x: f32, y: f32, z: f32,
    vx: f32, vy: f32, vz: f32,
    mass: f32,
}

// SoA — separated, cache-friendly for single-field scans
struct ParticleSoA {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
    vx: Vec<f32>,
    vy: Vec<f32>,
    vz: Vec<f32>,
    mass: Vec<f32>,
}

impl ParticleSoA {
    fn update_positions(&mut self, dt: f32) {
        // Tight loop over contiguous f32 slices — SIMD-friendly
        for i in 0..self.x.len() {
            self.x[i] += self.vx[i] * dt;
            self.y[i] += self.vy[i] * dt;
            self.z[i] += self.vz[i] * dt;
        }
    }
}
```

Key Rust advantages: `Vec<f32>` guarantees contiguous heap allocation; the borrow
checker prevents accidental aliasing between arrays; iterator combinators like `zip`
let you traverse multiple SoA columns safely.

## OCaml Approach

OCaml's GC uses a uniform boxed representation: every `float` inside a record is
heap-allocated with a header word unless the record contains *only* floats, in which
case OCaml applies a float-array optimization. For mixed-field structs, AoS is
unavoidable without manual `Bigarray` usage:

```ocaml
(* Unboxed float arrays — cache-friendly for float-only data *)
type particle_soa = {
  x    : float array;
  y    : float array;
  mass : float array;
}

let update_positions soa dt =
  Array.iteri (fun i _ ->
    soa.x.(i) <- soa.x.(i) +. soa.vx.(i) *. dt
  ) soa.x
```

OCaml 5 introduces unboxed types (`#float`, `#int`) that reduce boxing overhead but
full SoA still requires manual field separation.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Default float layout | Unboxed, inline | Boxed (GC header per value) |
| Float-only struct | Contiguous, SIMD-ready | Flat array optimization applies |
| Mixed struct | Interleaved, inline fields | Each non-float field boxed |
| SoA ergonomics | `Vec<f32>` per field, safe | `Bigarray` or manual arrays |
| Auto-vectorization | Common with contiguous slices | Rare without Bigarray |

Rust's value semantics mean `ParticleSoA` fields are guaranteed contiguous without
annotation. OCaml needs explicit `Bigarray.Array1` for numeric-intensive SoA work.

## Key Differences (Summary)

The SoA pattern is a memory layout optimization, not a language feature. Rust enables
it naturally via `Vec<T>` and slice iteration; OCaml requires more manual effort due to
the GC's uniform representation. The performance gap between AoS and SoA grows with
the number of unused fields per iteration and the total data size relative to L1/L2
cache.

## Exercises

1. Benchmark `ParticleAoS` vs `ParticleSoA` position update with 1 million particles
   using `criterion`. Measure with and without compiler auto-vectorization (`-C target-cpu=native`).
2. Implement a `ParticleSoAIter` that yields `(x, y, z, vx, vy, vz, mass)` tuples by
   zipping the seven slices, maintaining borrowing safety.
3. Extend `ParticleSoA` to support sorting particles by mass while keeping all arrays
   synchronized (use an index array or a sort-and-permute approach).
4. Implement an "AoSoA" (Array of Struct of Arrays) layout using `[f32; 8]` lanes per
   field group to combine SIMD width with cache locality.
5. Profile a real workload (ray-sphere intersection, N-body gravity) to identify which
   fields are hot and restructure accordingly.
