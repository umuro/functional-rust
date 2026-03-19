//! # CSV Parsing Pattern
//!
//! Simple CSV parser without external dependencies.

/// A parsed CSV row
pub type Row = Vec<String>;

/// CSV parse error
#[derive(Debug, PartialEq)]
pub enum CsvError {
    UnterminatedQuote(usize),
    InconsistentColumns {
        expected: usize,
        got: usize,
        line: usize,
    },
}

/// Parse a CSV string into rows
pub fn parse_csv(input: &str) -> Result<Vec<Row>, CsvError> {
    let mut rows = Vec::new();
    let mut expected_cols = None;

    for (line_num, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let row = parse_row(line, line_num)?;

        match expected_cols {
            None => expected_cols = Some(row.len()),
            Some(n) if row.len() != n => {
                return Err(CsvError::InconsistentColumns {
                    expected: n,
                    got: row.len(),
                    line: line_num,
                });
            }
            _ => {}
        }

        rows.push(row);
    }

    Ok(rows)
}

/// Parse a single CSV row
fn parse_row(line: &str, line_num: usize) -> Result<Row, CsvError> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    current.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                current.push(ch);
            }
        } else {
            match ch {
                '"' => in_quotes = true,
                ',' => {
                    fields.push(current.trim().to_string());
                    current = String::new();
                }
                _ => current.push(ch),
            }
        }
    }

    if in_quotes {
        return Err(CsvError::UnterminatedQuote(line_num));
    }

    fields.push(current.trim().to_string());
    Ok(fields)
}

/// Format rows as CSV
pub fn format_csv(rows: &[Row]) -> String {
    rows.iter()
        .map(|row| {
            row.iter()
                .map(|field| {
                    if field.contains(',') || field.contains('"') || field.contains('\n') {
                        format!("\"{}\"", field.replace('"', "\"\""))
                    } else {
                        field.clone()
                    }
                })
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Parse CSV with headers, returning maps
pub fn parse_csv_with_headers(
    input: &str,
) -> Result<Vec<std::collections::HashMap<String, String>>, CsvError> {
    let rows = parse_csv(input)?;
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let headers = &rows[0];
    let mut result = Vec::new();

    for row in rows.iter().skip(1) {
        let mut map = std::collections::HashMap::new();
        for (i, value) in row.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                map.insert(header.clone(), value.clone());
            }
        }
        result.push(map);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_csv() {
        let input = "a,b,c\n1,2,3\n4,5,6";
        let rows = parse_csv(input).unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], vec!["a", "b", "c"]);
        assert_eq!(rows[1], vec!["1", "2", "3"]);
    }

    #[test]
    fn test_quoted_field() {
        let input = r#"name,value
"hello, world",42"#;
        let rows = parse_csv(input).unwrap();
        assert_eq!(rows[1][0], "hello, world");
    }

    #[test]
    fn test_escaped_quote() {
        let input = "text\n\"say \"\"hello\"\"\"";
        let rows = parse_csv(input).unwrap();
        assert_eq!(rows[1][0], "say \"hello\"");
    }

    #[test]
    fn test_inconsistent_columns() {
        let input = "a,b,c\n1,2";
        let result = parse_csv(input);
        assert!(matches!(result, Err(CsvError::InconsistentColumns { .. })));
    }

    #[test]
    fn test_format_csv() {
        let rows = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ];
        let output = format_csv(&rows);
        assert_eq!(output, "a,b\n1,2");
    }

    #[test]
    fn test_with_headers() {
        let input = "name,age\nAlice,30\nBob,25";
        let records = parse_csv_with_headers(input).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].get("name").unwrap(), "Alice");
        assert_eq!(records[0].get("age").unwrap(), "30");
    }
}
