//! Static vs Dynamic Trait Dispatch

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub r: f64,
}
pub struct Square {
    pub side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// Static dispatch (monomorphization) - compile-time
pub fn total_area_static<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// Dynamic dispatch (vtable) - runtime
pub fn total_area_dynamic(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// Trade-offs:
// Static: faster (no vtable), larger binary (code duplication)
// Dynamic: smaller binary, slower (indirection), heterogeneous collections

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_circles() {
        let circles = vec![Circle { r: 1.0 }, Circle { r: 2.0 }];
        let area = total_area_static(&circles);
        assert!((area - 5.0 * std::f64::consts::PI).abs() < 0.001);
    }
    #[test]
    fn test_dynamic_mixed() {
        let shapes: Vec<Box<dyn Shape>> =
            vec![Box::new(Circle { r: 1.0 }), Box::new(Square { side: 2.0 })];
        let area = total_area_dynamic(&shapes);
        assert!((area - (std::f64::consts::PI + 4.0)).abs() < 0.001);
    }
    #[test]
    fn test_square_area() {
        let s = Square { side: 3.0 };
        assert_eq!(s.area(), 9.0);
    }
    #[test]
    fn test_circle_area() {
        let c = Circle { r: 1.0 };
        assert!((c.area() - std::f64::consts::PI).abs() < 0.001);
    }
}
