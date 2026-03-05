// Example 058: FlatMap/Bind Chains
// Long monadic chains for sequential computation with early exit

// Approach 1: Multi-step data processing pipeline
fn parse_json(s: &str) -> Option<&str> {
    if s.starts_with('{') { Some(s) } else { None }
}

fn extract_field<'a>(key: &str, json: &'a str) -> Option<&'a str> {
    let pattern = format!("\"{}\":\"", key);
    let start = json.find(&pattern)? + pattern.len();
    let rest = &json[start..];
    let end = rest.find('"')?;
    Some(&rest[..end])
}

fn validate_length(min: usize, max: usize, s: &str) -> Option<&str> {
    if s.len() >= min && s.len() <= max { Some(s) } else { None }
}

fn process_name(json: &str) -> Option<String> {
    parse_json(json)
        .and_then(|j| extract_field("name", j))
        .and_then(|name| validate_length(1, 50, name))
        .map(|s| s.to_uppercase())
}

// Approach 2: Database-like lookup chain with ?
#[derive(Clone, Debug)]
struct User { id: u32, dept_id: u32, name: String }

#[derive(Clone, Debug)]
struct Dept { id: u32, mgr_id: u32, name: String }

fn find_manager_dept_name(
    user_id: u32,
    users: &[User],
    depts: &[Dept],
) -> Option<String> {
    let user = users.iter().find(|u| u.id == user_id)?;
    let dept = depts.iter().find(|d| d.id == user.dept_id)?;
    let manager = users.iter().find(|u| u.id == dept.mgr_id)?;
    Some(format!("{}'s manager is {} in {}", user.name, manager.name, dept.name))
}

// Approach 3: Computation with bounds checking
fn step_add(n: i32, acc: i32) -> Option<i32> {
    let result = acc + n;
    if result > 100 { None } else { Some(result) }
}

fn step_mul(n: i32, acc: i32) -> Option<i32> {
    let result = acc * n;
    if result > 100 { None } else { Some(result) }
}

fn compute() -> Option<i32> {
    Some(0)
        .and_then(|a| step_add(10, a))
        .and_then(|a| step_mul(3, a))
        .and_then(|a| step_add(20, a))
        .and_then(|a| step_add(40, a))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_name_valid() {
        assert_eq!(process_name("{\"name\":\"alice\"}"), Some("ALICE".to_string()));
    }

    #[test]
    fn test_process_name_invalid_json() {
        assert_eq!(process_name("not json"), None);
    }

    #[test]
    fn test_process_name_missing_field() {
        assert_eq!(process_name("{\"age\":\"30\"}"), None);
    }

    fn setup() -> (Vec<User>, Vec<Dept>) {
        let users = vec![
            User { id: 1, dept_id: 10, name: "Alice".into() },
            User { id: 2, dept_id: 20, name: "Bob".into() },
        ];
        let depts = vec![
            Dept { id: 10, mgr_id: 2, name: "Engineering".into() },
            Dept { id: 20, mgr_id: 1, name: "Marketing".into() },
        ];
        (users, depts)
    }

    #[test]
    fn test_find_manager() {
        let (users, depts) = setup();
        let result = find_manager_dept_name(1, &users, &depts);
        assert_eq!(result, Some("Alice's manager is Bob in Engineering".to_string()));
    }

    #[test]
    fn test_find_manager_missing() {
        let (users, depts) = setup();
        assert_eq!(find_manager_dept_name(99, &users, &depts), None);
    }

    #[test]
    fn test_compute() {
        assert_eq!(compute(), Some(90)); // 0+10=10, 10*3=30, 30+20=50, 50+40=90
    }

    #[test]
    fn test_compute_overflow() {
        // If we added step that would exceed 100
        let r = Some(50)
            .and_then(|a| step_mul(3, a)); // 150 > 100
        assert_eq!(r, None);
    }
}
