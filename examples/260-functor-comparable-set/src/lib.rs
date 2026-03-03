// A generic ordered set backed by a sorted Vec, parameterised by the element
// type's `Ord` bound — the Rust equivalent of OCaml's `Map.Make` / `Set.Make`
// functor pattern.
//
// OCaml uses a *functor* (a module-level function) to create a new Set module
// for a specific `COMPARABLE` type.  Rust achieves the same result with a
// generic struct and a trait bound: `ComparableSet<T: Ord>`.

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — generic struct with Ord trait bound
// ---------------------------------------------------------------------------

/// An ordered, deduplicated set of elements that implement `Ord`.
///
/// Mirrors OCaml's `Set.Make(COMPARABLE)` functor output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparableSet<T: Ord> {
    items: Vec<T>,
}

impl<T: Ord> Default for ComparableSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> ComparableSet<T> {
    /// Create an empty set — `MakeSet(C).empty`.
    pub fn new() -> Self {
        ComparableSet { items: Vec::new() }
    }

    /// Return `true` if `x` is a member — `mem x s`.
    pub fn contains(&self, x: &T) -> bool {
        self.items.binary_search(x).is_ok()
    }

    /// Insert `x`, preserving sorted order and uniqueness — `add x s`.
    ///
    /// Returns a new set (immutable-style), matching the OCaml API where
    /// `add` returns a new set value rather than mutating in place.
    #[must_use]
    pub fn insert(mut self, x: T) -> Self {
        match self.items.binary_search(&x) {
            Ok(_) => self, // already present — set unchanged
            Err(pos) => {
                self.items.insert(pos, x);
                self
            }
        }
    }

    /// Return a sorted slice of all elements — `to_list s`.
    pub fn to_sorted_vec(&self) -> &[T] {
        &self.items
    }

    /// Number of elements in the set.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Return `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Solution 2: Functional / recursive style — closer to OCaml list-based impl
// ---------------------------------------------------------------------------
//
// OCaml's `MakeSet` stores elements in an unsorted list and sorts on `to_list`.
// We replicate that approach with a wrapper that defers sorting.

/// An unordered, deduplicated set of elements backed by a `Vec`.
///
/// Matches the OCaml implementation strategy: insertion is O(n), `to_list`
/// sorts on demand.  Contrast with `ComparableSet` which keeps items sorted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctorSet<T: Ord> {
    items: Vec<T>,
}

impl<T: Ord> Default for FunctorSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> FunctorSet<T> {
    pub fn new() -> Self {
        FunctorSet { items: Vec::new() }
    }

    /// OCaml: `let mem x = List.exists (fun y -> C.compare x y = 0)`
    pub fn mem(&self, x: &T) -> bool {
        self.items.iter().any(|y| y == x)
    }

    /// OCaml: `let add x s = if mem x s then s else x :: s`
    ///
    /// Renamed to `push` to avoid confusion with `std::ops::Add`.
    #[must_use]
    pub fn push(mut self, x: T) -> Self {
        if self.mem(&x) {
            self
        } else {
            self.items.push(x);
            self
        }
    }

    /// OCaml: `let to_list s = List.sort C.compare s`
    pub fn to_list(&self) -> Vec<&T> {
        let mut sorted: Vec<&T> = self.items.iter().collect();
        sorted.sort();
        sorted
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- ComparableSet (idiomatic) ---

    #[test]
    fn test_comparable_set_empty() {
        let s: ComparableSet<i32> = ComparableSet::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert!(!s.contains(&1));
    }

    #[test]
    fn test_comparable_set_single_insert() {
        let s = ComparableSet::new().insert(42);
        assert_eq!(s.len(), 1);
        assert!(s.contains(&42));
        assert!(!s.contains(&0));
    }

    #[test]
    fn test_comparable_set_deduplication() {
        let s = ComparableSet::new()
            .insert(3)
            .insert(1)
            .insert(3) // duplicate — ignored
            .insert(2);
        assert_eq!(s.len(), 3);
        assert_eq!(s.to_sorted_vec(), &[1, 2, 3]);
    }

    #[test]
    fn test_comparable_set_sorted_order() {
        let s = ComparableSet::new()
            .insert(5)
            .insert(1)
            .insert(4)
            .insert(2)
            .insert(3);
        assert_eq!(s.to_sorted_vec(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_comparable_set_strings() {
        let s = ComparableSet::new()
            .insert("banana")
            .insert("apple")
            .insert("cherry")
            .insert("apple"); // duplicate
        assert_eq!(s.len(), 3);
        assert_eq!(s.to_sorted_vec(), &["apple", "banana", "cherry"]);
    }

    // --- FunctorSet (OCaml-style functional) ---

    #[test]
    fn test_functor_set_empty() {
        let s: FunctorSet<i32> = FunctorSet::new();
        assert!(!s.mem(&1));
        assert_eq!(s.to_list(), Vec::<&i32>::new());
    }

    #[test]
    fn test_functor_set_push_and_mem() {
        let s = FunctorSet::new().push(10).push(20).push(10); // 10 deduplicated
        assert!(s.mem(&10));
        assert!(s.mem(&20));
        assert!(!s.mem(&30));
        assert_eq!(s.items.len(), 2);
    }

    #[test]
    fn test_functor_set_to_list_sorted() {
        // mirrors the OCaml: IntSet.(empty |> add 3 |> add 1 |> add 3 |> add 2)
        let s = FunctorSet::new().push(3).push(1).push(3).push(2);
        assert_eq!(s.to_list(), vec![&1, &2, &3]);
    }

    #[test]
    fn test_functor_set_string() {
        let s = FunctorSet::new()
            .push("gamma")
            .push("alpha")
            .push("beta")
            .push("alpha");
        assert_eq!(s.to_list(), vec![&"alpha", &"beta", &"gamma"]);
    }
}
