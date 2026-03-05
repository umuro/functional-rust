//! Example 124: dyn Trait — Dynamic Dispatch
//!
//! Three strategies for polymorphism in Rust, shown side-by-side:
//!
//! 1. `dyn Trait` — fat-pointer vtable dispatch; accepts an open, heterogeneous
//!    set of types at the cost of one pointer indirection per call and heap
//!    allocation per value.
//!
//! 2. `impl Trait` / generics — monomorphized at compile time; zero overhead,
//!    but every element in a collection must be the same concrete type.
//!
//! 3. Enum dispatch — exhaustive `match`; no vtable, no heap, fastest runtime,
//!    but the set of variants is closed (you control them all).

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Shared trait
// ---------------------------------------------------------------------------

pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

// ---------------------------------------------------------------------------
// Concrete types
// ---------------------------------------------------------------------------

pub struct Circle {
    pub radius: f64,
}

pub struct Rect {
    pub width: f64,
    pub height: f64,
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    fn name(&self) -> &str {
        "circle"
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn name(&self) -> &str {
        "rectangle"
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
    fn name(&self) -> &str {
        "triangle"
    }
}

// ---------------------------------------------------------------------------
// Approach 1: dyn Trait — dynamic dispatch via vtable
//
// `Box<dyn Shape>` is a fat pointer: one pointer to the heap-allocated value,
// one pointer to the vtable. The vtable stores function pointers for each
// trait method. This allows a Vec of mixed concrete types — the canonical
// use case for `dyn Trait`.
// ---------------------------------------------------------------------------

pub fn total_area_dyn(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

pub fn largest_dyn(shapes: &[Box<dyn Shape>]) -> Option<&dyn Shape> {
    shapes
        .iter()
        .max_by(|a, b| {
            a.area()
                .partial_cmp(&b.area())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|s| s.as_ref())
}

// ---------------------------------------------------------------------------
// Approach 2: impl Trait / generics — static dispatch (zero-cost)
//
// Monomorphized per concrete type at compile time: no vtable, no heap boxing.
// All elements in a slice must share the same concrete type.
// ---------------------------------------------------------------------------

pub fn total_area_static<S: Shape>(shapes: &[S]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// ---------------------------------------------------------------------------
// Approach 3: Enum dispatch — closed set, no allocation, exhaustive matching
//
// The compiler sees every variant at compile time. No fat pointers, no heap.
// The compiler can inline match arms and avoid any indirection. The trade-off:
// you must own every variant; third-party types cannot be added.
// ---------------------------------------------------------------------------

pub enum AnyShape {
    Circle(Circle),
    Rect(Rect),
    Triangle(Triangle),
}

impl AnyShape {
    pub fn area(&self) -> f64 {
        match self {
            AnyShape::Circle(c) => c.area(),
            AnyShape::Rect(r) => r.area(),
            AnyShape::Triangle(t) => t.area(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            AnyShape::Circle(c) => c.name(),
            AnyShape::Rect(r) => r.name(),
            AnyShape::Triangle(t) => t.name(),
        }
    }
}

pub fn total_area_enum(shapes: &[AnyShape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn test_individual_shape_areas() {
        assert!(approx_eq(Circle { radius: 1.0 }.area(), PI));
        assert!(approx_eq(
            Rect {
                width: 3.0,
                height: 4.0
            }
            .area(),
            12.0
        ));
        assert!(approx_eq(
            Triangle {
                base: 6.0,
                height: 4.0
            }
            .area(),
            12.0
        ));
    }

    #[test]
    fn test_dyn_heterogeneous_collection() {
        // This is the defining use case for dyn Trait: mixed concrete types in one Vec.
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle { radius: 1.0 }),
            Box::new(Rect {
                width: 2.0,
                height: 3.0,
            }),
            Box::new(Triangle {
                base: 4.0,
                height: 2.0,
            }),
        ];
        // circle: PI, rect: 6.0, triangle: 4.0
        assert!(approx_eq(total_area_dyn(&shapes), PI + 6.0 + 4.0));
    }

    #[test]
    fn test_dyn_empty_collection() {
        let shapes: Vec<Box<dyn Shape>> = vec![];
        assert!(approx_eq(total_area_dyn(&shapes), 0.0));
    }

    #[test]
    fn test_largest_dyn_picks_correct_shape() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle { radius: 1.0 }),
            Box::new(Rect {
                width: 10.0,
                height: 10.0,
            }), // area 100 — largest
            Box::new(Triangle {
                base: 2.0,
                height: 2.0,
            }),
        ];
        assert_eq!(largest_dyn(&shapes).unwrap().name(), "rectangle");
    }

    #[test]
    fn test_largest_dyn_single_element() {
        let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle { radius: 3.0 })];
        assert_eq!(largest_dyn(&shapes).unwrap().name(), "circle");
    }

    #[test]
    fn test_largest_dyn_empty_returns_none() {
        let shapes: Vec<Box<dyn Shape>> = vec![];
        assert!(largest_dyn(&shapes).is_none());
    }

    #[test]
    fn test_static_dispatch_homogeneous_slice() {
        // static dispatch: all elements must be the same type
        let circles = [Circle { radius: 1.0 }, Circle { radius: 2.0 }];
        // PI*1^2 + PI*2^2 = PI + 4*PI = 5*PI
        assert!(approx_eq(total_area_static(&circles), 5.0 * PI));
    }

    #[test]
    fn test_enum_dispatch_total_area() {
        let shapes = vec![
            AnyShape::Circle(Circle { radius: 1.0 }),
            AnyShape::Rect(Rect {
                width: 3.0,
                height: 4.0,
            }),
            AnyShape::Triangle(Triangle {
                base: 6.0,
                height: 4.0,
            }),
        ];
        assert!(approx_eq(total_area_enum(&shapes), PI + 12.0 + 12.0));
    }

    #[test]
    fn test_enum_dispatch_names() {
        let shapes = vec![
            AnyShape::Circle(Circle { radius: 1.0 }),
            AnyShape::Rect(Rect {
                width: 1.0,
                height: 1.0,
            }),
            AnyShape::Triangle(Triangle {
                base: 1.0,
                height: 1.0,
            }),
        ];
        let names: Vec<&str> = shapes.iter().map(|s| s.name()).collect();
        assert_eq!(names, vec!["circle", "rectangle", "triangle"]);
    }

    #[test]
    fn test_all_three_strategies_agree() {
        // dyn Trait, static dispatch, and enum must all produce the same total
        // for the same logical shapes.
        let dyn_shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle { radius: 2.0 }),
            Box::new(Rect {
                width: 3.0,
                height: 5.0,
            }),
        ];
        let dyn_total = total_area_dyn(&dyn_shapes);

        let static_circles = [Circle { radius: 2.0 }];
        let static_rects = [Rect {
            width: 3.0,
            height: 5.0,
        }];
        let static_total = total_area_static(&static_circles) + total_area_static(&static_rects);

        let enum_shapes = vec![
            AnyShape::Circle(Circle { radius: 2.0 }),
            AnyShape::Rect(Rect {
                width: 3.0,
                height: 5.0,
            }),
        ];
        let enum_total = total_area_enum(&enum_shapes);

        assert!(approx_eq(dyn_total, static_total));
        assert!(approx_eq(dyn_total, enum_total));
    }
}
