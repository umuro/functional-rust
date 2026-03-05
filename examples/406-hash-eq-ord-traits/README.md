📖 **[View on hightechmind.io →](https://hightechmind.io/rust/406-hash-eq-ord-traits)**

---

# 406: Hash, Eq, and Ord Traits

**Difficulty:** 2  **Level:** Intermediate

Implement comparison and hashing for custom types to use them in collections and sorting.

## The Problem This Solves

You've defined a `Point` struct and want to use it as a `HashMap` key. Or a `Task` enum and want to sort tasks by priority. These operations require the compiler to know how to compare or hash your type — information that doesn't exist until you provide it.

`PartialEq` and `Eq` define equality. `PartialOrd` and `Ord` define ordering. `Hash` enables hash-based collections. In most cases, `#[derive]` does the right thing automatically. But when you need *domain-specific* semantics — tasks sorted by priority rather than alphabetically, floats compared by business rules rather than IEEE 754 edge cases — you implement these traits manually.

Understanding the trait relationships also prevents bugs: `Eq` requires `PartialEq`. `Ord` requires `PartialOrd` and `Eq`. If you implement `Hash`, you must ensure that equal values produce the same hash — a property derive handles automatically but manual impls can violate.

## The Intuition

There are four comparison traits in a hierarchy:
- `PartialEq` — equality, possibly partial (e.g., `f64`: `NaN != NaN`)
- `Eq` — total equality (marker: all values compare equal to themselves)
- `PartialOrd` — ordering, possibly partial (e.g., `f64` has incomparable NaN)
- `Ord` — total ordering (every pair of values has a defined order)

For custom types, `derive` generates field-by-field comparison in declaration order. Manual `impl Ord` lets you define any ordering — by priority value, by composite key, by domain-specific rules.

`Hash` must be consistent with `Eq`: if `a == b`, then `hash(a) == hash(b)`. Derive handles this automatically. Manual `impl Hash` requires care to maintain this invariant.

## How It Works in Rust

```rust
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, BTreeMap};

// Derive: lexicographic comparison, hash by fields
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point { x: i32, y: i32 }

// Custom ordering: enum by priority value, not declaration order
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Priority { Low, Medium, High, Critical }

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        fn val(p: &Priority) -> u8 {
            match p { Priority::Low => 0, Priority::Medium => 1,
                      Priority::High => 2, Priority::Critical => 3 }
        }
        val(self).cmp(&val(other))
    }
}

// Composite ordering: higher priority first, then alphabetical name
#[derive(Debug, Clone, PartialEq, Eq)]
struct Task { name: String, priority: Priority }

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)  // reversed: higher priority sorts first
            .then(self.name.cmp(&other.name))  // tiebreak by name
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

fn main() {
    // Hash: Point as HashMap key (requires Eq + Hash)
    let mut map: HashMap<Point, String> = HashMap::new();
    map.insert(Point { x: 0, y: 0 }, "origin".to_string());
    println!("{:?}", map[&Point { x: 0, y: 0 }]);

    // HashSet deduplication (requires Eq + Hash)
    let mut set: HashSet<Point> = HashSet::new();
    set.insert(Point { x: 1, y: 1 });
    set.insert(Point { x: 1, y: 1 }); // duplicate — ignored
    println!("Set size: {}", set.len()); // 1

    // Sort by custom Ord
    let mut tasks = vec![
        Task { name: "Write docs".to_string(), priority: Priority::Low },
        Task { name: "Fix bug".to_string(), priority: Priority::Critical },
        Task { name: "Deploy".to_string(), priority: Priority::High },
    ];
    tasks.sort();  // uses our custom Ord
    for t in &tasks {
        println!("[{:?}] {}", t.priority, t.name);
    }

    // BTreeMap requires Ord on keys
    let _btree: BTreeMap<Priority, String> = BTreeMap::new();
}
```

## What This Unlocks

- **Custom collection keys** — any type with `Eq + Hash` works as a `HashMap`/`HashSet` key; any type with `Ord` works in `BTreeMap`/`BTreeSet`.
- **Domain-aware sorting** — `tasks.sort()` using your ordering beats manual comparator functions; `BinaryHeap` works directly with your `Ord` for priority queues.
- **Protocol compliance** — `PartialEq` enables `assert_eq!`, `==` in tests, and `contains()` on collections; derivable for most types, manual when needed.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Comparison | Polymorphic `compare` — works on all types, structural | Traits: `PartialOrd`/`Ord` — explicit, custom per type |
| Equality | Polymorphic `=` — structural by default | `PartialEq`/`Eq` — derived or manual; `f64` is only `PartialEq` |
| Hash | `Hashtbl.hash` — polymorphic, internal | `Hash` trait — explicit, consistent with `Eq` |
| Custom ordering | `Map.Make(struct type t = ... let compare = ... end)` | `impl Ord for T` — used by all stdlib sorted collections |
