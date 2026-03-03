/// Collatz Conjecture
///
/// Computing the 3n+1 sequence step count. Demonstrates simple recursion,
/// Result-typed safe API, and iterative variants.

/// Naive recursive — mirrors OCaml's version directly.
pub fn collatz_steps(n: u64) -> u64 {
    match n {
        1 => 0,
        n if n % 2 == 0 => 1 + collatz_steps(n / 2),
        n => 1 + collatz_steps(3 * n + 1),
    }
}

/// Safe API with Result — rejects non-positive inputs.
pub fn collatz(n: i64) -> Result<u64, String> {
    if n <= 0 {
        Err("Only positive integers are allowed".to_string())
    } else {
        Ok(collatz_steps(n as u64))
    }
}

/// Iterative version — idiomatic Rust, no recursion.
pub fn collatz_iter(n: i64) -> Result<u64, String> {
    if n <= 0 {
        return Err("Only positive integers are allowed".to_string());
    }
    let mut current = n as u64;
    let mut steps = 0u64;
    while current != 1 {
        current = if current % 2 == 0 {
            current / 2
        } else {
            3 * current + 1
        };
        steps += 1;
    }
    Ok(steps)
}

/// Generate the full Collatz sequence.
pub fn collatz_sequence(n: u64) -> Vec<u64> {
    let mut seq = vec![n];
    let mut current = n;
    while current != 1 {
        current = if current % 2 == 0 {
            current / 2
        } else {
            3 * current + 1
        };
        seq.push(current);
    }
    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_1() {
        assert_eq!(collatz(1), Ok(0));
    }

    #[test]
    fn test_collatz_6() {
        assert_eq!(collatz(6), Ok(8));
    }

    #[test]
    fn test_collatz_11() {
        assert_eq!(collatz(11), Ok(14));
    }

    #[test]
    fn test_collatz_27() {
        assert_eq!(collatz(27), Ok(111));
    }

    #[test]
    fn test_collatz_negative() {
        assert!(collatz(-1).is_err());
        assert!(collatz(0).is_err());
    }

    #[test]
    fn test_iter_matches_recursive() {
        for n in 1..=100 {
            assert_eq!(collatz(n), collatz_iter(n));
        }
    }

    #[test]
    fn test_sequence() {
        assert_eq!(collatz_sequence(6), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }
}

fn main() {
    println!("{:?}", collatz(1), Ok(0));
    println!("{:?}", collatz(6), Ok(8));
    println!("{:?}", collatz(11), Ok(14));
}
