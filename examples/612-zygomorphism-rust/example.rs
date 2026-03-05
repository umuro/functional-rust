// Zygomorphism: two algebras computed simultaneously in one pass

// Generic zygo over slices
fn zygo<A, R1, R2>(
    xs: &[A],
    init1: R1,
    init2: R2,
    step: impl Fn(R1, R2, &A) -> (R1, R2),
) -> (R1, R2) {
    xs.iter().fold((init1, init2), |(r1,r2), a| step(r1, r2, a))
}

// Mean and variance in one pass (Welford's / naive)
fn mean_variance(xs: &[f64]) -> (f64, f64) {
    let n = xs.len() as f64;
    let (sum, sum_sq) = zygo(xs, 0.0_f64, 0.0_f64, |s, sq, &x| (s+x, sq+x*x));
    let mean = sum / n;
    let variance = sum_sq / n - mean*mean;
    (mean, variance)
}

// Count even and odd simultaneously
fn count_even_odd(xs: &[i32]) -> (usize, usize) {
    zygo(xs, 0usize, 0usize, |evens, odds, &x| {
        if x % 2 == 0 { (evens+1, odds) } else { (evens, odds+1) }
    })
}

// Min and max simultaneously
fn min_max(xs: &[i32]) -> Option<(i32, i32)> {
    if xs.is_empty() { return None; }
    let (mn, mx) = zygo(&xs[1..], xs[0], xs[0], |mn, mx, &x| (mn.min(x), mx.max(x)));
    Some((mn, mx))
}

// Zip with index and running sum
fn indexed_running_sum(xs: &[i32]) -> Vec<(usize, i32)> {
    let (result, _) = zygo(xs, Vec::new(), 0i32, |mut v, sum, &x| {
        let new_sum = sum + x;
        v.push((v.len(), new_sum));
        (v, new_sum)
    });
    result
}

fn main() {
    let xs = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let (mean, var) = mean_variance(&xs);
    println!("mean={:.2} variance={:.2} stddev={:.2}", mean, var, var.sqrt());

    let nums = [1,2,3,4,5,6,7,8,9,10];
    let (evens, odds) = count_even_odd(&nums);
    println!("evens={} odds={}", evens, odds);

    println!("min_max: {:?}", min_max(&nums));
    println!("indexed: {:?}", indexed_running_sum(&[1,2,3,4]));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_mean_var() {
        let (m,v) = mean_variance(&[2.0,2.0,2.0]);
        assert!((m-2.0).abs() < 1e-10);
        assert!(v.abs() < 1e-10);
    }
    #[test] fn test_even_odd() { assert_eq!(count_even_odd(&[1,2,3,4,5]), (2,3)); }
    #[test] fn test_min_max()  { assert_eq!(min_max(&[3,1,4,1,5]), Some((1,5))); }
}
