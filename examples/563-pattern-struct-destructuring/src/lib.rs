//! Struct Destructuring
//!
//! Extracting fields from structs in patterns.

pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub struct Person {
    pub name: String,
    pub age: u32,
}

/// Basic destructuring.
pub fn get_x(p: &Point) -> i32 {
    let Point { x, .. } = p;
    *x
}

/// Full destructuring.
pub fn distance_from_origin(p: &Point) -> f64 {
    let Point { x, y } = p;
    ((*x as f64).powi(2) + (*y as f64).powi(2)).sqrt()
}

/// Destructure with rename.
pub fn describe_person(p: &Person) -> String {
    let Person { name: n, age: a } = p;
    format!("{} is {} years old", n, a)
}

/// Match with destructuring.
pub fn quadrant(p: &Point) -> &'static str {
    match p {
        Point { x: 0, y: 0 } => "origin",
        Point { x, y } if *x > 0 && *y > 0 => "Q1",
        Point { x, y } if *x < 0 && *y > 0 => "Q2",
        Point { x, y } if *x < 0 && *y < 0 => "Q3",
        Point { x, y } if *x > 0 && *y < 0 => "Q4",
        _ => "axis",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_x() {
        let p = Point { x: 5, y: 10 };
        assert_eq!(get_x(&p), 5);
    }

    #[test]
    fn test_distance() {
        let p = Point { x: 3, y: 4 };
        assert!((distance_from_origin(&p) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_describe() {
        let p = Person {
            name: "Alice".into(),
            age: 30,
        };
        assert!(describe_person(&p).contains("Alice"));
    }

    #[test]
    fn test_quadrant() {
        assert_eq!(quadrant(&Point { x: 1, y: 1 }), "Q1");
        assert_eq!(quadrant(&Point { x: 0, y: 0 }), "origin");
    }
}
