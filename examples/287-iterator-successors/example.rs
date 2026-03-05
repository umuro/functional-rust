//! 287. Recursive sequences with successors()
//!
//! `successors(first, f)` generates a sequence: first, f(first), f(f(first)), ...

fn main() {
    // Powers of 2
    let powers_of_2: Vec<u32> = std::iter::successors(Some(1u32), |&n| {
        if n < 512 { Some(n * 2) } else { None }
    }).collect();
    println!("Powers of 2: {:?}", powers_of_2);

    // Collatz sequence from 6
    let collatz: Vec<u64> = std::iter::successors(Some(6u64), |&n| {
        if n == 1 { None }
        else if n % 2 == 0 { Some(n / 2) }
        else { Some(3 * n + 1) }
    }).collect();
    println!("Collatz(6): {:?}", collatz);

    // Geometric sequence (multiply by 3 each step)
    let geometric: Vec<i32> = std::iter::successors(Some(1i32), |&n| {
        if n >= 729 { None } else { Some(n * 3) }
    }).collect();
    println!("Geometric (x3): {:?}", geometric);

    // String processing: repeatedly remove first char
    let shrinking: Vec<String> = std::iter::successors(
        Some("hello".to_string()),
        |s| if s.is_empty() { None } else { Some(s[1..].to_string()) }
    ).collect();
    println!("Shrinking: {:?}", shrinking);

    // Newton's method square root approximation (finite steps)
    let sqrt2_approx: Vec<f64> = std::iter::successors(Some(1.0f64), |&x| {
        let next = 0.5 * (x + 2.0 / x);
        if (next - x).abs() < 1e-10 { None } else { Some(next) }
    }).collect();
    println!("sqrt(2) steps: {:?}", &sqrt2_approx[..sqrt2_approx.len().min(5)]);
    println!("sqrt(2) ≈ {:.10}", sqrt2_approx.last().unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_powers_of_2() {
        let result: Vec<u32> = std::iter::successors(Some(1u32), |&n| {
            if n < 16 { Some(n * 2) } else { None }
        }).collect();
        assert_eq!(result, vec![1, 2, 4, 8, 16]);
    }

    #[test]
    fn test_collatz_6() {
        let result: Vec<u64> = std::iter::successors(Some(6u64), |&n| {
            if n == 1 { None }
            else if n % 2 == 0 { Some(n / 2) }
            else { Some(3 * n + 1) }
        }).collect();
        assert_eq!(result, vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn test_successors_empty_if_first_is_none() {
        let result: Vec<i32> = std::iter::successors(None, |&_n: &i32| Some(1))
            .collect();
        assert!(result.is_empty());
    }
}
