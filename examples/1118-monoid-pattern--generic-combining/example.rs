// Monoid typeclass pattern using Rust traits.
// OCaml uses first-class modules to pass MONOID implementations.
// Rust uses traits with associated constants/methods and zero-sized marker types.

pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

pub fn concat_all<T: Monoid>(items: impl IntoIterator<Item = T>) -> T {
    items.into_iter().fold(T::empty(), T::combine)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sum(pub i64);
impl Monoid for Sum {
    fn empty() -> Self { Sum(0) }
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Product(pub i64);
impl Monoid for Product {
    fn empty() -> Self { Product(1) }
    fn combine(self, other: Self) -> Self { Product(self.0 * other.0) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Concat(pub String);
impl Monoid for Concat {
    fn empty() -> Self { Concat(String::new()) }
    fn combine(self, other: Self) -> Self { Concat(self.0 + &other.0) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct All(pub bool);
impl Monoid for All {
    fn empty() -> Self { All(true) }
    fn combine(self, other: Self) -> Self { All(self.0 && other.0) }
}

fn main() {
    let sum = concat_all([1, 2, 3, 4, 5].map(Sum));
    println!("sum: {}", sum.0);

    let product = concat_all([1, 2, 3, 4, 5].map(Product));
    println!("product: {}", product.0);

    let words = ["hello", " ", "world"].map(|s| Concat(s.to_owned()));
    println!("concat: {}", concat_all(words).0);

    let all = concat_all([All(true), All(true), All(false)]);
    println!("all: {}", all.0);
}

/* Output:
   sum: 15
   product: 120
   concat: hello world
   all: false
*/
