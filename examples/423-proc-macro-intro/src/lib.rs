//! Procedural Macro Introduction
//!
//! Understanding proc macros without implementing them.

/// Proc macros operate on token streams.
/// This example shows the concepts, not actual proc macro code.

/// Example: what a derive macro generates.
/// #[derive(MyDebug)] on Point would generate something like:
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

/// Example: what an attribute macro might do.
/// #[log_calls] on a function adds logging.
pub fn example_function(x: i32) -> i32 {
    // Imagine: println!("Entering example_function");
    let result = x * 2;
    // Imagine: println!("Exiting example_function");
    result
}

/// Three types of proc macros:
/// 1. Derive macros: #[derive(Trait)]
/// 2. Attribute macros: #[attribute]
/// 3. Function-like macros: my_macro!(...)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_format() {
        let p = Point { x: 1, y: 2 };
        let s = format!("{:?}", p);
        assert!(s.contains("Point"));
        assert!(s.contains("x: 1"));
    }

    #[test]
    fn test_example_function() {
        assert_eq!(example_function(5), 10);
    }

    #[test]
    fn test_point_fields() {
        let p = Point { x: 3, y: 4 };
        assert_eq!(p.x, 3);
        assert_eq!(p.y, 4);
    }

    #[test]
    fn test_debug_struct() {
        let p = Point { x: 0, y: 0 };
        assert_eq!(format!("{:?}", p), "Point { x: 0, y: 0 }");
    }

    #[test]
    fn test_pretty_debug() {
        let p = Point { x: 1, y: 2 };
        let s = format!("{:#?}", p);
        assert!(s.contains("Point"));
    }
}
