// Reverse the elements of a list. OCaml uses a tail-recursive accumulator.
// Rust offers multiple approaches — in-place mutation, iterator collect, and
// a functional fold that mirrors OCaml's accumulator pattern.

// ---------------------------------------------------------------------------
// Approach 1: Idiomatic Rust — clone + reverse (in-place mutation)
// ---------------------------------------------------------------------------
// `.rev()` on iterators is lazy; collecting reverses during iteration.
// We return a new Vec to match OCaml's immutable semantics.
pub fn reverse<T: Clone>(slice: &[T]) -> Vec<T> {
    slice.iter().rev().cloned().collect()
}

// ---------------------------------------------------------------------------
// Approach 2: Fold — mirrors OCaml's accumulator pattern
// ---------------------------------------------------------------------------
// OCaml: `let rec aux acc = function | [] -> acc | h :: t -> aux (h :: acc) t`
// Rust fold equivalent: prepend each element to accumulator.
// We use `insert(0, ...)` which is O(n²) total — illustrative, not optimal.
pub fn reverse_fold<T: Clone>(slice: &[T]) -> Vec<T> {
    slice.iter().fold(Vec::new(), |mut acc, item| {
        acc.insert(0, item.clone());
        acc
    })
}

// ---------------------------------------------------------------------------
// Approach 3: Recursive — closest to OCaml's structure
// ---------------------------------------------------------------------------
// Slice pattern matching with accumulator passed through recursion.
pub fn reverse_recursive<T: Clone>(slice: &[T]) -> Vec<T> {
    fn aux<T: Clone>(acc: Vec<T>, slice: &[T]) -> Vec<T> {
        match slice {
            [] => acc,
            [head, rest @ ..] => {
                let mut new_acc = vec![head.clone()];
                new_acc.extend(acc);
                aux(new_acc, rest)
            }
        }
    }
    aux(Vec::new(), slice)
}

// ---------------------------------------------------------------------------
// Approach 4: In-place mutation (truly idiomatic for owned data)
// ---------------------------------------------------------------------------
// When you own the Vec, just mutate it — zero allocations.
pub fn reverse_in_place<T>(slice: &mut [T]) {
    slice.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(reverse::<i32>(&[]), vec![]);
        assert_eq!(reverse_fold::<i32>(&[]), vec![]);
        assert_eq!(reverse_recursive::<i32>(&[]), vec![]);
    }

    #[test]
    fn test_single() {
        assert_eq!(reverse(&[1]), vec![1]);
        assert_eq!(reverse_fold(&[1]), vec![1]);
        assert_eq!(reverse_recursive(&[1]), vec![1]);
    }

    #[test]
    fn test_multiple() {
        assert_eq!(reverse(&[1, 2, 3, 4]), vec![4, 3, 2, 1]);
        assert_eq!(reverse_fold(&[1, 2, 3, 4]), vec![4, 3, 2, 1]);
        assert_eq!(reverse_recursive(&[1, 2, 3, 4]), vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_strings() {
        assert_eq!(reverse(&["a", "b", "c"]), vec!["c", "b", "a"]);
    }

    #[test]
    fn test_in_place() {
        let mut v = vec![1, 2, 3, 4];
        reverse_in_place(&mut v);
        assert_eq!(v, vec![4, 3, 2, 1]);
    }
}
