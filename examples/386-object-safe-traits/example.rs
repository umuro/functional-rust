// Object safety rules in Rust
use std::fmt;

// Object-SAFE trait: no Self returns, no generics
trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
    // This method is excluded from vtable with where Self: Sized
    fn clone_box(&self) -> Box<dyn Drawable> where Self: Sized + Clone {
        Box::new(self.clone())
    }
}

// NOT object safe if it had:
// fn clone_self(&self) -> Self;  // returns Self
// fn map<T, F: Fn(f64) -> T>(&self, f: F) -> T;  // generic method

#[derive(Clone)]
struct Circle { radius: f64 }
#[derive(Clone)]
struct Rectangle { width: f64, height: f64 }

impl Drawable for Circle {
    fn draw(&self) { println!("Circle(r={})", self.radius); }
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

impl Drawable for Rectangle {
    fn draw(&self) { println!("Rectangle({}x{})", self.width, self.height); }
    fn area(&self) -> f64 { self.width * self.height }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Circle({})", self.radius) }
}

fn total_area(shapes: &[Box<dyn Drawable>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 4.0, height: 6.0 }),
        Box::new(Circle { radius: 2.0 }),
    ];

    for s in &shapes {
        s.draw();
        println!("  area = {:.2}", s.area());
    }
    println!("Total area: {:.2}", total_area(&shapes));

    // Demonstrating object safety check at compile time
    // The next line would fail to compile:
    // let _: &dyn fmt::Display = &Circle { radius: 1.0 }; // Display IS object safe
    println!("Display is object safe: {:?}",
        format!("{}", Circle { radius: 3.0 }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let c = Circle { radius: 1.0 };
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-9);
        let r = Rectangle { width: 3.0, height: 4.0 };
        assert_eq!(r.area(), 12.0);
    }

    #[test]
    fn test_dyn_dispatch() {
        let shapes: Vec<Box<dyn Drawable>> = vec![
            Box::new(Rectangle { width: 2.0, height: 3.0 }),
        ];
        assert_eq!(total_area(&shapes), 6.0);
    }
}
