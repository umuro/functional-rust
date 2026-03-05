// Quick Sort in Rust

fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    let pivot = partition(arr);
    quick_sort(&mut arr[..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let n = arr.len();
    let mut i = 0;
    for j in 0..n - 1 {
        if arr[j] <= arr[n - 1] { arr.swap(i, j); i += 1; }
    }
    arr.swap(i, n - 1);
    i
}

fn main() {
    let mut arr = vec![10, 7, 8, 9, 1, 5];
    println!("Before: {:?}", arr);
    quick_sort(&mut arr);
    println!("After:  {:?}", arr);
}
