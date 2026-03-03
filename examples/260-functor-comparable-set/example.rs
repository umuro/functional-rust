// Solution 1: Idiomatic Rust — sorted Vec with binary search, Ord trait bound
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
    pub fn new() -> Self {
        ComparableSet { items: Vec::new() }
    }

    pub fn contains(&self, x: &T) -> bool {
        self.items.binary_search(x).is_ok()
    }

    #[must_use]
    pub fn insert(mut self, x: T) -> Self {
        match self.items.binary_search(&x) {
            Ok(_) => self,
            Err(pos) => {
                self.items.insert(pos, x);
                self
            }
        }
    }

    pub fn to_sorted_vec(&self) -> &[T] {
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// Solution 2: OCaml-style functional — unsorted Vec, sort on to_list
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

    pub fn mem(&self, x: &T) -> bool {
        self.items.iter().any(|y| y == x)
    }

    #[must_use]
    pub fn push(mut self, x: T) -> Self {
        if self.mem(&x) {
            self
        } else {
            self.items.push(x);
            self
        }
    }

    pub fn to_list(&self) -> Vec<&T> {
        let mut sorted: Vec<&T> = self.items.iter().collect();
        sorted.sort();
        sorted
    }
}

fn main() {
    // ComparableSet — idiomatic, sorted on insertion
    let int_set = ComparableSet::new()
        .insert(3)
        .insert(1)
        .insert(3) // duplicate
        .insert(2);
    println!("ComparableSet integers: {:?}", int_set.to_sorted_vec());
    println!("contains(1): {}", int_set.contains(&1));
    println!("contains(5): {}", int_set.contains(&5));

    let str_set = ComparableSet::new()
        .insert("banana")
        .insert("apple")
        .insert("cherry")
        .insert("apple"); // duplicate
    println!("ComparableSet strings:  {:?}", str_set.to_sorted_vec());

    // FunctorSet — OCaml-style, mirrors: IntSet.(empty |> add 3 |> add 1 |> add 3 |> add 2)
    let fs = FunctorSet::new().push(3).push(1).push(3).push(2);
    let sorted: Vec<_> = fs.to_list();
    println!("FunctorSet to_list:     {:?}", sorted);
}

/* Output:
   ComparableSet integers: [1, 2, 3]
   contains(1): true
   contains(5): false
   ComparableSet strings:  ["apple", "banana", "cherry"]
   FunctorSet to_list:     [1, 2, 3]
*/

#[cfg(test)]
mod tests {
    use super::*;

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
        let s = ComparableSet::new().insert(3).insert(1).insert(3).insert(2);
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
            .insert("apple");
        assert_eq!(s.len(), 3);
        assert_eq!(s.to_sorted_vec(), &["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_functor_set_empty() {
        let s: FunctorSet<i32> = FunctorSet::new();
        assert!(!s.mem(&1));
        assert_eq!(s.to_list(), Vec::<&i32>::new());
    }

    #[test]
    fn test_functor_set_push_and_mem() {
        let s = FunctorSet::new().push(10).push(20).push(10);
        assert!(s.mem(&10));
        assert!(s.mem(&20));
        assert!(!s.mem(&30));
        assert_eq!(s.items.len(), 2);
    }

    #[test]
    fn test_functor_set_to_list_sorted() {
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
