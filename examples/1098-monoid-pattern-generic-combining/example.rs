/// A monoid is a type with an associative binary operation and an identity element.
///
/// This mirrors OCaml's `module type MONOID = sig type t val empty : t val combine : t -> t -> t end`
/// using Rust's trait system instead of first-class modules.
pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

// --- Solution 1: Idiomatic Rust — iterator fold with trait bound ---
// Uses Iterator::fold, the direct analogue of OCaml's List.fold_left
pub fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), M::combine)
}

// --- Solution 2: Recursive — closer to OCaml's fold_left unrolled ---
// Explicit recursion over a slice, mirroring OCaml pattern matching on lists.
// Requires Clone because we need to produce owned values from references.
pub fn concat_all_recursive<M: Monoid + Clone>(items: &[M]) -> M {
    match items {
        [] => M::empty(),
        [x] => x.clone(),
        [x, rest @ ..] => M::combine(x.clone(), concat_all_recursive(rest)),
    }
}

// --- Solution 3: reduce-based — avoids calling empty() when list is non-empty ---
// Uses Iterator::reduce, returning None for empty iterators.
pub fn concat_all_reduce<M: Monoid>(items: impl IntoIterator<Item = M>) -> Option<M> {
    items.into_iter().reduce(M::combine)
}

// --- Monoid instances ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sum(pub i64);

impl Monoid for Sum {
    fn empty() -> Self {
        Sum(0)
    }
    fn combine(self, other: Self) -> Self {
        Sum(self.0 + other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Product(pub i64);

impl Monoid for Product {
    fn empty() -> Self {
        Product(1)
    }
    fn combine(self, other: Self) -> Self {
        Product(self.0 * other.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Concat(pub String);

impl Monoid for Concat {
    fn empty() -> Self {
        Concat(String::new())
    }
    fn combine(self, other: Self) -> Self {
        Concat(self.0 + &other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct All(pub bool);

impl Monoid for All {
    fn empty() -> Self {
        All(true)
    }
    fn combine(self, other: Self) -> Self {
        All(self.0 && other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Any(pub bool);

impl Monoid for Any {
    fn empty() -> Self {
        Any(false)
    }
    fn combine(self, other: Self) -> Self {
        Any(self.0 || other.0)
    }
}

fn main() {
    let sum = concat_all(vec![Sum(1), Sum(2), Sum(3), Sum(4), Sum(5)]);
    println!("sum: {}", sum.0);

    let product = concat_all(vec![Product(1), Product(2), Product(3), Product(4), Product(5)]);
    println!("product: {}", product.0);

    let concat = concat_all(vec![
        Concat("hello".into()),
        Concat(" ".into()),
        Concat("world".into()),
    ]);
    println!("concat: {}", concat.0);

    let all = concat_all(vec![All(true), All(true), All(false)]);
    println!("all: {}", all.0);

    // Recursive variant
    let sum_rec = concat_all_recursive(&[Sum(1), Sum(2), Sum(3), Sum(4), Sum(5)]);
    println!("sum (recursive): {}", sum_rec.0);

    // Reduce variant — returns None for empty
    let empty_sum = concat_all_reduce::<Sum>(vec![]);
    println!("reduce([]): {:?}", empty_sum);

    let some_sum = concat_all_reduce(vec![Sum(10), Sum(20)]);
    println!("reduce([10,20]): {:?}", some_sum);
}

/* Output:
   sum: 15
   product: 120
   concat: hello world
   all: false
   sum (recursive): 15
   reduce([]): None
   reduce([10,20]): Some(Sum(30))
*/
