#![allow(clippy::all)]
//! Derive Macro Concepts
//!
//! Understanding what derive macros generate.

/// A point with derived traits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

/// Manual Debug implementation for comparison.
pub struct ManualDebug {
    pub value: i32,
}

impl std::fmt::Debug for ManualDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ManualDebug")
            .field("value", &self.value)
            .finish()
    }
}

/// Manual Clone implementation.
impl Clone for ManualDebug {
    fn clone(&self) -> Self {
        ManualDebug { value: self.value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_debug_derive() {
        let p = Point::new(1, 2);
        assert_eq!(format!("{:?}", p), "Point { x: 1, y: 2 }");
    }

    #[test]
    fn test_clone_derive() {
        let p1 = Point::new(3, 4);
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_copy_derive() {
        let p1 = Point::new(5, 6);
        let p2 = p1;
        let p3 = p1; // Still valid (Copy)
        assert_eq!(p2, p3);
    }

    #[test]
    fn test_hash_derive() {
        let mut set = HashSet::new();
        set.insert(Point::new(1, 1));
        set.insert(Point::new(1, 1)); // Duplicate
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_default_derive() {
        let p = Point::default();
        assert_eq!(p, Point::new(0, 0));
    }

    #[test]
    fn test_manual_debug() {
        let m = ManualDebug { value: 42 };
        assert!(format!("{:?}", m).contains("42"));
    }
}
