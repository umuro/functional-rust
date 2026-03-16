// ---------------------------------------------------------------------------
// Solution 1: Closure-based — direct mapping of OCaml first-class modules
// ---------------------------------------------------------------------------

pub fn concat_with<T>(
    empty: T,
    combine: impl Fn(T, T) -> T,
    items: impl IntoIterator<Item = T>,
) -> T {
    items.into_iter().fold(empty, combine)
}

// ---------------------------------------------------------------------------
// Solution 2: Idiomatic Rust — trait-based, zero-cost abstraction
// ---------------------------------------------------------------------------

pub trait Monoid: Sized {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

pub fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), M::combine)
}

// ---------------------------------------------------------------------------
// Solution 3: std::iter::Sum / Product for numeric monoids
// ---------------------------------------------------------------------------

pub fn sum_all<T: std::iter::Sum>(items: impl IntoIterator<Item = T>) -> T {
    items.into_iter().sum()
}

pub fn product_all<T: std::iter::Product>(items: impl IntoIterator<Item = T>) -> T {
    items.into_iter().product()
}

// ---------------------------------------------------------------------------
// Newtype wrappers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sum(pub i32);
impl Monoid for Sum {
    fn empty() -> Self { Sum(0) }
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Product(pub i32);
impl Monoid for Product {
    fn empty() -> Self { Product(1) }
    fn combine(self, other: Self) -> Self { Product(self.0 * other.0) }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Concat(pub String);
impl Monoid for Concat {
    fn empty() -> Self { Concat(String::new()) }
    fn combine(self, other: Self) -> Self { Concat(self.0 + &other.0) }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct All(pub bool);
impl Monoid for All {
    fn empty() -> Self { All(true) }
    fn combine(self, other: Self) -> Self { All(self.0 && other.0) }
}

fn main() {
    // Solution 1: closure-based (closest to OCaml's first-class modules)
    println!("=== Closure-based (direct OCaml analogy) ===");
    println!("sum:     {}", concat_with(0, |a, b| a + b, [1, 2, 3, 4, 5]));
    println!("product: {}", concat_with(1, |a, b| a * b, [1, 2, 3, 4, 5]));
    println!(
        "concat:  {}",
        concat_with(
            String::new(),
            |a, b| a + &b,
            ["hello".to_owned(), " ".to_owned(), "world".to_owned()]
        )
    );
    println!("all:     {}", concat_with(true, |a, b| a && b, [true, true, false]));

    // Solution 2: trait-based
    println!("\n=== Trait-based (idiomatic Rust) ===");
    println!("sum:     {:?}", concat_all([Sum(1), Sum(2), Sum(3), Sum(4), Sum(5)]));
    println!("product: {:?}", concat_all([Product(1), Product(2), Product(3), Product(4), Product(5)]));
    println!(
        "concat:  {:?}",
        concat_all([Concat("hello".into()), Concat(" ".into()), Concat("world".into())])
    );
    println!("all:     {:?}", concat_all([All(true), All(true), All(false)]));

    // Solution 3: stdlib
    println!("\n=== std::iter::Sum / Product ===");
    println!("sum:     {}", sum_all([1, 2, 3, 4, 5]));
    println!("product: {}", product_all([1, 2, 3, 4, 5]));
}

/* Output:
   === Closure-based (direct OCaml analogy) ===
   sum:     15
   product: 120
   concat:  hello world
   all:     false

   === Trait-based (idiomatic Rust) ===
   sum:     Sum(15)
   product: Product(120)
   concat:  Concat("hello world")
   all:     All(false)

   === std::iter::Sum / Product ===
   sum:     15
   product: 120
*/
