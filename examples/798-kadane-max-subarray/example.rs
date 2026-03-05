// Kadane's Algorithm — maximum subarray sum O(n)

fn max_subarray(arr: &[i64]) -> (i64, usize, usize) {
    assert!(!arr.is_empty(), "array must be non-empty");
    let mut best_sum   = arr[0];
    let mut best_start = 0;
    let mut best_end   = 0;
    let mut curr_sum   = arr[0];
    let mut curr_start = 0;

    for (i, &x) in arr.iter().enumerate().skip(1) {
        if x > curr_sum + x {
            curr_sum   = x;
            curr_start = i;
        } else {
            curr_sum  += x;
        }
        if curr_sum > best_sum {
            best_sum   = curr_sum;
            best_start = curr_start;
            best_end   = i;
        }
    }
    (best_sum, best_start, best_end)
}

fn main() {
    let arr = vec![-2i64, 1, -3, 4, -1, 2, 1, -5, 4];
    let (sum, s, e) = max_subarray(&arr);
    println!("Array:             {:?}", arr);
    println!("Max subarray sum:  {sum}  (indices {s}..{e})");
    println!("Subarray:          {:?}", &arr[s..=e]);

    let all_neg = vec![-5i64, -3, -1, -2, -4];
    let (s2, i2, j2) = max_subarray(&all_neg);
    println!("All-negative: sum={s2}, subarray={:?}", &all_neg[i2..=j2]);
}
