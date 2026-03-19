#![allow(clippy::all)]
// 1022: Sentinel Values vs Result
// Migrating sentinel values to Option/Result

// Approach 1: Sentinel values — the C way (DON'T DO THIS in Rust)
fn find_index_sentinel(haystack: &[i32], needle: i32) -> i32 {
    for (i, &val) in haystack.iter().enumerate() {
        if val == needle {
            return i as i32;
        }
    }
    -1 // sentinel: "not found"
}

fn get_config_sentinel(key: &str) -> &str {
    match key {
        "port" => "8080",
        _ => "", // sentinel: "missing"
    }
}

// Approach 2: Option — explicit absence (PREFERRED for lookups)
fn find_index(haystack: &[i32], needle: i32) -> Option<usize> {
    haystack.iter().position(|&x| x == needle)
}

fn get_config(key: &str) -> Option<&str> {
    match key {
        "port" => Some("8080"),
        _ => None,
    }
}

// Approach 3: Result — absence with reason (PREFERRED when error matters)
fn find_index_result(haystack: &[&str], needle: &str) -> Result<usize, String> {
    haystack
        .iter()
        .position(|&x| x == needle)
        .ok_or_else(|| format!("{} not in list", needle))
}

fn get_config_result(key: &str) -> Result<&str, String> {
    match key {
        "port" => Ok("8080"),
        _ => Err(format!("key not found: {}", key)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentinel_found() {
        assert_eq!(find_index_sentinel(&[1, 2, 3], 2), 1);
    }

    #[test]
    fn test_sentinel_not_found() {
        assert_eq!(find_index_sentinel(&[1, 2, 3], 9), -1);
        // Problem: caller must remember to check for -1
    }

    #[test]
    fn test_option_found() {
        assert_eq!(find_index(&[1, 2, 3], 2), Some(1));
    }

    #[test]
    fn test_option_not_found() {
        assert_eq!(find_index(&[1, 2, 3], 9), None);
        // Compiler forces you to handle None
    }

    #[test]
    fn test_result_found() {
        assert_eq!(find_index_result(&["a", "b", "c"], "b"), Ok(1));
    }

    #[test]
    fn test_result_not_found() {
        let err = find_index_result(&["a", "b"], "z").unwrap_err();
        assert!(err.contains("not in list"));
    }

    #[test]
    fn test_config_sentinel_ambiguity() {
        // Is "" a valid config value or "missing"? Can't tell!
        assert_eq!(get_config_sentinel("missing"), "");
        // With Option, it's clear:
        assert_eq!(get_config("missing"), None);
    }

    #[test]
    fn test_config_result() {
        assert_eq!(get_config_result("port"), Ok("8080"));
        assert!(get_config_result("unknown").is_err());
    }

    #[test]
    fn test_migration_pattern() {
        // Common migration: wrap sentinel check in Option
        fn migrate(val: i32) -> Option<i32> {
            if val == -1 {
                None
            } else {
                Some(val)
            }
        }
        assert_eq!(migrate(find_index_sentinel(&[1, 2], 2)), Some(1));
        assert_eq!(migrate(find_index_sentinel(&[1, 2], 9)), None);
    }
}
