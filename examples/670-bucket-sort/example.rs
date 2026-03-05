// Bucket Sort in Rust

fn bucket_sort(arr: &mut [f64]) {
    if arr.len() <= 1 { return; }
    let n = arr.len();
    let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); n];
    
    for &x in arr.iter() {
        let idx = (x * n as f64).min((n - 1) as f64) as usize;
        buckets[idx].push(x);
    }
    
    for bucket in &mut buckets {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
    
    let mut i = 0;
    for bucket in buckets {
        for x in bucket { arr[i] = x; i += 1; }
    }
}

fn main() {
    let mut arr = vec![0.42, 0.32, 0.23, 0.52, 0.25, 0.47, 0.51];
    println!("Before: {:?}", arr);
    bucket_sort(&mut arr);
    println!("After:  {:?}", arr);
}
