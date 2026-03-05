// 449. rayon::join — shown with std threads
use std::thread;

fn join<A: Send+'static, B: Send+'static>(f: impl FnOnce()->A+Send+'static, g: impl FnOnce()->B+Send+'static) -> (A,B) {
    let h = thread::spawn(f);
    let b = g();
    (h.join().unwrap(), b)
}

fn merge(a: Vec<i64>, b: Vec<i64>) -> Vec<i64> {
    let mut out = Vec::with_capacity(a.len()+b.len());
    let (mut i,mut j) = (0,0);
    while i<a.len() && j<b.len() { if a[i]<=b[j]{out.push(a[i]);i+=1;}else{out.push(b[j]);j+=1;} }
    out.extend_from_slice(&a[i..]); out.extend_from_slice(&b[j..]); out
}

fn par_sort(mut v: Vec<i64>) -> Vec<i64> {
    if v.len() <= 512 { v.sort(); return v; }
    let right = v.split_off(v.len()/2);
    let left = v;
    let (sl, sr) = join(move || par_sort(left), move || par_sort(right));
    merge(sl, sr)
}

fn main() {
    let (a,b) = join(|| (1u64..=5000).sum::<u64>(), || (5001u64..=10000).sum::<u64>());
    println!("sum 1..10000 = {} (expected 50005000)", a+b);
    let data: Vec<i64> = (0..1000).rev().collect();
    let sorted = par_sort(data);
    println!("sorted[0..3]: {:?}", &sorted[..3]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_join()     { let (a,b)=join(||6*7, ||"hi".len()); assert_eq!(a,42); assert_eq!(b,2); }
    #[test] fn test_par_sort() { let d=vec![5i64,3,8,1,9,2,7,4,6]; assert_eq!(par_sort(d),vec![1,2,3,4,5,6,7,8,9]); }
}
