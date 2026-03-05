// Derive macros: concept and usage in Rust

// All standard derivable traits
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

// Enum derivations
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Shape {
    Circle { radius: u32 },
    Rectangle { width: u32, height: u32 },
    Triangle { base: u32, height: u32 },
}

// What derive actually generates — shown manually for Point
// (This is equivalent to #[derive(Debug)] for Point)
mod manual_impls {
    use super::Point;
    use std::fmt;

    // What #[derive(Debug)] generates:
    impl fmt::Debug for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Point")
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        }
    }
}

// Using derived traits in practice
use std::collections::{HashMap, HashSet, BTreeSet};

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 3, y: 4 };

    // Debug
    println!("{:?}", p1);
    println!("{:#?}", p1); // pretty-print

    // PartialEq / Eq
    println!("p1 == p2: {}", p1 == p2);
    println!("p1 == p3: {}", p1 == p3);

    // Ord — sorting
    let mut points = vec![p3.clone(), p1.clone(), p2.clone()];
    points.sort();
    println!("Sorted: {:?}", points);

    // Hash — use as HashMap key
    let mut map: HashMap<Point, String> = HashMap::new();
    map.insert(p1.clone(), "origin-ish".to_string());
    println!("Map lookup: {:?}", map[&p1]);

    // Clone
    let p4 = p1.clone();
    println!("Cloned: {:?}", p4);

    // Default
    let default_p = Point::default();
    println!("Default: {:?}", default_p);

    // Enum derive
    let shapes = vec![
        Shape::Circle { radius: 5 },
        Shape::Rectangle { width: 3, height: 4 },
    ];
    let mut btree: BTreeSet<Shape> = shapes.into_iter().collect();
    btree.insert(Shape::Triangle { base: 3, height: 4 });
    println!("Shapes (sorted): {:?}", btree);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_eq() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 1, y: 2 };
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_derived_ord() {
        let mut v = vec![Point { x: 3, y: 0 }, Point { x: 1, y: 0 }];
        v.sort();
        assert_eq!(v[0], Point { x: 1, y: 0 });
    }

    #[test]
    fn test_derived_clone() {
        let p = Point { x: 5, y: 6 };
        let q = p.clone();
        assert_eq!(p, q);
    }

    #[test]
    fn test_derived_default() {
        let p = Point::default();
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
    }
}
