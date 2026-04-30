use example_089_lazy_sequences::{fibs, from_fn_indexed, naturals, take};

fn main() {
    // Natural numbers — Rust ranges are lazy iterators
    let first_five = take(5, naturals());
    println!("First 5 naturals: {first_five:?}");
    assert_eq!(first_five, [0, 1, 2, 3, 4]);

    // Fibonacci — built with std::iter::successors, fully lazy
    let first_eight_fibs = take(8, fibs());
    println!("First 8 Fibonacci: {first_eight_fibs:?}");
    assert_eq!(first_eight_fibs, [0, 1, 1, 2, 3, 5, 8, 13]);

    // from_fn_indexed — stateful generator terminated by None
    let powers_of_2: Vec<u64> =
        from_fn_indexed(|n| if n >= 10 { None } else { Some(1u64 << n) }).collect();
    println!("Powers of 2 (0..9): {powers_of_2:?}");
    assert_eq!(&powers_of_2[..4], [1, 2, 4, 8]);

    // Chaining lazy operations — nothing is evaluated until collect()
    let evens: Vec<u64> = naturals()
        .filter(|n| n % 2 == 0)
        .take(5)
        .collect();
    println!("First 5 even naturals: {evens:?}");
    assert_eq!(evens, [0, 2, 4, 6, 8]);

    println!("All assertions passed.");
}
