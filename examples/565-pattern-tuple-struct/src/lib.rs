//! Tuple Struct Patterns
//!
//! Destructuring tuple structs and newtypes.

pub struct Point(pub i32, pub i32);
pub struct Color(pub u8, pub u8, pub u8);
pub struct Meters(pub f64);
pub struct Seconds(pub f64);

/// Destructure tuple struct.
pub fn get_coords(p: &Point) -> (i32, i32) {
    let Point(x, y) = p;
    (*x, *y)
}

/// Pattern in function params.
pub fn add_points(Point(x1, y1): &Point, Point(x2, y2): &Point) -> Point {
    Point(x1 + x2, y1 + y2)
}

/// Newtype pattern.
pub fn meters_to_feet(Meters(m): Meters) -> f64 {
    m * 3.28084
}

/// Match with tuple struct.
pub fn describe_color(Color(r, g, b): &Color) -> &'static str {
    match (r, g, b) {
        (255, 0, 0) => "red",
        (0, 255, 0) => "green",
        (0, 0, 255) => "blue",
        (0, 0, 0) => "black",
        (255, 255, 255) => "white",
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords() {
        let p = Point(3, 4);
        assert_eq!(get_coords(&p), (3, 4));
    }

    #[test]
    fn test_add_points() {
        let p = add_points(&Point(1, 2), &Point(3, 4));
        assert_eq!((p.0, p.1), (4, 6));
    }

    #[test]
    fn test_meters() {
        let ft = meters_to_feet(Meters(1.0));
        assert!((ft - 3.28084).abs() < 0.001);
    }

    #[test]
    fn test_color() {
        assert_eq!(describe_color(&Color(255, 0, 0)), "red");
        assert_eq!(describe_color(&Color(128, 128, 128)), "other");
    }
}
