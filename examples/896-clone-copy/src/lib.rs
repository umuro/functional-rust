// Example 896: Clone and Copy Traits
//
// Copy: bitwise copy, implicit, for simple stack types (i32, f64, bool, tuples of Copy, etc.)
// Clone: explicit deep copy via .clone(), for heap-owning types (String, Vec, etc.)
//
// The key insight: seeing `.clone()` in code signals a potentially expensive heap allocation.
// Not seeing it means the copy is cheap (stack-only). No hidden costs.

// --- Copy types ---

/// Demonstrates Copy semantics: assignment silently duplicates the value.
/// Both the original and the copy are independently valid after assignment.
pub fn copy_integer(x: i32) -> (i32, i32) {
    let y = x; // bitwise copy — x is still valid
    (x, y)
}

/// Tuples of Copy types are themselves Copy.
pub fn copy_tuple(p: (f64, f64), dx: f64, dy: f64) -> (f64, f64) {
    let q = p; // p is copied, both remain valid
    (q.0 + dx, q.1 + dy)
}

/// A simple point struct that derives both Copy and Clone.
/// Copy enables silent duplication; Clone enables `.clone()` calls.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Returns a new translated point — original is preserved via Copy.
    pub fn translate(self, dx: f64, dy: f64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

// --- Clone types ---

/// Demonstrates Clone semantics: explicit `.clone()` required for heap-owning types.
/// After `s2 = s1.clone()`, both strings are independent heap allocations.
pub fn clone_string(s: &str) -> (String, String) {
    let s1 = s.to_owned();
    let s2 = s1.clone(); // explicit — you see the cost
    (s1, s2)
}

/// Clones a Vec: each clone is a fresh heap allocation with copied elements.
pub fn clone_vec(v: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let v1 = v.to_vec();
    let v2 = v1.clone(); // explicit deep copy
    (v1, v2)
}

/// Shows that modifying a cloned value does not affect the original.
pub fn independent_after_clone(original: &str) -> (String, String) {
    let s1 = original.to_owned();
    let mut s2 = s1.clone();
    s2.push_str(" (copy)");
    (s1, s2)
}

// --- Mixed: a struct that owns heap data, so only Clone (not Copy) ---

/// A named point that owns its label string — cannot be Copy, only Clone.
#[derive(Debug, Clone, PartialEq)]
pub struct NamedPoint {
    pub label: String,
    pub x: f64,
    pub y: f64,
}

impl NamedPoint {
    pub fn new(label: &str, x: f64, y: f64) -> Self {
        Self {
            label: label.to_owned(),
            x,
            y,
        }
    }

    pub fn translate(&self, dx: f64, dy: f64) -> Self {
        Self {
            label: self.label.clone(), // must clone the String
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_integer_both_valid() {
        let (x, y) = copy_integer(42);
        assert_eq!(x, 42);
        assert_eq!(y, 42);
    }

    #[test]
    fn test_copy_tuple_independence() {
        let p = (1.0_f64, 2.0_f64);
        let translated = copy_tuple(p, 3.0, 4.0);
        // original p is still (1.0, 2.0) — Copy means independent
        assert_eq!(p, (1.0, 2.0));
        assert_eq!(translated, (4.0, 6.0));
    }

    #[test]
    fn test_copy_struct_translate_preserves_original() {
        let origin = Point::new(0.0, 0.0);
        let moved = origin.translate(1.0, 2.0);
        // origin unchanged — Point is Copy
        assert_eq!(origin, Point::new(0.0, 0.0));
        assert_eq!(moved, Point::new(1.0, 2.0));
    }

    #[test]
    fn test_clone_string_independence() {
        let (s1, s2) = clone_string("hello");
        assert_eq!(s1, "hello");
        assert_eq!(s2, "hello");
        // they are equal but independent allocations
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_clone_vec_independence() {
        let (v1, v2) = clone_vec(&[1, 2, 3]);
        assert_eq!(v1, vec![1, 2, 3]);
        assert_eq!(v2, vec![1, 2, 3]);
    }

    #[test]
    fn test_independent_after_clone_no_aliasing() {
        let (original, copy) = independent_after_clone("hello");
        assert_eq!(original, "hello");
        assert_eq!(copy, "hello (copy)");
        // modifying the clone did not affect the original
        assert_ne!(original, copy);
    }

    #[test]
    fn test_named_point_clone_and_translate() {
        let np = NamedPoint::new("origin", 0.0, 0.0);
        let moved = np.translate(5.0, -3.0);
        // np is still valid — we borrowed it in translate
        assert_eq!(np.label, "origin");
        assert_eq!(np.x, 0.0);
        assert_eq!(moved.label, "origin");
        assert_eq!(moved.x, 5.0);
        assert_eq!(moved.y, -3.0);
    }

    #[test]
    fn test_named_point_clone_is_independent() {
        let np1 = NamedPoint::new("point", 1.0, 2.0);
        let mut np2 = np1.clone();
        np2.label = "clone".to_owned();
        assert_eq!(np1.label, "point"); // original unaffected
        assert_eq!(np2.label, "clone");
    }
}
