// Find the last two elements of a list, returning them as a tuple.

// ---------------------------------------------------------------------------
// Approach 1: Idiomatic Rust — slice pattern matching
// ---------------------------------------------------------------------------
// Slices in Rust support pattern matching similar to OCaml's list patterns.
// We borrow the slice (`&[T]`) to avoid taking ownership.
pub fn last_two<T>(slice: &[T]) -> Option<(&T, &T)> {
    let len = slice.len();
    if len < 2 {
        None
    } else {
        // Direct indexing — O(1) since slices are contiguous memory
        Some((&slice[len - 2], &slice[len - 1]))
    }
}

// ---------------------------------------------------------------------------
// Approach 2: Functional — recursive, mirrors the OCaml version
// ---------------------------------------------------------------------------
// Uses slice patterns `[x, y]` and `[_, rest @ ..]` for destructuring.
// Note: Rust doesn't guarantee TCO, but this is educational.
pub fn last_two_recursive<T>(slice: &[T]) -> Option<(&T, &T)> {
    match slice {
        // Empty or single element — no pair exists
        [] | [_] => None,
        // Exactly two elements — base case
        [x, y] => Some((x, y)),
        // More than two — recurse on tail (skip first element)
        [_, rest @ ..] => last_two_recursive(rest),
    }
}

// ---------------------------------------------------------------------------
// Approach 3: Iterator-based — using windows
// ---------------------------------------------------------------------------
// `windows(2)` gives sliding pairs; we take the last one.
pub fn last_two_windows<T>(slice: &[T]) -> Option<(&T, &T)> {
    // windows(2) yields &[T] slices of length 2
    slice.windows(2).last().map(|w| (&w[0], &w[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(last_two::<i32>(&[]), None);
        assert_eq!(last_two_recursive::<i32>(&[]), None);
        assert_eq!(last_two_windows::<i32>(&[]), None);
    }

    #[test]
    fn test_single() {
        assert_eq!(last_two(&[1]), None);
        assert_eq!(last_two_recursive(&[1]), None);
        assert_eq!(last_two_windows(&[1]), None);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(last_two(&[1, 2]), Some((&1, &2)));
        assert_eq!(last_two_recursive(&[1, 2]), Some((&1, &2)));
        assert_eq!(last_two_windows(&[1, 2]), Some((&1, &2)));
    }

    #[test]
    fn test_multiple() {
        assert_eq!(last_two(&[1, 2, 3, 4]), Some((&3, &4)));
        assert_eq!(last_two_recursive(&[1, 2, 3, 4]), Some((&3, &4)));
        assert_eq!(last_two_windows(&[1, 2, 3, 4]), Some((&3, &4)));
    }

    #[test]
    fn test_strings() {
        let v = ["a", "b", "c", "d"];
        assert_eq!(last_two(&v), Some((&"c", &"d")));
    }
}
