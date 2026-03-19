//! 280. Existential checks: any() and all()
//!
//! `any(pred)` is ∃, `all(pred)` is ∀ — both short-circuit.

#[cfg(test)]
mod tests {
    #[test]
    fn test_any_true() {
        assert!([1i32, 2, 3].iter().any(|&x| x == 2));
    }

    #[test]
    fn test_any_false() {
        assert!(![1i32, 2, 3].iter().any(|&x| x == 9));
    }

    #[test]
    fn test_all_true() {
        assert!([2i32, 4, 6].iter().all(|&x| x % 2 == 0));
    }

    #[test]
    fn test_all_false() {
        assert!(![1i32, 2, 3].iter().all(|&x| x % 2 == 0));
    }

    #[test]
    fn test_vacuous_truth() {
        let empty: Vec<i32> = vec![];
        assert!(empty.iter().all(|_| false)); // vacuously true
        assert!(!empty.iter().any(|_| true)); // no elements
    }
}
