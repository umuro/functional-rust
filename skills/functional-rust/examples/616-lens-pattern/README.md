# 616: Lens Pattern — Composable Getters and Setters

**Difficulty:** 5  **Level:** Master

A Lens is a first-class, composable getter+setter for a field in a nested struct. Define it once per field; compose them to reach any depth; use `over` to transform without boilerplate.

## The Problem This Solves

You have a three-level nested struct: `Event` → `Location` → `Coords`. You need to update `lat`. Here's what the code looks like without Lenses:

```rust
let updated = Event {
    location: Location {
        coords: Coords {
            lat: e.location.coords.lat + 1.0,
            lon: e.location.coords.lon,  // unchanged, but must be named
        },
        ..e.location.clone()  // city, name — unchanged, but must be spread
    },
    ..e.clone()  // title, attendees — unchanged, but must be spread
};
```

That's nine lines to add 1.0 to `lat`. Every field that isn't changing still has to be mentioned — either explicitly or via `..spread`. The signal (add 1.0 to lat) is lost in the noise (rebuilding every level of the struct).

Now multiply this by every field you ever update, every nesting level, every update function in your codebase. You end up with a codebase where struct update expressions are a major source of verbosity and a common place to introduce bugs (forgetting to spread a field, spreading the wrong intermediate struct, etc.).

Lenses abstract over this pattern. You write the struct update expression **once** — inside the Lens definition — and every call site that needs to update that field calls the Lens instead. This example exists to solve exactly that pain.

## The Intuition

A Lens for field `field` of struct `S` is the answer to two questions:

1. "How do I read `field` from an `S`?" → the `get` function
2. "How do I produce a new `S` where `field` is replaced?" → the `set` function

Once you have those two functions, everything else follows:
- **`get`**: just call it
- **`set`**: just call it
- **`over`**: call `get`, run the function, call `set`
- **Composition**: the `get` of `A→B` composed with the `get` of `B→C` gives the `get` of `A→C`; same for `set`

The key: these two functions, once bundled, can be **passed as a value**, **stored**, and **composed with other Lenses**. They're not tied to a specific call site. A Lens from `Event→Location` can be composed with any Lens that starts from `Location` — you don't have to know in advance what the second step will be.

```rust
// Two simple Lenses
let event_to_location: Lens<Event, Location>  = …;
let location_to_coords: Lens<Location, Coords> = …;
let coords_to_lat: Lens<Coords, f64>           = …;

// Composed — reaches lat directly from Event
let event_lat = event_to_location
    .compose(location_to_coords)
    .compose(coords_to_lat);
```

## How It Works in Rust

This example shows two approaches side by side:

**Approach 1 — Type-parameterized Lens (zero-cost, no allocation):**

```rust
struct Lens<S, A, F: Fn(&S) -> A, G: Fn(A, S) -> S> {
    get_fn: F,
    set_fn: G,
    _phantom: PhantomData<(S, A)>,
}
```

The closure types `F` and `G` are baked into the type parameters — no `Box`, no heap allocation. The downside: composition produces new types, and storing heterogeneous Lenses requires trait objects.

**Approach 2 — SimpleLens with `Box<dyn Fn>` (ergonomic, composable):**

```rust
struct SimpleLens<S, A> {
    getter: Box<dyn Fn(&S) -> A>,
    setter: Box<dyn Fn(A, &S) -> S>,
}
```

Slightly more runtime cost (heap allocation per Lens), but composition is straightforward and the Lens type is uniform regardless of which field it accesses.

**Using direct accessor functions (the practical middle ground):**

```rust
fn get_lat(e: &Event) -> f64     { e.location.coords.lat }
fn set_lat(lat: f64, e: &Event) -> Event {
    let mut e2 = e.clone();
    e2.location.coords.lat = lat;
    e2
}

// Equivalent to over:
let e2 = set_lat(get_lat(&e) + 1.0, &e);
```

This is what Lens looks like before you generalize it. `get_lat` and `set_lat` are a Lens for `lat`, they're just not first-class yet. The Lens abstraction lets you write this pattern once and use it anywhere.

## What This Unlocks

- **One-line deep updates**: `event_lat.over(|lat| lat + 1.0, &event)` instead of nine lines of struct rebuilding.
- **Reusable field accessors**: define `coords_to_lat` once; compose it into `event_lat`, `user_home_lat`, `delivery_lat` — any path that ends at a `Coords.lat` reuses the same terminal Lens.
- **Type-safe navigation**: the types `S` and `A` enforce that Lenses compose correctly — if `Location→Coords` doesn't connect to `Event→Coords`, the compiler rejects it.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lens composition | Function composition, infix `@@` | `.compose()` method consuming `self` |
| Immutable update | `{ r with field = v }` — no clone needed | `Clone` required at each level |
| Zero-cost Lens | Via functors (Van Laarhoven encoding) | Type-parameterized `Lens<S,A,F,G>` |
| Ergonomic Lens | Simple record of functions | `Box<dyn Fn>` in `SimpleLens<S,A>` |
| Libraries | `Accessor`, `Lens` | `lens`, `lenses` crates |
