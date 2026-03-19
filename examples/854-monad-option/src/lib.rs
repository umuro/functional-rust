// Example 055: Option Monad
// Monadic bind (and_then) for Option: chain computations that may fail

use std::collections::HashMap;

// Approach 1: Safe lookup chain using and_then
fn find_user_docs(env: &HashMap<&str, &str>, paths: &HashMap<&str, Vec<&str>>) -> Option<String> {
    env.get("HOME")
        .and_then(|home| paths.get(home.to_owned()))
        .and_then(|dirs| {
            if dirs.contains(&"documents") {
                Some("documents found".to_string())
            } else {
                None
            }
        })
}

// Approach 2: Safe arithmetic chain
fn safe_div(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn safe_sqrt(x: i32) -> Option<f64> {
    if x < 0 {
        None
    } else {
        Some((x as f64).sqrt())
    }
}

fn compute(a: i32, b: i32) -> Option<i32> {
    safe_div(a, b).and_then(|q| safe_sqrt(q)).map(|r| r as i32)
}

// Approach 3: Using ? operator (Rust's monadic sugar for Option)
fn compute_question_mark(a: i32, b: i32) -> Option<i32> {
    let q = safe_div(a, b)?;
    let r = safe_sqrt(q)?;
    Some(r as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (
        HashMap<&'static str, &'static str>,
        HashMap<&'static str, Vec<&'static str>>,
    ) {
        let mut env = HashMap::new();
        env.insert("HOME", "/home/user");
        let mut paths = HashMap::new();
        paths.insert("/home/user", vec!["documents", "photos"]);
        (env, paths)
    }

    #[test]
    fn test_lookup_chain_success() {
        let (env, paths) = setup();
        assert_eq!(
            find_user_docs(&env, &paths),
            Some("documents found".to_string())
        );
    }

    #[test]
    fn test_lookup_chain_missing_key() {
        let env = HashMap::new();
        let paths = HashMap::new();
        assert_eq!(find_user_docs(&env, &paths), None);
    }

    #[test]
    fn test_safe_div_success() {
        assert_eq!(safe_div(10, 2), Some(5));
    }

    #[test]
    fn test_safe_div_by_zero() {
        assert_eq!(safe_div(10, 0), None);
    }

    #[test]
    fn test_compute_success() {
        assert_eq!(compute(100, 4), Some(5));
    }

    #[test]
    fn test_compute_div_zero() {
        assert_eq!(compute(100, 0), None);
    }

    #[test]
    fn test_compute_negative_sqrt() {
        assert_eq!(compute(-100, 1), None);
    }

    #[test]
    fn test_question_mark_same_as_and_then() {
        assert_eq!(compute(100, 4), compute_question_mark(100, 4));
        assert_eq!(compute(100, 0), compute_question_mark(100, 0));
        assert_eq!(compute(-100, 1), compute_question_mark(-100, 1));
    }
}
