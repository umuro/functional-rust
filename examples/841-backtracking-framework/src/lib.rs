//! # Backtracking Framework
pub fn generate_permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    let mut result = vec![]; let mut current = vec![]; let mut used = vec![false; items.len()];
    fn backtrack<T: Clone>(items: &[T], curr: &mut Vec<T>, used: &mut [bool], res: &mut Vec<Vec<T>>) {
        if curr.len() == items.len() { res.push(curr.clone()); return; }
        for i in 0..items.len() {
            if !used[i] { used[i] = true; curr.push(items[i].clone());
                backtrack(items, curr, used, res); curr.pop(); used[i] = false; }
        }
    }
    backtrack(items, &mut current, &mut used, &mut result); result
}

pub fn generate_subsets<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    let mut result = vec![]; let mut current = vec![];
    fn backtrack<T: Clone>(items: &[T], start: usize, curr: &mut Vec<T>, res: &mut Vec<Vec<T>>) {
        res.push(curr.clone());
        for i in start..items.len() { curr.push(items[i].clone()); backtrack(items, i + 1, curr, res); curr.pop(); }
    }
    backtrack(items, 0, &mut current, &mut result); result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_perms() { assert_eq!(generate_permutations(&[1, 2, 3]).len(), 6); }
    #[test] fn test_subsets() { assert_eq!(generate_subsets(&[1, 2, 3]).len(), 8); }
}
