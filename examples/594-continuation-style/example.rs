// CPS: pass result to continuation k
fn fact_k<R>(n: u64, k: impl FnOnce(u64) -> R) -> R {
    if n <= 1 { k(1) }
    else      { fact_k(n-1, move |r| k(n * r)) }
}

fn fib_k<R: 'static>(n: u64, k: Box<dyn FnOnce(u64) -> R>) -> R {
    if n <= 1 { k(n) }
    else {
        fib_k(n-1, Box::new(move |r1| {
            fib_k(n-2, Box::new(move |r2| { k(r1+r2) }))
        }))
    }
}

fn map_k<T,U,R>(
    items: Vec<T>,
    f: impl Fn(T) -> U + Clone,
    k: impl FnOnce(Vec<U>) -> R,
) -> R {
    fn go<T,U,R>(mut items: Vec<T>, f: impl Fn(T)->U+Clone, mut acc: Vec<U>, k: impl FnOnce(Vec<U>)->R) -> R {
        if items.is_empty() { acc.reverse(); k(acc) }
        else {
            let head = items.remove(0);
            let u = f(head);
            acc.push(u);
            go(items, f, acc, k)
        }
    }
    go(items, f, Vec::new(), k)
}

// CPS error handling (no exceptions needed)
fn safe_div_k<R>(a: f64, b: f64, ok: impl FnOnce(f64)->R, err: impl FnOnce(&str)->R) -> R {
    if b == 0.0 { err("division by zero") }
    else        { ok(a / b) }
}

fn main() {
    fact_k(10, |n| println!("10! = {}", n));
    fib_k(10,  Box::new(|n| println!("fib(10) = {}", n)));
    map_k(vec![1u64,2,3,4,5], |x| x*2, |r| println!("{:?}", r));
    safe_div_k(10.0, 2.0, |r| println!("10/2={}", r), |e| println!("err: {}", e));
    safe_div_k(10.0, 0.0, |r| println!("10/0={}", r), |e| println!("err: {}", e));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn fact_cps() { fact_k(5, |n| assert_eq!(n, 120)); }
    #[test] fn div_ok()   { safe_div_k(10.0,2.0, |r| assert_eq!(r,5.0), |_| panic!()); }
    #[test] fn div_err()  { let mut e=false; safe_div_k(1.0,0.0,|_|{},|_|{e=true;}); assert!(e); }
}
