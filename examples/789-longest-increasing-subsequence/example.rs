// LIS — patience sorting O(n log n)
// Rust style: iterative, Vec::partition_point for binary search

fn lis_length(arr: &[i64]) -> usize {
    let mut tails: Vec<i64> = Vec::new();
    for &x in arr {
        // partition_point finds first index where tails[i] >= x
        let pos = tails.partition_point(|&t| t < x);
        if pos == tails.len() {
            tails.push(x);
        } else {
            tails[pos] = x;
        }
    }
    tails.len()
}

fn lis_reconstruct(arr: &[i64]) -> Vec<i64> {
    let n = arr.len();
    if n == 0 {
        return vec![];
    }
    let mut tails: Vec<i64> = Vec::new();
    let mut idx: Vec<usize> = Vec::new();   // tails slot -> arr index
    let mut pred: Vec<Option<usize>> = vec![None; n]; // predecessor

    for (i, &x) in arr.iter().enumerate() {
        let pos = tails.partition_point(|&t| t < x);
        if pos == tails.len() {
            tails.push(x);
            idx.push(i);
        } else {
            tails[pos] = x;
            idx[pos] = i;
        }
        pred[i] = if pos > 0 { Some(idx[pos - 1]) } else { None };
    }

    // Reconstruct from last idx
    let mut result = Vec::with_capacity(tails.len());
    let mut k = idx[tails.len() - 1];
    loop {
        result.push(arr[k]);
        match pred[k] {
            Some(p) => k = p,
            None => break,
        }
    }
    result.reverse();
    result
}

fn main() {
    let arr = vec![10i64, 9, 2, 5, 3, 7, 101, 18];
    println!("Array:        {:?}", arr);
    println!("LIS length:   {}", lis_length(&arr));
    println!("LIS sequence: {:?}", lis_reconstruct(&arr));

    // Edge cases
    println!("Empty:        {}", lis_length(&[]));
    println!("[3,3,3]:      {}", lis_length(&[3, 3, 3]));
    println!("[1..5]:       {}", lis_length(&[1, 2, 3, 4, 5]));
    println!("[5..1]:       {}", lis_length(&[5, 4, 3, 2, 1]));
}
