// Example 063: Monad Transformers
// Stacking monads: Option inside Result

// OptionT<E, A> = Result<Option<A>, E>
type OptionT<A, E> = Result<Option<A>, E>;

// Approach 1: Helper functions for OptionT
mod option_t {
    pub fn pure<A, E>(a: A) -> Result<Option<A>, E> {
        Ok(Some(a))
    }

    pub fn none<A, E>() -> Result<Option<A>, E> {
        Ok(None)
    }

    pub fn bind<A, B, E>(
        m: Result<Option<A>, E>,
        f: impl FnOnce(A) -> Result<Option<B>, E>,
    ) -> Result<Option<B>, E> {
        match m {
            Err(e) => Err(e),
            Ok(None) => Ok(None),
            Ok(Some(a)) => f(a),
        }
    }

    pub fn map<A, B, E>(
        m: Result<Option<A>, E>,
        f: impl FnOnce(A) -> B,
    ) -> Result<Option<B>, E> {
        match m {
            Err(e) => Err(e),
            Ok(None) => Ok(None),
            Ok(Some(a)) => Ok(Some(f(a))),
        }
    }

    pub fn lift_result<A, E>(r: Result<A, E>) -> Result<Option<A>, E> {
        r.map(Some)
    }

    pub fn lift_option<A, E>(o: Option<A>) -> Result<Option<A>, E> {
        Ok(o)
    }
}

// Approach 2: Database operations
fn find_user(id: i32) -> OptionT<String, String> {
    if id > 0 {
        Ok(Some(format!("User_{}", id)))
    } else if id == 0 {
        Ok(None)
    } else {
        Err("Invalid ID".to_string())
    }
}

fn find_email(name: &str) -> OptionT<String, String> {
    match name {
        "User_1" => Ok(Some("user1@example.com".to_string())),
        "User_2" => Ok(None),
        _ => Err("DB connection failed".to_string()),
    }
}

fn get_user_email(id: i32) -> OptionT<String, String> {
    option_t::bind(find_user(id), |name| find_email(&name))
}

// Approach 3: Using ? with nested unwrapping (idiomatic Rust)
fn get_user_email_idiomatic(id: i32) -> Result<Option<String>, String> {
    let user = match find_user(id)? {
        Some(u) => u,
        None => return Ok(None),
    };
    find_email(&user)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found_with_email() {
        assert_eq!(get_user_email(1), Ok(Some("user1@example.com".to_string())));
    }

    #[test]
    fn test_user_not_found() {
        assert_eq!(get_user_email(0), Ok(None));
    }

    #[test]
    fn test_invalid_id() {
        assert_eq!(get_user_email(-1), Err("Invalid ID".to_string()));
    }

    #[test]
    fn test_user_no_email() {
        assert_eq!(get_user_email(2), Ok(None));
    }

    #[test]
    fn test_map() {
        let upper = option_t::map(get_user_email(1), |s| s.to_uppercase());
        assert_eq!(upper, Ok(Some("USER1@EXAMPLE.COM".to_string())));
    }

    #[test]
    fn test_lift_result() {
        assert_eq!(option_t::lift_result::<_, String>(Ok(42)), Ok(Some(42)));
        assert_eq!(option_t::lift_result::<i32, _>(Err("e".to_string())), Err("e".to_string()));
    }

    #[test]
    fn test_lift_option() {
        assert_eq!(option_t::lift_option::<_, String>(Some(42)), Ok(Some(42)));
        assert_eq!(option_t::lift_option::<i32, String>(None), Ok(None));
    }

    #[test]
    fn test_idiomatic_same_results() {
        for id in [-1, 0, 1, 2] {
            assert_eq!(get_user_email(id), get_user_email_idiomatic(id));
        }
    }
}
