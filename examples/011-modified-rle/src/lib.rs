/// Modified Run-Length Encoding (99 Problems #11)
///
/// Modify the RLE result so that elements without duplicates are simply
/// copied into the result list, while duplicates are encoded as (count, elem).

#[derive(Debug, PartialEq, Clone)]
pub enum RleItem<T> {
    One(T),
    Many(usize, T),
}

// ── Idiomatic Rust: iterator-based ──────────────────────────────────────────

pub fn encode_modified<T: PartialEq + Clone>(list: &[T]) -> Vec<RleItem<T>> {
    if list.is_empty() {
        return vec![];
    }
    let mut result = Vec::new();
    let mut count = 1;
    for i in 1..=list.len() {
        if i < list.len() && list[i] == list[i - 1] {
            count += 1;
        } else {
            result.push(if count == 1 {
                RleItem::One(list[i - 1].clone())
            } else {
                RleItem::Many(count, list[i - 1].clone())
            });
            count = 1;
        }
    }
    result
}

// ── Functional/recursive style ──────────────────────────────────────────────

pub fn encode_modified_recursive<T: PartialEq + Clone>(list: &[T]) -> Vec<RleItem<T>> {
    fn pack_run<T: PartialEq + Clone>(list: &[T]) -> (&[T], &[T]) {
        if list.len() <= 1 {
            return (list, &[]);
        }
        let first = &list[0];
        let end = list.iter().position(|x| x != first).unwrap_or(list.len());
        (&list[..end], &list[end..])
    }

    fn aux<T: PartialEq + Clone>(list: &[T], acc: &mut Vec<RleItem<T>>) {
        if list.is_empty() {
            return;
        }
        let (run, rest) = pack_run(list);
        acc.push(if run.len() == 1 {
            RleItem::One(run[0].clone())
        } else {
            RleItem::Many(run.len(), run[0].clone())
        });
        aux(rest, acc);
    }

    let mut result = Vec::new();
    aux(list, &mut result);
    result
}

// ── Using chunk_by (functional, slice-based) ────────────────────────────────

pub fn encode_modified_chunks<T: PartialEq + Clone>(list: &[T]) -> Vec<RleItem<T>> {
    // Manual chunk-by since slice::chunk_by is nightly
    let mut result = Vec::new();
    let mut i = 0;
    while i < list.len() {
        let start = i;
        while i < list.len() && list[i] == list[start] {
            i += 1;
        }
        let len = i - start;
        result.push(if len == 1 {
            RleItem::One(list[start].clone())
        } else {
            RleItem::Many(len, list[start].clone())
        });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use RleItem::*;

    #[test]
    fn test_empty() {
        assert_eq!(encode_modified::<char>(&[]), vec![]);
        assert_eq!(encode_modified_recursive::<char>(&[]), vec![]);
    }

    #[test]
    fn test_no_duplicates() {
        assert_eq!(
            encode_modified(&['a', 'b', 'c']),
            vec![One('a'), One('b'), One('c')]
        );
    }

    #[test]
    fn test_all_same() {
        assert_eq!(encode_modified(&['x', 'x', 'x']), vec![Many(3, 'x')]);
    }

    #[test]
    fn test_mixed() {
        let input = vec!['a', 'a', 'a', 'b', 'c', 'c', 'd', 'd', 'd', 'd'];
        let expected = vec![Many(3, 'a'), One('b'), Many(2, 'c'), Many(4, 'd')];
        assert_eq!(encode_modified(&input), expected);
        assert_eq!(encode_modified_recursive(&input), expected);
        assert_eq!(encode_modified_chunks(&input), expected);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(encode_modified(&[42]), vec![One(42)]);
    }

    #[test]
    fn test_strings() {
        let input = vec!["hi", "hi", "bye"];
        assert_eq!(encode_modified(&input), vec![Many(2, "hi"), One("bye")]);
    }
}
