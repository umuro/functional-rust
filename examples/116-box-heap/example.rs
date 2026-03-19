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

pub fn compute(expr: &Expr) -> i32 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => compute(a) + compute(b),
        Expr::Mul(a, b) => compute(a) * compute(b),
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

fn main() {
    // Approach 1: Box wraps a heap-allocated Vec
    let total = sum_boxed_squares(5);
    println!("sum of squares [0..5) = {total}"); // 0+1+4+9+16 = 30

    // Approach 2: Recursive expression tree
    let expr = add(num(1), mul(num(2), num(3)));
    println!("1 + 2*3 = {}", compute(&expr));

    let expr2 = mul(add(num(1), num(2)), add(num(3), num(4)));
    println!("(1+2) * (3+4) = {}", compute(&expr2));

    // Approach 3: Box<dyn Shape> heterogeneous collection
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle { width: 4.0, height: 5.0 }),
        Box::new(Circle { radius: 3.0 }),
    ];
    println!("total area = {:.4}", total_area(&shapes));
}

/* Output:
   sum of squares [0..5) = 30
   1 + 2*3 = 7
   (1+2) * (3+4) = 21
   total area = 48.2743
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxed_value_dereferences_transparently() {
        let b: Box<i32> = Box::new(42);
        assert_eq!(*b, 42);
        assert_eq!(*b + 1, 43);
    }

    #[test]
    fn test_sum_boxed_squares() {
        assert_eq!(sum_boxed_squares(5), 30);
        assert_eq!(sum_boxed_squares(0), 0);
        assert_eq!(sum_boxed_squares(1), 0);
    }

    #[test]
    fn test_recursive_expr_compute() {
        let expr = add(num(1), mul(num(2), num(3)));
        assert_eq!(compute(&expr), 7);

        let expr2 = mul(add(num(1), num(2)), add(num(3), num(4)));
        assert_eq!(compute(&expr2), 21);
    }

    #[test]
    fn test_trait_objects_in_vec() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Rectangle { width: 4.0, height: 5.0 }),
            Box::new(Circle { radius: 0.0 }),
        ];
        let total = total_area(&shapes);
        assert!((total - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_box_size_is_pointer_sized() {
        assert_eq!(
            std::mem::size_of::<Box<[i32; 1000]>>(),
            std::mem::size_of::<*const u8>()
        );
    }

    #[test]
    fn test_nested_expr_num_only() {
        assert_eq!(compute(&num(0)), 0);
        assert_eq!(compute(&num(-5)), -5);
    }
}
