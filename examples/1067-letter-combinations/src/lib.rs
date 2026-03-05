// 1067: Phone Keypad Letter Combinations

const PHONE_MAP: &[&str] = &["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

// Approach 1: Backtracking
fn letter_combos(digits: &str) -> Vec<String> {
    if digits.is_empty() { return vec![]; }
    let mut results = Vec::new();
    let mut current = String::new();
    let digits: Vec<usize> = digits.bytes().map(|b| (b - b'0') as usize).collect();

    fn backtrack(idx: usize, digits: &[usize], current: &mut String, results: &mut Vec<String>) {
        if idx == digits.len() {
            results.push(current.clone());
            return;
        }
        for c in PHONE_MAP[digits[idx]].chars() {
            current.push(c);
            backtrack(idx + 1, digits, current, results);
            current.pop();
        }
    }

    backtrack(0, &digits, &mut current, &mut results);
    results
}

// Approach 2: Iterative with queue
fn letter_combos_iter(digits: &str) -> Vec<String> {
    if digits.is_empty() { return vec![]; }
    let mut queue = vec![String::new()];
    for b in digits.bytes() {
        let letters = PHONE_MAP[(b - b'0') as usize];
        let mut next_queue = Vec::new();
        for current in &queue {
            for c in letters.chars() {
                let mut s = current.clone();
                s.push(c);
                next_queue.push(s);
            }
        }
        queue = next_queue;
    }
    queue
}

// Approach 3: Functional with fold
fn letter_combos_fold(digits: &str) -> Vec<String> {
    if digits.is_empty() { return vec![]; }
    digits.bytes().fold(vec![String::new()], |acc, b| {
        let letters = PHONE_MAP[(b - b'0') as usize];
        acc.iter()
            .flat_map(|prefix| letters.chars().map(move |c| format!("{}{}", prefix, c)))
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtrack() {
        let r = letter_combos("23");
        assert_eq!(r.len(), 9);
        assert!(r.contains(&"ad".to_string()));
        assert!(r.contains(&"cf".to_string()));
    }

    #[test]
    fn test_iter() {
        let r = letter_combos_iter("23");
        assert_eq!(r.len(), 9);
    }

    #[test]
    fn test_fold() {
        let r = letter_combos_fold("23");
        assert_eq!(r.len(), 9);
    }

    #[test]
    fn test_empty() {
        assert!(letter_combos("").is_empty());
    }

    #[test]
    fn test_single() {
        assert_eq!(letter_combos("7").len(), 4); // pqrs
    }
}
