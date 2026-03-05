// Counting Sort in Rust

fn counting_sort(arr: &mut [usize]) {
    if arr.is_empty() { return; }
    let max = *arr.iter().max().unwrap();
    let mut count = vec![0; max + 1];
    for &x in arr.iter() { count[x] += 1; }
    
    let mut i = 0;
    for (val, &cnt) in count.iter().enumerate() {
        for _ in 0..cnt { arr[i] = val; i += 1; }
    }
}

fn main() {
    let mut arr = vec![4, 2, 2, 8, 3, 3, 1];
    println!("Before: {:?}", arr);
    counting_sort(&mut arr);
    println!("After:  {:?}", arr);
}
