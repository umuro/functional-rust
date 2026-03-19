#![allow(clippy::all)]
// 1019: Fallible Iterator
// Iterator that can fail: next() -> Option<Result<T,E>>

// Approach 1: Iterator yielding Result items
struct LineParser {
    lines: Vec<String>,
    index: usize,
}

impl LineParser {
    fn new(lines: Vec<&str>) -> Self {
        LineParser {
            lines: lines.into_iter().map(String::from).collect(),
            index: 0,
        }
    }
}

impl Iterator for LineParser {
    type Item = Result<i64, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.lines.len() {
            return None;
        }
        let line = &self.lines[self.index];
        self.index += 1;
        Some(
            line.trim()
                .parse::<i64>()
                .map_err(|_| format!("bad line: {}", line)),
        )
    }
}

// Approach 2: Adaptor that stops at first error
fn take_while_ok<T, E>(iter: impl Iterator<Item = Result<T, E>>) -> Result<Vec<T>, E> {
    let mut results = Vec::new();
    for item in iter {
        results.push(item?);
    }
    Ok(results)
}

// Approach 3: Process all, keeping partial results
fn process_all<T, E>(iter: impl Iterator<Item = Result<T, E>>) -> (Vec<T>, Vec<E>) {
    let mut oks = Vec::new();
    let mut errs = Vec::new();
    for item in iter {
        match item {
            Ok(v) => oks.push(v),
            Err(e) => errs.push(e),
        }
    }
    (oks, errs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid() {
        let parser = LineParser::new(vec!["1", "2", "3"]);
        let result = take_while_ok(parser);
        assert_eq!(result, Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_stops_at_error() {
        let parser = LineParser::new(vec!["1", "abc", "3"]);
        let result = take_while_ok(parser);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("bad line"));
    }

    #[test]
    fn test_empty_iterator() {
        let parser = LineParser::new(vec![]);
        assert_eq!(take_while_ok(parser), Ok(vec![]));
    }

    #[test]
    fn test_process_all_mixed() {
        let parser = LineParser::new(vec!["1", "abc", "3", "def"]);
        let (oks, errs) = process_all(parser);
        assert_eq!(oks, vec![1, 3]);
        assert_eq!(errs.len(), 2);
    }

    #[test]
    fn test_process_all_valid() {
        let parser = LineParser::new(vec!["10", "20"]);
        let (oks, errs) = process_all(parser);
        assert_eq!(oks, vec![10, 20]);
        assert!(errs.is_empty());
    }

    #[test]
    fn test_collect_shortcircuit() {
        // Standard collect on Result also works
        let parser = LineParser::new(vec!["1", "2", "3"]);
        let result: Result<Vec<i64>, String> = parser.collect();
        assert_eq!(result, Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_iterator_is_lazy() {
        let parser = LineParser::new(vec!["1", "bad", "3"]);
        // Take only first item — error not reached
        let first = parser.into_iter().next();
        assert_eq!(first, Some(Ok(1)));
    }
}
