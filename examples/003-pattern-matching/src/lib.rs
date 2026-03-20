#![allow(clippy::all)]
// 003: Pattern Matching
// Tuples, enums, nested patterns, guards

// Approach 1: Simple tuple matching
fn describe_pair(pair: (i32, i32)) -> String {
    match pair {
        (0, 0) => "origin".to_string(),
        (x, 0) => format!("x-axis at {}", x),
        (0, y) => format!("y-axis at {}", y),
        (x, y) => format!("point ({}, {})", x, y),
    }
}

// Approach 2: Enum matching
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

fn shape_name(shape: &Shape) -> &str {
    match shape {
        Shape::Circle(_) => "circle",
        Shape::Rectangle(_, _) => "rectangle",
        Shape::Triangle(_, _, _) => "triangle",
    }
}

// Approach 3: Nested patterns with guards
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

fn classify_number(n: i32) -> &'static str {
    match n {
        n if n < 0 => "negative",
        0 => "zero",
        n if n % 2 == 0 => "positive even",
        _ => "positive odd",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_pair() {
        assert_eq!(describe_pair((0, 0)), "origin");
        assert_eq!(describe_pair((3, 0)), "x-axis at 3");
        assert_eq!(describe_pair((0, 5)), "y-axis at 5");
        assert_eq!(describe_pair((2, 3)), "point (2, 3)");
    }

    #[test]
    fn test_area() {
        assert!((area(&Shape::Circle(1.0)) - std::f64::consts::PI).abs() < 0.001);
        assert_eq!(area(&Shape::Rectangle(3.0, 4.0)), 12.0);
    }

    #[test]
    fn test_shape_name() {
        assert_eq!(shape_name(&Shape::Circle(1.0)), "circle");
    }

    #[test]
    fn test_eval() {
        let e = Expr::Add(
            Box::new(Expr::Num(1)),
            Box::new(Expr::Mul(Box::new(Expr::Num(2)), Box::new(Expr::Num(3)))),
        );
        assert_eq!(eval(&e), 7);
    }

    #[test]
    fn test_classify() {
        assert_eq!(classify_number(-5), "negative");
        assert_eq!(classify_number(0), "zero");
        assert_eq!(classify_number(4), "positive even");
        assert_eq!(classify_number(7), "positive odd");
    }
}
