// 448. Rayon parallel iterators — concept via std threads
use std::thread;

fn parallel_map<T: Sync, U: Send+Default+Clone, F: Fn(&T)->U+Sync>(data: &[T], f: F) -> Vec<U> {
    let n = thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let chunk = (data.len() / n).max(1);
    let mut out = vec![U::default(); data.len()];
    thread::scope(|s| {
        for (ci, co) in data.chunks(chunk).zip(out.chunks_mut(chunk)) {
            s.spawn(|| { for (d,r) in ci.iter().zip(co.iter_mut()) { *r = f(d); } });
        }
    });
    out
}

fn parallel_sum(data: &[f64]) -> f64 {
    let n = 4usize;
    let chunk = (data.len() / n).max(1);
    let partials: Vec<f64> = thread::scope(|s|
        data.chunks(chunk).map(|c| s.spawn(move || c.iter().sum::<f64>()))
            .collect::<Vec<_>>().into_iter().map(|h| h.join().unwrap()).collect()
    );
    partials.iter().sum()
}

fn main() {
    let data: Vec<f64> = (1..=1000).map(|x| x as f64).collect();
    let sq = parallel_map(&data, |x| x*x);
    println!("sum squares = {:.0}", sq.iter().sum::<f64>());
    println!("parallel sum = {:.0}", parallel_sum(&data));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_map()  { let d:Vec<f64>=(1..=5).map(|x| x as f64).collect(); let r=parallel_map(&d,|x|x*x); assert_eq!(r,vec![1.,4.,9.,16.,25.]); }
    #[test] fn test_sum()  { let d:Vec<f64>=(1..=100).map(|x|x as f64).collect(); assert!((parallel_sum(&d)-5050.).abs()<1e-9); }
}
