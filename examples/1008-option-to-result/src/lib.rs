// 1008: Option to Result Conversion
// Convert Option<T> to Result<T, E> with ok_or / ok_or_else

use std::collections::HashMap;

fn build_users() -> HashMap<String, (String, u32)> {
    let mut m = HashMap::new();
    m.insert("Alice".into(), ("alice@ex.com".into(), 30));
    m.insert("Bob".into(), ("bob@ex.com".into(), 17));
    m
}

// Approach 1: ok_or — eager error value
fn find_user_eager<'a>(users: &'a HashMap<String, (String, u32)>, name: &str) -> Result<&'a (String, u32), String> {
    users.get(name).ok_or(format!("user not found: {}", name))
}

// Approach 2: ok_or_else — lazy error (avoids allocation if Some)
fn find_user_lazy<'a>(users: &'a HashMap<String, (String, u32)>, name: &str) -> Result<&'a (String, u32), String> {
    users.get(name).ok_or_else(|| format!("user not found: {}", name))
}

// Approach 3: Chaining Option->Result in a pipeline
fn find_and_validate(
    users: &HashMap<String, (String, u32)>,
    name: &str,
    min_age: u32,
) -> Result<(String, u32), String> {
    users
        .get(name)
        .ok_or_else(|| format!("user not found: {}", name))
        .and_then(|(email, age)| {
            if *age >= min_age {
                Ok((email.clone(), *age))
            } else {
                Err(format!("{} is too young ({} < {})", name, age, min_age))
            }
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_or_found() {
        let users = build_users();
        let result = find_user_eager(&users, "Alice");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().1, 30);
    }

    #[test]
    fn test_ok_or_not_found() {
        let users = build_users();
        let result = find_user_eager(&users, "Unknown");
        assert_eq!(result.unwrap_err(), "user not found: Unknown");
    }

    #[test]
    fn test_ok_or_else_lazy() {
        let users = build_users();
        assert!(find_user_lazy(&users, "Bob").is_ok());
        assert!(find_user_lazy(&users, "Nobody").is_err());
    }

    #[test]
    fn test_validate_success() {
        let users = build_users();
        let result = find_and_validate(&users, "Alice", 18);
        assert_eq!(result.unwrap(), ("alice@ex.com".into(), 30));
    }

    #[test]
    fn test_validate_too_young() {
        let users = build_users();
        let result = find_and_validate(&users, "Bob", 18);
        assert!(result.unwrap_err().contains("too young"));
    }

    #[test]
    fn test_validate_not_found() {
        let users = build_users();
        let result = find_and_validate(&users, "Nobody", 18);
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_option_methods() {
        // Direct Option -> Result conversions
        assert_eq!(Some(42).ok_or("missing"), Ok(42));
        assert_eq!(None::<i32>.ok_or("missing"), Err("missing"));

        // Result -> Option conversions
        assert_eq!(Ok::<i32, &str>(42).ok(), Some(42));
        assert_eq!(Err::<i32, &str>("fail").ok(), None);
    }
}
