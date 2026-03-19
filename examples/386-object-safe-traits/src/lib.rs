#![allow(clippy::all)]
//! Object Safety Rules

pub trait Drawable {
    fn draw(&self) -> String;
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("Circle(r={})", self.radius)
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> String {
        format!("Rectangle({}x{})", self.width, self.height)
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub fn total_area(shapes: &[Box<dyn Drawable>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let c = Circle { radius: 1.0 };
        assert!((c.area() - std::f64::consts::PI).abs() < 0.001);
    }
    #[test]
    fn test_rect_area() {
        let r = Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert_eq!(r.area(), 12.0);
    }
    #[test]
    fn test_total_area() {
        let shapes: Vec<Box<dyn Drawable>> = vec![
            Box::new(Circle { radius: 1.0 }),
            Box::new(Rectangle {
                width: 2.0,
                height: 3.0,
            }),
        ];
        let total = total_area(&shapes);
        assert!(total > 9.0); // PI + 6
    }
    #[test]
    fn test_draw() {
        let c = Circle { radius: 5.0 };
        assert!(c.draw().contains("Circle"));
    }
}
