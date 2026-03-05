// Type Alias Impl Trait (TAIT) in Rust
// Note: TAIT requires nightly or Rust 1.75+ for full support.
// Here we show the stable approximation and the concept.

// Stable: returning impl Trait from functions
fn make_counter(start: i32, end: i32) -> impl Iterator<Item = i32> {
    start..end
}

fn make_even_filter(v: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().filter(|x| x % 2 == 0)
}

// With a named type alias (stable, but requires boxing or concrete type)
type BoxedIter<T> = Box<dyn Iterator<Item = T>>;

fn range_boxed(start: i32, end: i32) -> BoxedIter<i32> {
    Box::new(start..end)
}

fn evens_boxed(v: Vec<i32>) -> BoxedIter<i32> {
    Box::new(v.into_iter().filter(|x| x % 2 == 0))
}

// TAIT concept: type Squares = impl Iterator<Item=i64>
// (requires nightly; shown conceptually)
fn squares(n: u32) -> impl Iterator<Item = i64> {
    (1..=n).map(|x| (x as i64) * (x as i64))
}

fn main() {
    let counter = make_counter(1, 6);
    println!("Counter: {:?}", counter.collect::<Vec<_>>());

    let evens = make_even_filter(vec![1, 2, 3, 4, 5, 6]);
    println!("Evens: {:?}", evens.collect::<Vec<_>>());

    let boxed: BoxedIter<i32> = range_boxed(10, 15);
    println!("Boxed range: {:?}", boxed.collect::<Vec<_>>());

    println!("Squares: {:?}", squares(5).collect::<Vec<_>>());

    // Chain iterators using the same opaque type concept
    let chained: Vec<i32> = make_counter(1, 4)
        .chain(make_counter(10, 13))
        .collect();
    println!("Chained: {:?}", chained);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let v: Vec<i32> = make_counter(0, 5).collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_even_filter() {
        let v: Vec<i32> = make_even_filter(vec![1,2,3,4,5,6]).collect();
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn test_squares() {
        let v: Vec<i64> = squares(4).collect();
        assert_eq!(v, vec![1, 4, 9, 16]);
    }
}
