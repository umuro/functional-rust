/// A zipper over a non-empty list — O(1) navigation and focused update.
///
/// The `left` field stores elements to the left of `focus` in **reverse** order
/// (the head of `left` is the immediate left neighbour), matching the OCaml original.
#[derive(Debug, Clone, PartialEq)]
pub struct Zipper<T> {
    pub left: Vec<T>,
    pub focus: T,
    pub right: Vec<T>,
}

// ── Construction ──────────────────────────────────────────────────────────────

/// Build a zipper from a non-empty iterator, focusing on the first element.
/// Returns `None` if the iterator is empty.
pub fn of_iter<T>(mut iter: impl Iterator<Item = T>) -> Option<Zipper<T>> {
    let focus = iter.next()?;
    Some(Zipper {
        left: vec![],
        focus,
        right: iter.collect(),
    })
}

/// Convenience wrapper: build from a slice.
pub fn of_slice<T: Clone>(slice: &[T]) -> Option<Zipper<T>> {
    of_iter(slice.iter().cloned())
}

// ── Navigation ────────────────────────────────────────────────────────────────

/// Move the focus one step to the right. Returns `None` at the right boundary.
pub fn go_right<T>(z: Zipper<T>) -> Option<Zipper<T>> {
    let mut right = z.right;
    if right.is_empty() {
        return None;
    }
    let new_focus = right.remove(0);
    let mut left = z.left;
    left.insert(0, z.focus);
    Some(Zipper {
        left,
        focus: new_focus,
        right,
    })
}

/// Move the focus one step to the left. Returns `None` at the left boundary.
pub fn go_left<T>(z: Zipper<T>) -> Option<Zipper<T>> {
    let mut left = z.left;
    if left.is_empty() {
        return None;
    }
    let new_focus = left.remove(0); // head of left = immediate left neighbour
    let mut right = z.right;
    right.insert(0, z.focus);
    Some(Zipper {
        left,
        focus: new_focus,
        right,
    })
}

// ── Modification ──────────────────────────────────────────────────────────────

/// Replace the focused element by applying `f` to it.
pub fn update<T, F: FnOnce(T) -> T>(z: Zipper<T>, f: F) -> Zipper<T> {
    Zipper {
        focus: f(z.focus),
        ..z
    }
}

/// Collect the zipper back into a `Vec` in logical left-to-right order.
pub fn to_vec<T>(z: Zipper<T>) -> Vec<T> {
    let mut result: Vec<T> = z.left.into_iter().rev().collect();
    result.push(z.focus);
    result.extend(z.right);
    result
}

// ── Idiomatic struct-method API ───────────────────────────────────────────────
//
// Many Rust developers prefer methods on the struct rather than free functions.
// This mirrors a more OO-friendly cursor API without losing the functional core.

impl<T> Zipper<T> {
    /// Build from a non-empty slice (idiomatic constructor).
    pub fn from_slice(slice: &[T]) -> Option<Self>
    where
        T: Clone,
    {
        of_slice(slice)
    }

    /// Move right, consuming self; returns `None` at the boundary.
    pub fn move_right(self) -> Option<Self> {
        go_right(self)
    }

    /// Move left, consuming self; returns `None` at the boundary.
    pub fn move_left(self) -> Option<Self> {
        go_left(self)
    }

    /// Apply `f` to the focused element in-place.
    pub fn map_focus<F: FnOnce(T) -> T>(self, f: F) -> Self {
        update(self, f)
    }

    /// Reconstruct the full list.
    pub fn into_vec(self) -> Vec<T> {
        to_vec(self)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_returns_none() {
        assert_eq!(of_slice::<i32>(&[]), None);
    }

    #[test]
    fn test_single_element_navigation() {
        let z = of_slice(&[42]).unwrap();
        assert_eq!(z.focus, 42);
        assert!(go_right(z.clone()).is_none());
        assert!(go_left(z).is_none());
    }

    #[test]
    fn test_go_right_advances_focus() {
        let z = of_slice(&[1, 2, 3]).unwrap();
        let z = go_right(z).unwrap();
        assert_eq!(z.focus, 2);
        let z = go_right(z).unwrap();
        assert_eq!(z.focus, 3);
        assert!(go_right(z).is_none());
    }

    #[test]
    fn test_go_left_retreats_focus() {
        let z = of_slice(&[1, 2, 3]).unwrap();
        let z = go_right(z).unwrap();
        let z = go_right(z).unwrap();
        assert_eq!(z.focus, 3);
        let z = go_left(z).unwrap();
        assert_eq!(z.focus, 2);
        let z = go_left(z).unwrap();
        assert_eq!(z.focus, 1);
        assert!(go_left(z).is_none());
    }

    #[test]
    fn test_update_focus() {
        let z = of_slice(&[1, 2, 3, 4, 5]).unwrap();
        let z = go_right(z).unwrap();
        let z = go_right(z).unwrap(); // focus = 3
        let z = update(z, |x| x * 10);
        assert_eq!(to_vec(z), vec![1, 2, 30, 4, 5]);
    }

    #[test]
    fn test_ocaml_example_sequence() {
        // Mirrors the OCaml demo: [1;2;3;4;5], move right twice, multiply focus by 10
        let z = of_slice(&[1, 2, 3, 4, 5]).unwrap();
        let z = go_right(z).unwrap();
        let z = go_right(z).unwrap();
        let z = update(z, |x| x * 10);
        assert_eq!(to_vec(z), vec![1, 2, 30, 4, 5]);
    }

    #[test]
    fn test_round_trip_preserves_order() {
        let original = vec![10, 20, 30, 40];
        let z = of_slice(&original).unwrap();
        assert_eq!(to_vec(z), original);
    }

    #[test]
    fn test_method_api_matches_free_functions() {
        let z = Zipper::from_slice(&[1, 2, 3, 4]).unwrap();
        let result = z.move_right().unwrap().map_focus(|x| x + 100).into_vec();
        assert_eq!(result, vec![1, 102, 3, 4]);
    }
}
