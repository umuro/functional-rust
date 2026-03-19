// 958: CSV Parser
// OCaml uses mutable Buffer + state ref; Rust uses an enum state machine with chars iterator

// Approach 1: Simple split (no quote handling)
pub fn split_simple(line: &str) -> Vec<&str> {
    line.split(',').collect()
}

// Approach 2: Full CSV state machine with quote handling
#[derive(Debug, PartialEq)]
enum State {
    Normal,
    InQuote,
    AfterQuote,
}

pub fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut state = State::Normal;

    for c in line.chars() {
        match (&state, c) {
            (State::Normal, '"') => {
                state = State::InQuote;
            }
            (State::Normal, ',') => {
                fields.push(current.clone());
                current.clear();
            }
            (State::Normal, c) => {
                current.push(c);
            }
            (State::InQuote, '"') => {
                state = State::AfterQuote;
            }
            (State::InQuote, c) => {
                current.push(c);
            }
            (State::AfterQuote, '"') => {
                // Escaped quote: "" inside quoted field
                current.push('"');
                state = State::InQuote;
            }
            (State::AfterQuote, ',') => {
                fields.push(current.clone());
                current.clear();
                state = State::Normal;
            }
            (State::AfterQuote, _) => {
                state = State::Normal;
            }
        }
    }
    // Push last field
    fields.push(current);
    fields
}

// Approach 3: Parse multiple rows
pub fn parse_csv(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .filter(|line| !line.is_empty())
        .map(parse_csv_line)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_split() {
        assert_eq!(split_simple("a,b,c"), vec!["a", "b", "c"]);
        assert_eq!(split_simple("one"), vec!["one"]);
    }

    #[test]
    fn test_quoted_fields() {
        assert_eq!(
            parse_csv_line("\"hello\",\"world\",plain"),
            vec!["hello", "world", "plain"]
        );
    }

    #[test]
    fn test_comma_inside_quotes() {
        assert_eq!(
            parse_csv_line("\"one, two\",three"),
            vec!["one, two", "three"]
        );
    }

    #[test]
    fn test_escaped_quotes() {
        assert_eq!(
            parse_csv_line("\"say \"\"hi\"\"\",end"),
            vec!["say \"hi\"", "end"]
        );
    }

    #[test]
    fn test_empty_fields() {
        assert_eq!(parse_csv_line(",,"), vec!["", "", ""]);
        assert_eq!(parse_csv_line("a,,c"), vec!["a", "", "c"]);
    }

    #[test]
    fn test_mixed() {
        assert_eq!(
            parse_csv_line("name,\"Alice, Bob\",42"),
            vec!["name", "Alice, Bob", "42"]
        );
    }

    #[test]
    fn test_multi_row() {
        let csv = "a,b,c\n1,2,3\n\"x,y\",z,w";
        let rows = parse_csv(csv);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], vec!["a", "b", "c"]);
        assert_eq!(rows[2], vec!["x,y", "z", "w"]);
    }
}
