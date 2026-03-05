//! Demo of custom iterator implementations

use example_281_custom_iterator::{Fibonacci, Squares, SteppedRange};

fn main() {
    // Squares iterator — uses all iterator adapters for free
    let squares: Vec<u32> = Squares::new(6).collect();
    println!("Squares: {:?}", squares);

    let sum_of_squares: u32 = Squares::new(6).sum();
    println!("Sum of squares: {}", sum_of_squares);

    let big_squares: Vec<u32> = Squares::new(10).filter(|&x| x > 10).collect();
    println!("Squares > 10: {:?}", big_squares);

    // Fibonacci — infinite iterator, use take()
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("First 10 Fibonacci: {:?}", fibs);

    let fib_sum: u64 = Fibonacci::new().take(10).sum();
    println!("Sum of first 10 Fibonacci: {}", fib_sum);

    // Stepped range with custom step
    let evens: Vec<i32> = SteppedRange::new(0, 20, 2).collect();
    println!("Evens: {:?}", evens);

    // Zip two custom iterators
    let zipped: Vec<(u32, u64)> = Squares::new(5).zip(Fibonacci::new()).collect();
    println!("Squares zip Fibonacci: {:?}", zipped);
}
