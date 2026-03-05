// Introsort concept - Rust's sort_unstable uses pdqsort which is similar
fn main() {
    let mut arr = vec![5, 2, 8, 1, 9, 3];
    arr.sort_unstable(); // Uses pdqsort (introsort variant)
    println!("Sorted: {:?}", arr);
}
