// PDQsort is Rust's sort_unstable
fn main() {
    let mut arr = vec![5, 2, 8, 1, 9, 3];
    arr.sort_unstable();
    println!("Sorted: {:?}", arr);
}
