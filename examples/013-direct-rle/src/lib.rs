#![allow(clippy::all)]
/// Direct Run-Length Encoding (99 Problems #13)
///
/// Implement RLE directly — don't create sublists first, then count.
/// Instead, count consecutive duplicates in a single pass.

#[derive(Debug, PartialEq, Clone)]
pub enum RleItem<T> {
    One(T),
    Many(usize, T),
}

// ── Idiomatic Rust: single-pass with state ──────────────────────────────────

pub fn encode_direct<T: PartialEq + Clone>(list: &[T]) -> Vec<RleItem<T>> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < list.len() {
        let start = i;
        while i < list.len() && list[i] == list[start] {
            i += 1;
        }
        let count = i - start;
        result.push(if count == 1 {
            RleItem::One(list[start].clone())
        } else {
            RleItem::Many(count, list[start].clone())
        });
    }
    result
}

// ── Recursive style with accumulator ────────────────────────────────────────

pub fn encode_direct_recursive<T: PartialEq + Clone>(list: &[T]) -> Vec<RleItem<T>> {
    fn aux<T: PartialEq + Clone>(list: &[T], count: usize, acc: &mut Vec<RleItem<T>>) {
        match list.split_first() {
            None => {
                // Flush any remaining count — handled by previous step
            }
            Some((head, tail)) => {
                match tail.first() {
                    Some(next) if next == head => {
                        aux(tail, count + 1, acc);
                    }
                    _ => {
                        // End of run (or end of list)
                        acc.push(if count + 1 == 1 {
                            RleItem::One(head.clone())
                        } else {
                            RleItem::Many(count + 1, head.clone())
                        });
                        aux(tail, 0, acc);
                    }
                }
            }
        }
    }
    let mut result = Vec::new();
    aux(list, 0, &mut result);
    result
}

// ── Fold-based approach ─────────────────────────────────────────────────────

pub fn encode_direct_fold<T: PartialEq + Clone>(list: &[T]) -> Vec<RleItem<T>> {
    list.iter().fold(Vec::new(), |mut acc, x| {
        match acc.last_mut() {
            Some(RleItem::One(ref y)) if y == x => {
                let y_clone = y.clone();
                *acc.last_mut().unwrap() = RleItem::Many(2, y_clone);
            }
            Some(RleItem::Many(n, ref y)) if y == x => {
                *n += 1;
            }
            _ => {
                acc.push(RleItem::One(x.clone()));
            }
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use RleItem::*;

    #[test]
    fn test_empty() {
        assert_eq!(encode_direct::<i32>(&[]), vec![]);
        assert_eq!(encode_direct_recursive::<i32>(&[]), vec![]);
        assert_eq!(encode_direct_fold::<i32>(&[]), vec![]);
    }

    #[test]
    fn test_no_repeats() {
        let input = vec!['a', 'b', 'c'];
        let expected = vec![One('a'), One('b'), One('c')];
        assert_eq!(encode_direct(&input), expected);
        assert_eq!(encode_direct_fold(&input), expected);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(encode_direct(&[1, 1, 1, 1]), vec![Many(4, 1)]);
    }

    #[test]
    fn test_classic_example() {
        let input = vec![
            'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
        ];
        let expected = vec![
            Many(4, 'a'),
            One('b'),
            Many(2, 'c'),
            Many(2, 'a'),
            One('d'),
            Many(4, 'e'),
        ];
        assert_eq!(encode_direct(&input), expected);
        assert_eq!(encode_direct_recursive(&input), expected);
        assert_eq!(encode_direct_fold(&input), expected);
    }

    #[test]
    fn test_single() {
        assert_eq!(encode_direct(&['z']), vec![One('z')]);
    }

    #[test]
    fn test_two_same() {
        assert_eq!(encode_direct(&[5, 5]), vec![Many(2, 5)]);
    }
}
