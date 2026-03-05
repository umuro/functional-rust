# OCaml vs Rust: Hash, Eq, and Ord Traits

## Side-by-Side Code

### OCaml — Module-based comparison
```ocaml
type priority = Low | Medium | High | Critical

let int_of_priority = function
  | Low -> 0 | Medium -> 1 | High -> 2 | Critical -> 3

let compare_priority a b =
  compare (int_of_priority a) (int_of_priority b)

module PriorityMap = Map.Make(struct
  type t = priority
  let compare = compare_priority
end)

let () =
  let map = PriorityMap.(empty |> add High "urgent") in
  match PriorityMap.find_opt High map with
  | Some v -> print_endline v
  | None -> ()
```

### Rust — Trait-based comparison
```rust
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Priority { Low, Medium, High, Critical }

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Priority {
    fn value(&self) -> u8 {
        match self { Self::Low => 0, Self::Medium => 1,
                     Self::High => 2, Self::Critical => 3 }
    }
}

fn main() {
    let mut map = BTreeMap::new();
    map.insert(Priority::High, "urgent");
    println!("{:?}", map.get(&Priority::High));
}
```

---

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Equality | Structural by default | `PartialEq` / `Eq` traits |
| Ordering | `compare` function | `PartialOrd` / `Ord` traits |
| Hashing | `Hashtbl.hash` | `Hash` trait |
| HashMap key | Any type (structural) | Must impl `Hash + Eq` |
| BTreeMap key | Needs `compare` via functor | Must impl `Ord` |
| Derivable | No (write compare manually) | `#[derive(Hash, Eq, Ord)]` |

---

## The Trait Hierarchy

```
PartialEq  →  Eq (total equality, no NaN-like values)
     ↓
PartialOrd →  Ord (total ordering)
```

- `PartialEq`: `==` and `!=`
- `Eq`: Marker trait indicating reflexivity (`a == a` always true)
- `PartialOrd`: `<`, `<=`, `>`, `>=`, returns `Option<Ordering>`
- `Ord`: Total ordering, returns `Ordering` directly

---

## Hash Consistency Rule

If you implement `Hash`, it must be consistent with `Eq`:

```rust
// RULE: a == b implies hash(a) == hash(b)

impl PartialEq for CaseInsensitive {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Hash for CaseInsensitive {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Must hash the same thing we compare!
        self.0.to_lowercase().hash(state);
    }
}
```

---

## 5 Takeaways

1. **`#[derive(PartialEq, Eq, Hash)]` covers most cases.**
   Use derive unless you need custom semantics.

2. **`Ord` requires `Eq`; `PartialOrd` requires `PartialEq`.**
   The trait hierarchy ensures consistency.

3. **OCaml uses functors; Rust uses trait bounds.**
   `Map.Make(...)` vs `BTreeMap<K: Ord, V>`.

4. **Hash must be consistent with Eq.**
   Equal values must hash to the same value.

5. **`Ord` is for BTreeMap; `Hash + Eq` is for HashMap.**
   Different collections require different traits.
