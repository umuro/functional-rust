// Solution 1: Direct recursion — mirrors OCaml `fib_naive`
pub fn fib_naive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fib_naive(n - 1) + fib_naive(n - 2),
    }
}

// Solution 2: Tail-recursive with accumulator — mirrors OCaml `fib_tail`
pub fn fib_tail(n: u64) -> u64 {
    fn go(a: u64, b: u64, n: u64) -> u64 {
        match n {
            0 => a,
            n => go(b, a + b, n - 1),
        }
    }
    go(0, 1, n)
}

// Solution 3: Fold-based — mirrors OCaml `fib_fold`
pub fn fib_fold(n: u64) -> u64 {
    let (a, _) = (0..n).fold((0u64, 1u64), |(a, b), _| (b, a + b));
    a
}

// Solution 4: Idiomatic Rust iterator
pub fn fib_iter(n: u64) -> u64 {
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a
}

fn main() {
    println!("{:>3} | naive | tail  | fold  | iter", "n");
    println!("----+-------+-------+-------+------");
    for n in [0, 1, 2, 5, 10, 20] {
        println!(
            "{:>3} | {:>5} | {:>5} | {:>5} | {:>5}",
            n,
            fib_naive(n),
            fib_tail(n),
            fib_fold(n),
            fib_iter(n),
        );
    }
}

/* Output:
  n | naive | tail  | fold  | iter
----+-------+-------+-------+------
  0 |     0 |     0 |     0 |     0
  1 |     1 |     1 |     1 |     1
  2 |     1 |     1 |     1 |     1
  5 |     5 |     5 |     5 |     5
 10 |    55 |    55 |    55 |    55
 20 |  6765 |  6765 |  6765 |  6765
*/

#[cfg(test)]
mod tests {
    use super::*;

    const KNOWN: &[(u64, u64)] = &[
        (0, 0),
        (1, 1),
        (2, 1),
        (3, 2),
        (5, 5),
        (10, 55),
        (20, 6765),
    ];

    #[test]
    fn test_naive_known_values() {
        for &(n, expected) in KNOWN {
            assert_eq!(fib_naive(n), expected, "fib_naive({n})");
        }
    }

    #[test]
    fn test_tail_known_values() {
        for &(n, expected) in KNOWN {
            assert_eq!(fib_tail(n), expected, "fib_tail({n})");
        }
    }

    #[test]
    fn test_fold_known_values() {
        for &(n, expected) in KNOWN {
            assert_eq!(fib_fold(n), expected, "fib_fold({n})");
        }
    }

    #[test]
    fn test_iter_known_values() {
        for &(n, expected) in KNOWN {
            assert_eq!(fib_iter(n), expected, "fib_iter({n})");
        }
    }

    #[test]
    fn test_all_implementations_agree() {
        for n in 0..=20u64 {
            let expected = fib_naive(n);
            assert_eq!(fib_tail(n), expected, "tail disagrees at {n}");
            assert_eq!(fib_fold(n), expected, "fold disagrees at {n}");
            assert_eq!(fib_iter(n), expected, "iter disagrees at {n}");
        }
    }

    #[test]
    fn test_base_cases() {
        assert_eq!(fib_naive(0), 0);
        assert_eq!(fib_naive(1), 1);
        assert_eq!(fib_tail(0), 0);
        assert_eq!(fib_tail(1), 1);
        assert_eq!(fib_fold(0), 0);
        assert_eq!(fib_fold(1), 1);
        assert_eq!(fib_iter(0), 0);
        assert_eq!(fib_iter(1), 1);
    }

    #[test]
    fn test_larger_value() {
        assert_eq!(fib_tail(30), 832_040);
        assert_eq!(fib_fold(30), 832_040);
        assert_eq!(fib_iter(30), 832_040);
    }
}
