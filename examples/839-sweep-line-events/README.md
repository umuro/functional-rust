📖 **[View on hightechmind.io →](https://hightechmind.io/rust/839-sweep-line-events)**

---

# Sweep Line Algorithm
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Many geometric problems — computing area of union of rectangles, finding all intersecting segment pairs, computing Voronoi diagrams — can be solved efficiently by imagining a vertical line sweeping left-to-right through the plane. The sweep line processes "events" (start/end of intervals, segment crossings) in sorted x-order, maintaining a data structure of active elements. This reduces 2D problems to a sequence of 1D operations. The sweep line technique is the basis for computational geometry algorithms used in GIS, VLSI design, and computer graphics.

## Learning Outcomes

- Model the sweep as a priority queue of events sorted by x-coordinate
- Maintain an active set of elements currently crossed by the sweep line
- Process start events (add to active set) and end events (remove from active set)
- Apply to: area of union of rectangles, all-segments intersection detection, closest pair
- Understand event-driven programming: only process significant x-positions, not every pixel

## Rust Application

```rust
#[derive(Debug)]
pub enum EventType { Start, End }
pub struct Event {
    pub x: f64,
    pub event_type: EventType,
    pub interval: (f64, f64),  // y-range of the element
}
pub fn sweep_union_area(rectangles: &[(f64, f64, f64, f64)]) -> f64 {
    // Generate start/end events, sort by x
    // Sweep: maintain active y-intervals, compute covered y-length at each x-step
}
```

Rust's enum `EventType` with `Start`/`End` variants cleanly models event types. The event list is a `Vec<Event>` sorted by x using `sort_by`. The active set during sweep is a data structure appropriate for the problem — a sorted set for segment intersection, a list for rectangle union. Rust's `BTreeSet` provides O(log n) insert/remove for the active set. The area computation multiplies the covered y-length (computed at each event) by the delta-x to the next event.

## OCaml Approach

OCaml uses a variant type for events and `List.sort` for the event queue. The active set is a `Set.Make` balanced BST. OCaml's `match event_type with Start -> ... | End -> ...` cleanly dispatches. The area accumulation uses a mutable `float ref` or a fold over events. OCaml's `float_of_int` handles mixed int/float coordinates. The sweep function is naturally recursive over the sorted event list.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Event type | `enum EventType` | Variant type |
| Event sort | `sort_by` on `Vec<Event>` | `List.sort` |
| Active set | `BTreeSet` for O(log n) ops | `Set.Make` balanced BST |
| Area accumulation | `f64` variable in loop | Mutable ref or fold |
| Dispatch | `match event.event_type` | `match event_type with` |
| Coordinate type | `f64` or `i64` | `float` or `int` |

## Exercises

1. Implement the area of union of n rectangles using the sweep line in O(n^2) (naive active set) then O(n log^2 n) (segment tree for y-coverage).
2. Find all intersecting pairs among n line segments using the Bentley-Ottmann sweep line algorithm.
3. Implement a sweep line to compute the perimeter of the union of rectangles.
4. Count the maximum number of overlapping intervals at any point using a sweep line over 1D intervals.
5. Extend the rectangle union to 3D: volume of union of axis-aligned boxes using two nested sweeps.
