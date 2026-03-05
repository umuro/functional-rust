//! # Zero-Copy Deserialize
//!
//! Borrowing from input data instead of copying.

/// A message that borrows from input
#[derive(Debug)]
pub struct Message<'a> {
    pub header: &'a str,
    pub body: &'a str,
}

/// Parse a message without copying
pub fn parse_message(input: &str) -> Option<Message<'_>> {
    let input = input.trim();
    let newline_pos = input.find('\n')?;
    
    Some(Message {
        header: &input[..newline_pos],
        body: &input[newline_pos + 1..],
    })
}

/// Key-value pair that borrows
#[derive(Debug)]
pub struct KeyValue<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

/// Parse key=value without copying
pub fn parse_kv(input: &str) -> Option<KeyValue<'_>> {
    let eq_pos = input.find('=')?;
    Some(KeyValue {
        key: input[..eq_pos].trim(),
        value: input[eq_pos + 1..].trim(),
    })
}

/// CSV row that borrows
#[derive(Debug)]
pub struct CsvRow<'a> {
    fields: Vec<&'a str>,
}

impl<'a> CsvRow<'a> {
    pub fn parse(line: &'a str) -> Self {
        CsvRow {
            fields: line.split(',').map(str::trim).collect(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&'a str> {
        self.fields.get(index).copied()
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

/// Parse multiple CSV rows, borrowing from input
pub fn parse_csv_rows(input: &str) -> Vec<CsvRow<'_>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(CsvRow::parse)
        .collect()
}

/// A JSON-like path reference
#[derive(Debug)]
pub struct JsonPath<'a> {
    segments: Vec<&'a str>,
}

impl<'a> JsonPath<'a> {
    pub fn parse(path: &'a str) -> Self {
        JsonPath {
            segments: path.split('.').collect(),
        }
    }

    pub fn segments(&self) -> &[&'a str] {
        &self.segments
    }
}

/// Header-body protocol message
#[derive(Debug)]
pub struct HttpLikeMessage<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub headers: Vec<(&'a str, &'a str)>,
    pub body: &'a str,
}

/// Parse HTTP-like message
pub fn parse_http_like(input: &str) -> Option<HttpLikeMessage<'_>> {
    let mut lines = input.lines();
    
    // Request line
    let request_line = lines.next()?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next()?;
    let path = parts.next()?;
    
    // Headers
    let mut headers = Vec::new();
    let mut body_start = request_line.len() + 1;
    
    for line in lines.by_ref() {
        body_start += line.len() + 1;
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(':') {
            headers.push((key.trim(), value.trim()));
        }
    }
    
    // Body is rest
    let body = if body_start < input.len() {
        &input[body_start..]
    } else {
        ""
    };
    
    Some(HttpLikeMessage {
        method,
        path,
        headers,
        body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message() {
        let input = "Hello\nWorld";
        let msg = parse_message(input).unwrap();
        assert_eq!(msg.header, "Hello");
        assert_eq!(msg.body, "World");
    }

    #[test]
    fn test_parse_kv() {
        let input = "name = Alice";
        let kv = parse_kv(input).unwrap();
        assert_eq!(kv.key, "name");
        assert_eq!(kv.value, "Alice");
    }

    #[test]
    fn test_csv_row() {
        let line = "a, b, c";
        let row = CsvRow::parse(line);
        assert_eq!(row.len(), 3);
        assert_eq!(row.get(0), Some("a"));
        assert_eq!(row.get(1), Some("b"));
    }

    #[test]
    fn test_json_path() {
        let path = "user.profile.name";
        let jp = JsonPath::parse(path);
        assert_eq!(jp.segments(), &["user", "profile", "name"]);
    }

    #[test]
    fn test_http_like() {
        let input = "GET /api/users\nContent-Type: application/json\n\n{\"id\": 1}";
        let msg = parse_http_like(input).unwrap();
        assert_eq!(msg.method, "GET");
        assert_eq!(msg.path, "/api/users");
        assert_eq!(msg.headers.len(), 1);
    }

    #[test]
    fn test_zero_copy_addresses() {
        let input = "key=value";
        let kv = parse_kv(input).unwrap();
        // Key and value point into original input
        assert!(input.as_ptr() <= kv.key.as_ptr());
        assert!(kv.key.as_ptr() < input.as_ptr().wrapping_add(input.len()));
    }
}
