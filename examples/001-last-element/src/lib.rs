// Find the last element of a list
//
// Three implementations showing different Rust idioms.
// All functions borrow the slice (&[T]) — no ownership transfer,
// and return Option<&T>, borrowing from the input.

// Solution 1: Idiomatic — delegate to the standard library.
// `slice::last()` is O(1): it reads the final index directly.
// The returned reference borrows from `list`, so the caller cannot
// mutate or drop `list` while the reference is live.
pub fn last<T>(list: &[T]) -> Option<&T> {
    list.last()
}

// Solution 2: Stdlib-based via iterator.
// `Iterator::last()` consumes the iterator in O(n) time.
// Useful as a reminder that iterators are lazy; here every element
// is visited just to reach the end — prefer `slice::last()` in practice.
pub fn last_stdlib<T>(list: &[T]) -> Option<&T> {
    list.iter().last()
}

// Solution 3: Recursive pattern matching (closest to the OCaml original).
// Rust slice patterns let us destructure `[head, rest @ ..]` the same way
// OCaml matches `_ :: t`. This is NOT tail-call optimised by the compiler,
// so very long lists could overflow the stack — idiomatic Rust prefers
// iteration over recursion for this reason.
pub fn last_recursive<T>(list: &[T]) -> Option<&T> {
    match list {
        // Empty slice — no element to return.
        [] => None,
        // Single element — this IS the last one.
        [x] => Some(x),
        // Head + tail: discard head, recurse into the tail.
        // `rest` is a &[T] sub-slice; no allocation occurs.
        [_, rest @ ..] => last_recursive(rest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Empty list: every implementation must return None.
    #[test]
    fn test_empty() {
        let empty: &[i32] = &[];
        assert_eq!(last(empty), None);
        assert_eq!(last_stdlib(empty), None);
        assert_eq!(last_recursive(empty), None);
    }

    // Single element: the only element is also the last.
    #[test]
    fn test_single() {
        assert_eq!(last(&[42]), Some(&42));
        assert_eq!(last_stdlib(&[42]), Some(&42));
        assert_eq!(last_recursive(&[42]), Some(&42));
    }

    // Multiple integers: last element is 4.
    #[test]
    fn test_multiple_integers() {
        let list = [1, 2, 3, 4];
        assert_eq!(last(&list), Some(&4));
        assert_eq!(last_stdlib(&list), Some(&4));
        assert_eq!(last_recursive(&list), Some(&4));
    }

    // Multiple strings: mirrors the OCaml test case exactly.
    #[test]
    fn test_multiple_strings() {
        let list = ["a", "b", "c", "d"];
        assert_eq!(last(&list), Some(&"d"));
        assert_eq!(last_stdlib(&list), Some(&"d"));
        assert_eq!(last_recursive(&list), Some(&"d"));
    }

    // All three implementations must agree on the same result.
    #[test]
    fn test_all_implementations_agree() {
        let list = [10, 20, 30, 40, 50];
        let expected = Some(&50);
        assert_eq!(last(&list), expected);
        assert_eq!(last_stdlib(&list), expected);
        assert_eq!(last_recursive(&list), expected);
    }
}
