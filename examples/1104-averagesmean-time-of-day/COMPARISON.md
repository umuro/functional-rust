# OCaml vs Rust: Averages/Mean time of day

## Side-by-Side Code

### OCaml
```ocaml
let pi_twice = 2.0 *. Float.pi
let day = float (24 * 60 * 60)

let rad_of_time t = t *. pi_twice /. day
let time_of_rad r = r *. day /. pi_twice

let mean_angle angles =
  let sum_sin = List.fold_left (fun s a -> s +. sin a) 0.0 angles
  and sum_cos = List.fold_left (fun s a -> s +. cos a) 0.0 angles in
  atan2 sum_sin sum_cos

let mean_time times =
  let angles = List.map rad_of_time times in
  let t = time_of_rad (mean_angle angles) in
  if t < 0.0 then t +. day else t
```

### Rust (idiomatic)
```rust
use std::f64::consts::PI;
const DAY: f64 = 86_400.0;

pub fn mean_angle(angles: &[f64]) -> f64 {
    let sum_sin: f64 = angles.iter().map(|a| a.sin()).sum();
    let sum_cos: f64 = angles.iter().map(|a| a.cos()).sum();
    sum_sin.atan2(sum_cos)
}

pub fn mean_time(times: &[f64]) -> f64 {
    let angles: Vec<f64> = times.iter().map(|&t| rad_of_time(t)).collect();
    let t = time_of_rad(mean_angle(&angles));
    if t < 0.0 { t + DAY } else { t }
}
```

### Rust (functional pipeline — one-shot entry point)
```rust
pub fn mean_time_of_strings(times: &[&str]) -> Option<String> {
    let parsed: Vec<f64> = times
        .iter()
        .map(|t| parse_time(t))
        .collect::<Option<Vec<_>>>()?;
    Some(format_time(mean_time(&parsed)))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Convert time to angle | `val rad_of_time : float -> float` | `fn rad_of_time(t: f64) -> f64` |
| Circular mean of angles | `val mean_angle : float list -> float` | `fn mean_angle(angles: &[f64]) -> f64` |
| Mean of times | `val mean_time : float list -> float` | `fn mean_time(times: &[f64]) -> f64` |
| Parse time string | `val parse_time : string -> float` (may raise) | `fn parse_time(s: &str) -> Option<f64>` |
| Format seconds | `val string_of_time : float -> string` | `fn format_time(t: f64) -> String` |

## Key Insights

1. **`atan2` call convention:** OCaml uses `atan2 y x` (free function, positional args); Rust
   uses `y.atan2(x)` (method on the `y` receiver). Same mathematics — in Rust the numerator
   is the receiver, which reads naturally in a data-flow pipeline.

2. **Fold vs `.sum()`:** OCaml accumulates with `List.fold_left (fun s a -> s +. sin a) 0.0`;
   Rust composes `.map(|a| a.sin()).sum()` — more declarative because `sum` captures the intent
   without an explicit accumulator variable.

3. **Error handling — exception vs `Option`:** OCaml's `Scanf.sscanf` raises on malformed
   input; Rust returns `Option<f64>` and the `?` operator propagates `None` early, making the
   failure path type-safe and composable without exceptions.

4. **Slice vs linked list:** OCaml operates on linked lists (`float list`); Rust uses borrowed
   slices (`&[f64]`), which are contiguous in memory and avoid pointer chasing. The algorithm
   is identical; the data layout differs substantially.

5. **Circular averaging — the core idea:** A naive arithmetic mean of times near midnight fails
   (mean of 23:00 and 01:00 gives 12:00 instead of 00:00). Mapping each time to an angle on
   the unit circle, computing the vector resultant with `atan2(sum_sin, sum_cos)`, and
   converting back gives the correct answer in both OCaml and Rust.

## When to Use Each Style

**Use idiomatic Rust (`.map().sum()`)** for numeric reductions over slices — zero-allocation
for the sums, expressive, and clippy-clean.

**Use the pipeline entry point (`mean_time_of_strings`)** when input comes as raw strings
from user input or configuration — `Option` propagation via `?` gives clean error handling
at the API boundary without any `.unwrap()` calls inside the implementation.
