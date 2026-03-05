// Monoid Laws in Rust

trait Monoid: Sized {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Sum(i32);

impl Monoid for Sum {
    fn empty() -> Self { Sum(0) }
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}

fn mconcat<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), |acc, x| acc.combine(x))
}

fn main() {
    let x = Sum(42);
    
    // Left identity
    let left_id = Sum::empty().combine(x) == x;
    println!("Left identity: {}", left_id);
    
    // Right identity  
    let right_id = x.combine(Sum::empty()) == x;
    println!("Right identity: {}", right_id);
    
    // mconcat
    let result = mconcat(vec![Sum(1), Sum(2), Sum(3)]);
    println!("mconcat: {:?}", result);
}
