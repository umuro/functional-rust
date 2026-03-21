#![allow(dead_code)]
#![allow(clippy::all)]
// 1020: try_fold — Fold that short-circuits on error

// Approach 1: Iterator::try_fold
fn sum_positive(numbers: &[i64]) -> Result<i64, String> {
    numbers.iter().try_fold(0i64, |acc, &n| {
        if n < 0 {
            Err(format!("negative number: {}", n))
        } else {
            Ok(acc + n)
        }
    })
}

// Approach 2: try_fold with accumulator transformation
fn concat_limited(strings: &[&str], max_len: usize) -> Result<String, String> {
    strings.iter().try_fold(String::new(), |mut acc, &s| {
        acc.push_str(s);
        if acc.len() > max_len {
            Err(format!("result too long: {} > {}", acc.len(), max_len))
        } else {
            Ok(acc)
        }
    })
}

// Approach 3: try_fold vs regular fold comparison
fn product_no_overflow(numbers: &[i64]) -> Result<i64, String> {
    numbers.iter().try_fold(1i64, |acc, &n| {
        acc.checked_mul(n)
            .ok_or_else(|| format!("overflow at {} * {}", acc, n))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_all_positive() {
        assert_eq!(sum_positive(&[1, 2, 3]), Ok(6));
    }

    #[test]
    fn test_sum_negative_fails() {
        let result = sum_positive(&[1, -2, 3]);
        assert_eq!(result, Err("negative number: -2".to_string()));
    }

    #[test]
    fn test_sum_empty() {
        assert_eq!(sum_positive(&[]), Ok(0));
    }

    #[test]
    fn test_concat_ok() {
        assert_eq!(
            concat_limited(&["hello", " ", "world"], 20),
            Ok("hello world".to_string())
        );
    }

    #[test]
    fn test_concat_too_long() {
        let result = concat_limited(&["hello", " ", "world!!!!!!!!!!!!"], 10);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too long"));
    }

    #[test]
    fn test_product_ok() {
        assert_eq!(product_no_overflow(&[2, 3, 4]), Ok(24));
    }

    #[test]
    fn test_product_overflow() {
        let result = product_no_overflow(&[i64::MAX, 2]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("overflow"));
    }

    #[test]
    fn test_short_circuit_proof() {
        // try_fold stops processing after first error
        let mut count = 0;
        let result = [1, -2, 3, 4, 5].iter().try_fold(0, |acc, &n| {
            count += 1;
            if n < 0 {
                Err("negative")
            } else {
                Ok(acc + n)
            }
        });
        assert!(result.is_err());
        assert_eq!(count, 2); // only processed [1, -2], stopped
    }

    #[test]
    fn test_try_fold_vs_fold() {
        // Regular fold processes everything
        let sum = [1, 2, 3].iter().fold(0, |acc, n| acc + n);
        assert_eq!(sum, 6);

        // try_fold can bail early
        let result = [1, 2, 3].iter().try_fold(0, |acc, &n| {
            if acc + n > 4 {
                Err("too big")
            } else {
                Ok(acc + n)
            }
        });
        assert!(result.is_err());
    }
}
