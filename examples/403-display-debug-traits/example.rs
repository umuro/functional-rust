// Display, Debug, and formatting in Rust
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
}

// Custom Display (user-facing)
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "red"),
            Color::Green => write!(f, "green"),
            Color::Blue => write!(f, "blue"),
            Color::Rgb(r, g, b) => write!(f, "rgb({},{},{})", r, g, b),
        }
    }
}

// Lower-hex formatter
impl fmt::LowerHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (r, g, b) = match self {
            Color::Red => (255, 0, 0),
            Color::Green => (0, 255, 0),
            Color::Blue => (0, 0, 255),
            Color::Rgb(r, g, b) => (*r as u32, *g as u32, *b as u32),
        };
        write!(f, "#{:02x}{:02x}{:02x}", r, g, b)
    }
}

#[derive(Debug, Clone)]
struct Point { x: f64, y: f64 }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Respect precision from format string
        match f.precision() {
            Some(p) => write!(f, "({:.prec$}, {:.prec$})", self.x, self.y, prec = p),
            None => write!(f, "({:.2}, {:.2})", self.x, self.y),
        }
    }
}

struct Matrix([[f64; 2]; 2]);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {:.1} {:.1} ]
[ {:.1} {:.1} ]",
            self.0[0][0], self.0[0][1], self.0[1][0], self.0[1][1])
    }
}

fn main() {
    let colors = [Color::Red, Color::Green, Color::Rgb(128, 64, 32)];
    for c in &colors {
        println!("Display: {}  Debug: {:?}  Hex: {:x}", c, c, c);
    }

    let p = Point { x: 3.14159, y: 2.71828 };
    println!("Default: {}", p);
    println!("2 decimals: {:.2}", p);
    println!("4 decimals: {:.4}", p);
    println!("Debug: {:?}", p);
    println!("Pretty debug: {:#?}", p);

    let m = Matrix([[1.0, 2.0], [3.0, 4.0]]);
    println!("Matrix:
{}", m);

    // Format specifiers
    println!("{:>10}", "right");
    println!("{:<10}", "left");
    println!("{:^10}", "center");
    println!("{:0>5}", 42);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Color::Red), "red");
        assert_eq!(format!("{}", Color::Rgb(10, 20, 30)), "rgb(10,20,30)");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Color::Blue), "Blue");
    }

    #[test]
    fn test_hex_format() {
        assert_eq!(format!("{:x}", Color::Red), "#ff0000");
    }

    #[test]
    fn test_point_precision() {
        let p = Point { x: 1.23456, y: 7.89012 };
        assert_eq!(format!("{:.1}", p), "(1.2, 7.9)");
    }
}
