// Semigroup Laws in Rust

trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Sum(i32);

impl Semigroup for Sum {
    fn combine(self, other: Self) -> Self {
        Sum(self.0 + other.0)
    }
}

fn main() {
    let a = Sum(1);
    let b = Sum(2);
    let c = Sum(3);
    
    // Associativity: (a <> b) <> c = a <> (b <> c)
    let left = a.combine(b).combine(c);
    let right = a.combine(b.combine(c));
    
    println!("Left:  {:?}", left);
    println!("Right: {:?}", right);
    println!("Associativity holds: {}", left == right);
}
