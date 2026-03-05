// Trampoline type
enum Bounce<T> {
    Done(T),
    More(Box<dyn FnOnce() -> Bounce<T>>),
}

fn run<T>(mut b: Bounce<T>) -> T {
    loop {
        match b {
            Bounce::Done(v)   => return v,
            Bounce::More(th)  => b = th(),
        }
    }
}

// Stack-safe factorial
fn fact_t(n: u64, acc: u64) -> Bounce<u64> {
    if n == 0 { Bounce::Done(acc) }
    else      { Bounce::More(Box::new(move || fact_t(n-1, n*acc))) }
}

// Mutually recursive even/odd — stack-safe!
fn even_t(n: u64) -> Bounce<bool> {
    if n == 0 { Bounce::Done(true)  }
    else      { Bounce::More(Box::new(move || odd_t(n-1))) }
}

fn odd_t(n: u64) -> Bounce<bool> {
    if n == 0 { Bounce::Done(false) }
    else      { Bounce::More(Box::new(move || even_t(n-1))) }
}

// Count-down: would stack-overflow without trampoline at large N
fn count_t(n: u64) -> Bounce<u64> {
    if n == 0 { Bounce::Done(0) }
    else      { Bounce::More(Box::new(move || count_t(n-1))) }
}

fn main() {
    println!("5! = {}", run(fact_t(5, 1)));
    println!("20! = {}", run(fact_t(20, 1)));
    println!("even(100) = {}", run(even_t(100)));
    println!("even(101) = {}", run(even_t(101)));
    // Stack-safe at depth 100_000
    println!("count(100000) = {}", run(count_t(100_000)));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn fact5()     { assert_eq!(run(fact_t(5,1)), 120); }
    #[test] fn even100()   { assert!(run(even_t(100))); }
    #[test] fn odd101()    { assert!(run(odd_t(101)));  }
    #[test] fn deep()      { assert_eq!(run(count_t(50_000)), 0); }
}
