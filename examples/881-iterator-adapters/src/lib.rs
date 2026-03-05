// Example 087: Iterator Adapters
// Chaining map/filter/flat_map/take/skip

// === Approach 1: Basic map/filter pipeline ===
fn pipeline(data: &[i32]) -> Vec<String> {
    data.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * x)
        .map(|x| x.to_string())
        .collect()
}

fn flat_map_example(data: &[&str]) -> Vec<String> {
    data.iter()
        .flat_map(|s| s.split_whitespace())
        .map(String::from)
        .collect()
}

// === Approach 2: Take/Skip/Chain ===
fn take_skip_demo(data: &[i32]) -> Vec<i32> {
    data.iter()
        .copied()
        .filter(|x| x % 2 == 0)
        .map(|x| x * 3)
        .take(5)
        .collect()
}

fn skip_demo(data: &[i32], n: usize) -> Vec<i32> {
    data.iter().copied().skip(n).collect()
}

fn chain_demo(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().chain(b.iter()).copied().collect()
}

// === Approach 3: Complex chained pipelines ===
fn word_lengths(text: &str) -> Vec<usize> {
    let mut lengths: Vec<usize> = text.split_whitespace()
        .map(|w| w.len())
        .collect();
    lengths.sort();
    lengths
}

fn top_n<T, B: Ord>(data: &[T], n: usize, transform: impl Fn(&T) -> B) -> Vec<B> {
    let mut results: Vec<B> = data.iter().map(transform).collect();
    results.sort_by(|a, b| b.cmp(a));
    results.truncate(n);
    results
}

// Enumerate + filter pattern
fn indexed_evens(data: &[i32]) -> Vec<(usize, i32)> {
    data.iter()
        .enumerate()
        .filter(|(_, &v)| v % 2 == 0)
        .map(|(i, &v)| (i, v))
        .collect()
}

// Inspect for debugging (side-effect adapter)
fn debug_pipeline(data: &[i32]) -> Vec<i32> {
    data.iter()
        .copied()
        .inspect(|x| eprintln!("before filter: {}", x))
        .filter(|x| x % 2 == 0)
        .inspect(|x| eprintln!("after filter: {}", x))
        .map(|x| x * 10)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline() {
        assert_eq!(pipeline(&[3, -1, 4, -5, 2]), vec!["9", "16", "4"]);
    }

    #[test]
    fn test_flat_map() {
        let result = flat_map_example(&["hello world", "foo bar"]);
        assert_eq!(result, vec!["hello", "world", "foo", "bar"]);
    }

    #[test]
    fn test_take_skip() {
        let data: Vec<i32> = (1..=14).collect();
        assert_eq!(take_skip_demo(&data), vec![6, 12, 18, 24, 30]);
    }

    #[test]
    fn test_skip() {
        assert_eq!(skip_demo(&[1,2,3,4,5], 3), vec![4, 5]);
    }

    #[test]
    fn test_chain() {
        assert_eq!(chain_demo(&[1,2], &[3,4]), vec![1,2,3,4]);
    }

    #[test]
    fn test_word_lengths() {
        // "the"=3, "quick"=5, "brown"=5, "fox"=3 → sorted: [3, 3, 5, 5]
        assert_eq!(word_lengths("the quick brown fox"), vec![3, 3, 5, 5]);
    }

    #[test]
    fn test_top_n() {
        assert_eq!(top_n(&[1,5,3,2,4], 3, |x| x * x), vec![25, 16, 9]);
    }

    #[test]
    fn test_indexed_evens() {
        assert_eq!(indexed_evens(&[1,2,3,4,5,6]), vec![(1,2), (3,4), (5,6)]);
    }

    #[test]
    fn test_empty_pipeline() {
        assert_eq!(pipeline(&[]), Vec::<String>::new());
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(pipeline(&[-1, -2, -3]), Vec::<String>::new());
    }
}
