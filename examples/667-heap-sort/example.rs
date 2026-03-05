// Heap Sort in Rust

fn heap_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in (0..n/2).rev() { heapify(arr, n, i); }
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    let (mut largest, left, right) = (i, 2*i+1, 2*i+2);
    if left < n && arr[left] > arr[largest] { largest = left; }
    if right < n && arr[right] > arr[largest] { largest = right; }
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn main() {
    let mut arr = vec![12, 11, 13, 5, 6, 7];
    println!("Before: {:?}", arr);
    heap_sort(&mut arr);
    println!("After:  {:?}", arr);
}
