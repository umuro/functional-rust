// 959: CSV Writer
// Escape quotes, handle commas/newlines in fields, produce valid CSV output

// Approach 1: Escape a single field
pub fn needs_quoting(s: &str) -> bool {
    s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r')
}

pub fn escape_field(s: &str) -> String {
    if needs_quoting(s) {
        let mut out = String::with_capacity(s.len() + 2);
        out.push('"');
        for c in s.chars() {
            if c == '"' {
                out.push('"');
                out.push('"');
            } else {
                out.push(c);
            }
        }
        out.push('"');
        out
    } else {
        s.to_string()
    }
}

// Approach 2: Write a single row
pub fn write_row(fields: &[&str]) -> String {
    fields
        .iter()
        .map(|f| escape_field(f))
        .collect::<Vec<_>>()
        .join(",")
}

pub fn write_row_owned(fields: &[String]) -> String {
    fields
        .iter()
        .map(|f| escape_field(f))
        .collect::<Vec<_>>()
        .join(",")
}

// Approach 3: Write complete CSV (rows of string slices)
pub fn write_csv(rows: &[Vec<&str>]) -> String {
    rows.iter()
        .map(|row| write_row(row))
        .collect::<Vec<_>>()
        .join("\n")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_quoting_needed() {
        assert_eq!(escape_field("hello"), "hello");
        assert_eq!(escape_field("42"), "42");
        assert_eq!(escape_field(""), "");
    }

    #[test]
    fn test_comma_quoting() {
        assert_eq!(escape_field("one, two"), "\"one, two\"");
    }

    #[test]
    fn test_quote_escaping() {
        assert_eq!(escape_field("say \"hi\""), "\"say \"\"hi\"\"\"");
    }

    #[test]
    fn test_newline_quoting() {
        assert_eq!(escape_field("line1\nline2"), "\"line1\nline2\"");
    }

    #[test]
    fn test_write_row_plain() {
        assert_eq!(write_row(&["name", "age", "city"]), "name,age,city");
    }

    #[test]
    fn test_write_row_with_special() {
        assert_eq!(
            write_row(&["Alice, Smith", "30", "Amsterdam"]),
            "\"Alice, Smith\",30,Amsterdam"
        );
    }

    #[test]
    fn test_write_csv() {
        let rows = vec![
            vec!["name", "age", "city"],
            vec!["Alice, Smith", "30", "Amsterdam"],
            vec!["Bob", "25", "say \"hi\""],
        ];
        let csv = write_csv(&rows);
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "name,age,city");
        assert_eq!(lines[1], "\"Alice, Smith\",30,Amsterdam");
        assert_eq!(lines[2], "Bob,25,\"say \"\"hi\"\"\"");
    }
}
