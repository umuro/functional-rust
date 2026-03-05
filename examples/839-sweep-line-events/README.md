📖 **[View on hightechmind.io →](https://hightechmind.io/rust/839-sweep-line-events)**

---

# 839: Sweep Line Algorithm with Event Queue

**Difficulty:** 4  **Level:** Advanced

Process geometric or temporal events sorted by position — the algorithmic skeleton behind interval union, rectangle area, and line intersection detection.

## The Problem This Solves

Many geometric and temporal problems can be solved by imagining a vertical line sweeping from left to right across the plane, processing events (interval starts, ends, intersections) as the line encounters them. Rather than examining all pairs of objects (O(n²)), the sweep line maintains an ordered state of "currently active" objects and updates it as events occur — often reducing complexity to O(n log n).

Sweep line algorithms solve: computing the total length/area of a union of intervals/rectangles, reporting all intersecting line segments (Bentley-Ottmann), computing Voronoi diagrams (Fortune's algorithm), and scheduling problems (earliest deadline first, interval colouring). It's the algorithmic paradigm behind many O(n log n) geometry algorithms that would naïvely be O(n²).

This example demonstrates sweep line on the interval union problem: given n intervals, compute the total length covered (counting overlaps only once). It also shows the skeleton for more general event processing.

## The Intuition

Sort all interval endpoints as events: START events at the left endpoint, END events at the right endpoint. Process events left to right, maintaining a counter of "active intervals." When the counter goes from 0→1 (entering covered territory), start a new covered segment. When it goes from 1→0 (leaving covered territory), close the segment.

For the general case, the "state" can be a balanced BST of active segments (ordered by y-coordinate at the current sweep x), with START inserting into the BST, END removing from it, and INTERSECTION events causing adjacent segments to swap order.

The event-driven approach is O(n log n): sort events O(n log n), process each event O(log n) with a priority queue and active set.

## How It Works in Rust

```rust
#[derive(Debug, Clone, PartialEq)]
enum EventType { Start, End }

#[derive(Debug, Clone)]
struct Event {
    x: f64,
    kind: EventType,
    id: usize,
}

fn sweep_line_union(intervals: &[(f64, f64)]) -> f64 {
    if intervals.is_empty() { return 0.0; }

    // Create START and END events for each interval
    let mut events: Vec<Event> = intervals.iter().enumerate()
        .flat_map(|(id, &(lo, hi))| {
            [
                Event { x: lo, kind: EventType::Start, id },
                Event { x: hi, kind: EventType::End,   id },
            ]
        })
        .collect();

    // Sort: by x, with END before START at the same x (avoids counting 0-gap)
    events.sort_by(|a, b| {
        a.x.partial_cmp(&b.x).unwrap()
            .then_with(|| {
                // At same x: process End before Start to avoid double-counting
                match (&a.kind, &b.kind) {
                    (EventType::End, EventType::Start) => std::cmp::Ordering::Less,
                    (EventType::Start, EventType::End) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                }
            })
    });

    let mut total = 0.0;
    let mut active = 0usize;  // count of active (overlapping) intervals
    let mut cover_start = 0.0; // x where current covered segment started

    for event in &events {
        match event.kind {
            EventType::Start => {
                if active == 0 {
                    cover_start = event.x; // entering covered territory
                }
                active += 1;
            }
            EventType::End => {
                active -= 1;
                if active == 0 {
                    total += event.x - cover_start; // leaving covered territory
                }
            }
        }
    }
    total
}

// Generic event processor skeleton
fn process_events<S, F>(events: &mut Vec<Event>, initial_state: S, handler: F) -> S
where
    F: Fn(S, &Event) -> S,
{
    events.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    events.iter().fold(initial_state, handler)
}
```

The tie-breaking rule (END before START at equal x) is critical: two intervals [1, 3] and [3, 5] share endpoint x=3. Without the tie-break, you'd count a gap at 3 that doesn't exist.

`then_with` on `Ordering` chains comparisons cleanly — compare by x first, then by event type to break ties. This is the idiomatic Rust multi-key sort pattern.

The `process_events` skeleton shows how sweep line generalises: `fold` over sorted events with an evolving state — a functional pattern that maps cleanly from OCaml's `List.fold_left`.

## What This Unlocks

- **Rectangle area union**: extend to 2D by running sweep line on x, with a segment tree tracking covered y-length at the current x. O(n log n) for n rectangles.
- **Bentley-Ottmann line sweep**: detect all intersections among n line segments in O((n + k) log n) using a balanced BST of active segments ordered by y.
- **Scheduling / interval colouring**: find the minimum number of machines to run all jobs — equals the maximum overlap depth at any point, which sweep line computes in O(n log n).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Event type | `type kind = Start \| End` | `enum EventType { Start, End }` |
| Multi-key sort | `List.sort (fun a b -> ...)` with nested compare | `sort_by` with `.then_with()` chaining |
| Active counter | Mutable `ref int` | `let mut active = 0usize` — direct |
| Fold over events | `List.fold_left handler state events` | `events.iter().fold(initial_state, handler)` |
| Float comparison | `compare` (total order, handles NaN poorly) | `partial_cmp(...).unwrap()` — explicit NaN panic |
