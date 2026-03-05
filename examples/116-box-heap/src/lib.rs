// Example 116: Box<T> — Heap Allocation
//
// Box<T> puts data on the heap with single ownership.
// Use for: large data, recursive types, and trait objects (dyn Trait).

// --- Approach 1: Heap-allocating large data ---
//
// The heap holds the array; the stack holds only an 8-byte pointer.
pub fn sum_boxed_squares(n: usize) -> i64 {
    let squares: Box<Vec<i64>> = Box::new((0..n as i64).map(|i| i * i).collect());
    squares.iter().sum()
}

// --- Approach 2: Recursive types require Box for known size ---
//
// Without Box the compiler cannot compute the size of Expr
// (it would be infinitely recursive).  Box breaks the cycle:
// each variant stores a pointer (8 bytes), not the full sub-tree.
#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

pub fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

// Convenience constructors so call-sites stay readable.
pub fn num(n: i32) -> Expr {
    Expr::Num(n)
}
pub fn add(a: Expr, b: Expr) -> Expr {
    Expr::Add(Box::new(a), Box::new(b))
}
pub fn mul(a: Expr, b: Expr) -> Expr {
    Expr::Mul(Box::new(a), Box::new(b))
}

// --- Approach 3: Trait objects — heterogeneous collections ---
//
// Box<dyn Shape> is always pointer-sized regardless of the concrete type,
// letting us store mixed shapes in a single Vec.
pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxed_value_dereferences_transparently() {
        let b: Box<i32> = Box::new(42);
        // Deref coercion: *b behaves like i32
        assert_eq!(*b, 42);
        assert_eq!(*b + 1, 43);
    }

    #[test]
    fn test_sum_boxed_squares() {
        // 0² + 1² + 2² + 3² + 4² = 0+1+4+9+16 = 30
        assert_eq!(sum_boxed_squares(5), 30);
        assert_eq!(sum_boxed_squares(0), 0);
        assert_eq!(sum_boxed_squares(1), 0); // only 0²
    }

    #[test]
    fn test_recursive_expr_eval() {
        // 1 + 2 * 3  (multiplication binds tighter in OCaml source)
        let expr = add(num(1), mul(num(2), num(3)));
        assert_eq!(eval(&expr), 7);

        // (1 + 2) * (3 + 4) = 3 * 7 = 21
        let expr2 = mul(add(num(1), num(2)), add(num(3), num(4)));
        assert_eq!(eval(&expr2), 21);
    }

    #[test]
    fn test_trait_objects_in_vec() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Rectangle {
                width: 4.0,
                height: 5.0,
            }),
            Box::new(Circle { radius: 0.0 }),
        ];
        let total = total_area(&shapes);
        assert!((total - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_box_size_is_pointer_sized() {
        // Box<T> is always one pointer wide, regardless of T's size.
        assert_eq!(
            std::mem::size_of::<Box<[i32; 1000]>>(),
            std::mem::size_of::<*const u8>()
        );
    }

    #[test]
    fn test_nested_expr_num_only() {
        assert_eq!(eval(&num(0)), 0);
        assert_eq!(eval(&num(-5)), -5);
    }
}
