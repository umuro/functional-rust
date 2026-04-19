// 068: Tail-Recursive Accumulator
// Transform naive recursion into tail-recursive form by carrying an accumulator.
// Rust does NOT guarantee TCO — `iter().fold()` and explicit loops are the
// idiomatic replacement for accumulator recursion on large inputs.

// --- Sum ---

fn sum_naive(v: &[i32]) -> i32 {
    match v {
        [] => 0,
        [x, rest @ ..] => *x + sum_naive(rest),
    }
}

fn sum_tail(v: &[i32]) -> i32 {
    fn aux(acc: i32, v: &[i32]) -> i32 {
        match v {
            [] => acc,
            [x, rest @ ..] => aux(acc + *x, rest),
        }
    }
    aux(0, v)
}

fn sum_fold(v: &[i32]) -> i32 {
    v.iter().sum()
}

// --- Factorial ---

fn fact_naive(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * fact_naive(n - 1)
    }
}

fn fact_tail(n: u64) -> u64 {
    fn aux(acc: u64, n: u64) -> u64 {
        if n <= 1 {
            acc
        } else {
            aux(acc * n, n - 1)
        }
    }
    aux(1, n)
}

fn fact_fold(n: u64) -> u64 {
    (1..=n).product()
}

// --- Fibonacci ---

fn fib_naive(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib_naive(n - 1) + fib_naive(n - 2)
    }
}

fn fib_tail(n: u64) -> u64 {
    fn aux(a: u64, b: u64, n: u64) -> u64 {
        if n == 0 {
            a
        } else {
            aux(b, a + b, n - 1)
        }
    }
    aux(0, 1, n)
}

fn fib_fold(n: u64) -> u64 {
    (0..n).fold((0u64, 1u64), |(a, b), _| (b, a + b)).0
}

fn main() {
    println!("sum_naive([1,2,3,4,5]) = {}", sum_naive(&[1, 2, 3, 4, 5]));
    println!("sum_tail([1,2,3,4,5])  = {}", sum_tail(&[1, 2, 3, 4, 5]));
    println!("sum_fold([1,2,3,4,5])  = {}", sum_fold(&[1, 2, 3, 4, 5]));
    println!("fact_naive(5) = {}", fact_naive(5));
    println!("fact_tail(5)  = {}", fact_tail(5));
    println!("fact_fold(10) = {}", fact_fold(10));
    println!("fib_naive(10) = {}", fib_naive(10));
    println!("fib_tail(10)  = {}", fib_tail(10));
    println!("fib_fold(50)  = {}", fib_fold(50));

    // `fold` compiles to a loop — stack-safe for large inputs.
    let large: Vec<i32> = vec![1; 100_000];
    println!("sum_fold(100k ones) = {}", sum_fold(&large));
}

/* Output:
   sum_naive([1,2,3,4,5]) = 15
   sum_tail([1,2,3,4,5])  = 15
   sum_fold([1,2,3,4,5])  = 15
   fact_naive(5) = 120
   fact_tail(5)  = 120
   fact_fold(10) = 3628800
   fib_naive(10) = 55
   fib_tail(10)  = 55
   fib_fold(50)  = 12586269025
   sum_fold(100k ones) = 100000
*/
