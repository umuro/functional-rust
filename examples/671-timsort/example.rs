// Timsort in Rust (simplified)
fn main() {
    // Rust's standard sort is based on timsort-like algorithm
    let mut arr = vec![5, 2, 8, 1, 9, 3];
    arr.sort(); // Uses pdqsort (pattern-defeating quicksort)
    println!("Sorted: {:?}", arr);
    
    // Stable sort (closer to timsort)
    let mut arr = vec![(3, 'a'), (1, 'b'), (3, 'c')];
    arr.sort_by_key(|x| x.0);
    println!("Stable sorted: {:?}", arr);
}
