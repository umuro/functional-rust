// Radix Sort in Rust (LSD)

fn radix_sort(arr: &mut [u32]) {
    if arr.is_empty() { return; }
    let max = *arr.iter().max().unwrap();
    let mut exp = 1u32;
    while max / exp > 0 {
        let mut output = vec![0; arr.len()];
        let mut count = [0usize; 10];
        for &x in arr.iter() { count[((x / exp) % 10) as usize] += 1; }
        for i in 1..10 { count[i] += count[i - 1]; }
        for &x in arr.iter().rev() {
            let d = ((x / exp) % 10) as usize;
            count[d] -= 1;
            output[count[d]] = x;
        }
        arr.copy_from_slice(&output);
        exp *= 10;
    }
}

fn main() {
    let mut arr = vec![170, 45, 75, 90, 802, 24, 2, 66];
    println!("Before: {:?}", arr);
    radix_sort(&mut arr);
    println!("After:  {:?}", arr);
}
