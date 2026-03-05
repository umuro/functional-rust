// Sorting Algorithms Overview

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] { arr.swap(j, j + 1); }
        }
    }
}

fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    let pivot = partition(arr);
    quick_sort(&mut arr[..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] { arr.swap(i, j); i += 1; }
    }
    arr.swap(i, len - 1);
    i
}

fn main() {
    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Original: {:?}", arr);
    
    let mut bubble = arr.clone();
    bubble_sort(&mut bubble);
    println!("Bubble sorted: {:?}", bubble);
    
    quick_sort(&mut arr);
    println!("Quick sorted: {:?}", arr);
}
