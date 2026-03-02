// Pattern Matching in Rust

// Basic patterns
#[derive(Debug, Clone)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
}

fn describe_color(color: &Color) -> String {
    match color {
        Color::Red => "pure red".to_string(),
        Color::Green => "pure green".to_string(),
        Color::Blue => "pure blue".to_string(),
        Color::RGB(r, g, b) => format!("RGB({}, {}, {})", r, g, b),
    }
}

// Guards
fn classify_number(n: i32) -> &'static str {
    match n {
        x if x < 0 => "negative",
        0 => "zero",
        x if x > 0 && x <= 10 => "small positive",
        _ => "large positive",
    }
}

// Nested patterns
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Point(f64, f64),
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => 3.14159 * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Point(_, _) => 0.0,
    }
}

fn describe_shape(shape: &Shape) -> &'static str {
    match shape {
        Shape::Circle(r) if *r > 10.0 => "large circle",
        Shape::Circle(_) => "small circle",
        Shape::Rectangle(w, h) if w == h => "square",
        Shape::Rectangle(_, _) => "rectangle",
        Shape::Point(0.0, 0.0) => "origin",
        Shape::Point(_, _) => "point",
    }
}

// Tuple patterns
fn swap<T>(tuple: (T, T)) -> (T, T) {
    let (x, y) = tuple;
    (y, x)
}

fn first_two<T: Clone>(lst: &[T]) -> Option<(T, T)> {
    match lst {
        [] | [_] => None,
        [x, y, ..] => Some((x.clone(), y.clone())),
    }
}

// Binding patterns (@ equivalent)
fn duplicate_first<T: Clone>(lst: &[T]) -> Vec<T> {
    match lst {
        [] => vec![],
        [x, rest @ ..] => {
            let mut result = vec![x.clone()];
            result.push(x.clone());
            result.extend_from_slice(rest);
            result
        }
    }
}

// Or patterns
fn is_primary_color(color: &Color) -> bool {
    matches!(color, Color::Red | Color::Green | Color::Blue)
}

fn main() {
    println!("Red: {}", describe_color(&Color::Red));
    println!("RGB: {}", describe_color(&Color::RGB(255, 128, 0)));
    
    println!("Classify -5: {}", classify_number(-5));
    println!("Classify 7: {}", classify_number(7));
    
    let circle = Shape::Circle(15.0);
    println!("Area: {:.2}", area(&circle));
    println!("Shape: {}", describe_shape(&circle));
    
    let rect = Shape::Rectangle(5.0, 5.0);
    println!("Square: {}", describe_shape(&rect));
    
    let swapped = swap((1, 2));
    println!("Swap (1,2): ({}, {})", swapped.0, swapped.1);
    
    println!("Is Red primary? {}", is_primary_color(&Color::Red));
    println!("Is RGB primary? {}", is_primary_color(&Color::RGB(100, 100, 100)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_color() {
        assert_eq!(describe_color(&Color::Red), "pure red");
        assert_eq!(describe_color(&Color::RGB(255, 0, 0)), "RGB(255, 0, 0)");
    }

    #[test]
    fn test_classify_number() {
        assert_eq!(classify_number(-5), "negative");
        assert_eq!(classify_number(0), "zero");
        assert_eq!(classify_number(7), "small positive");
        assert_eq!(classify_number(100), "large positive");
    }

    #[test]
    fn test_area() {
        let circle = Shape::Circle(2.0);
        assert!((area(&circle) - 12.566).abs() < 0.01);
    }

    #[test]
    fn test_is_primary() {
        assert!(is_primary_color(&Color::Red));
        assert!(!is_primary_color(&Color::RGB(100, 100, 100)));
    }
}
