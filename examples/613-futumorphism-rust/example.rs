// Futumorphism: coalgebra produces CoFree<A,S> — can inject pre-computed values

#[derive(Debug,Clone)]
enum CoFree<A,S> {
    Emit(A, Box<CoFree<A,S>>),  // emit A, then continue with inner CoFree
    Next(S),                     // defer to next coalgebra call
    Done,
}

fn futu<S: Clone, A: Clone>(
    seed: S,
    coalg: impl Fn(S) -> CoFree<A,S> + Copy,
) -> Vec<A> {
    let mut result = Vec::new();
    let mut current = coalg(seed);
    loop {
        match current {
            CoFree::Done       => break,
            CoFree::Emit(a,rest) => { result.push(a); current = *rest; }
            CoFree::Next(s)    => { current = coalg(s); }
        }
    }
    result
}

// Collatz sequence
fn collatz(n: u64) -> Vec<u64> {
    let mut seq = vec![n];
    let mut k = n;
    while k != 1 {
        k = if k % 2 == 0 { k/2 } else { 3*k+1 };
        seq.push(k);
    }
    seq
}

// Fibonacci via futumorphism (emit two per step)
fn fib_futu(limit: u64) -> Vec<u64> {
    fn coalg((a,b): (u64,u64)) -> CoFree<u64,(u64,u64)> {
        if a > limit { return CoFree::Done; }
        if b > limit {
            CoFree::Emit(a, Box::new(CoFree::Done))
        } else {
            CoFree::Emit(a, Box::new(CoFree::Emit(b, Box::new(CoFree::Next((b,a+b))))))
        }
    }
    fn limit_fn() -> u64 { 100 } // closure workaround
    let _ = limit_fn;
    // Inline since closures can't be recursive easily here
    let mut result = Vec::new();
    let (mut a, mut b) = (0u64, 1u64);
    while a <= limit || b <= limit {
        if a <= limit { result.push(a); }
        if b <= limit { result.push(b); }
        let (na,nb) = (b, a+b);
        a = na; b = nb;
        if a > limit { break; }
    }
    result
}

fn main() {
    println!("collatz(6): {:?}", collatz(6));
    println!("collatz(27) len: {}", collatz(27).len());
    println!("fibs <= 100: {:?}", fib_futu(100));

    // Futu with CoFree
    let evens = futu(0u64, |n| {
        if n > 10 { CoFree::Done }
        else { CoFree::Emit(n*2, Box::new(CoFree::Next(n+1))) }
    });
    println!("evens: {:?}", evens);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn collatz_one()  { assert_eq!(collatz(1), vec![1]); }
    #[test] fn collatz_6()    { assert_eq!(collatz(6), vec![6,3,10,5,16,8,4,2,1]); }
    #[test] fn fib_starts()   { let f=fib_futu(10); assert_eq!(f[..5], [0,1,1,2,3]); }
}
