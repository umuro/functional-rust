// 099: Group Consecutive Equal Elements

fn group_by<T: PartialEq + Clone>(v: &[T]) -> Vec<Vec<T>> {
    if v.is_empty() {
        return vec![];
    }
    let mut groups: Vec<Vec<T>> = vec![vec![v[0].clone()]];
    for item in &v[1..] {
        if item == groups.last().unwrap().last().unwrap() {
            groups.last_mut().unwrap().push(item.clone());
        } else {
            groups.push(vec![item.clone()]);
        }
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_by() {
        assert_eq!(
            group_by(&[1, 1, 2, 2, 2, 3, 1, 1]),
            vec![vec![1, 1], vec![2, 2, 2], vec![3], vec![1, 1]]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(group_by::<i32>(&[]), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_single() {
        assert_eq!(group_by(&[1]), vec![vec![1]]);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(group_by(&[5, 5, 5]), vec![vec![5, 5, 5]]);
    }
}
