// Merge Sort in Rust

fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    
    let (left, right) = (arr[..mid].to_vec(), arr[mid..].to_vec());
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] { arr[k] = left[i].clone(); i += 1; }
        else { arr[k] = right[j].clone(); j += 1; }
        k += 1;
    }
    while i < left.len() { arr[k] = left[i].clone(); i += 1; k += 1; }
    while j < right.len() { arr[k] = right[j].clone(); j += 1; k += 1; }
}

fn main() {
    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    println!("Before: {:?}", arr);
    merge_sort(&mut arr);
    println!("After:  {:?}", arr);
}
