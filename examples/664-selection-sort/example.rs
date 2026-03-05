// Selection Sort in Rust

fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let min_idx = (i..arr.len()).min_by_key(|&j| &arr[j]).unwrap();
        arr.swap(i, min_idx);
    }
}

fn main() {
    let mut arr = vec![64, 25, 12, 22, 11];
    println!("Before: {:?}", arr);
    selection_sort(&mut arr);
    println!("After:  {:?}", arr);
}
