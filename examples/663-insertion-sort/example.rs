// Insertion Sort in Rust

fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn main() {
    let mut arr = vec![12, 11, 13, 5, 6];
    println!("Before: {:?}", arr);
    insertion_sort(&mut arr);
    println!("After:  {:?}", arr);
}
