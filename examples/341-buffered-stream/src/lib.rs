// 341: Buffered Stream
// BufReader/BufWriter wrapping for efficient I/O

use std::io::{self, BufRead, BufReader, BufWriter, Cursor, Write};

// Approach 1: BufReader for efficient reading
fn count_lines(input: &[u8]) -> usize {
    let reader = BufReader::new(input);
    reader.lines().count()
}

fn read_lines(input: &[u8]) -> Vec<String> {
    let reader = BufReader::new(input);
    reader.lines().filter_map(|l| l.ok()).collect()
}

// Approach 2: BufWriter for efficient writing
fn write_lines(lines: &[&str]) -> Vec<u8> {
    let mut output = Vec::new();
    {
        let mut writer = BufWriter::new(&mut output);
        for line in lines {
            writeln!(writer, "{}", line).unwrap();
        }
        writer.flush().unwrap();
    }
    output
}

// Approach 3: String building with buffered writes
fn build_csv(headers: &[&str], rows: &[Vec<String>]) -> String {
    let mut buf = Vec::new();
    {
        let mut writer = BufWriter::new(&mut buf);
        writeln!(writer, "{}", headers.join(",")).unwrap();
        for row in rows {
            writeln!(writer, "{}", row.join(",")).unwrap();
        }
        writer.flush().unwrap();
    }
    String::from_utf8(buf).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines(b"a\nb\nc\n"), 3);
        assert_eq!(count_lines(b""), 0);
    }

    #[test]
    fn test_read_lines() {
        let lines = read_lines(b"hello\nworld\n");
        assert_eq!(lines, vec!["hello", "world"]);
    }

    #[test]
    fn test_write_lines() {
        let output = write_lines(&["hello", "world"]);
        let s = String::from_utf8(output).unwrap();
        assert!(s.contains("hello"));
        assert!(s.contains("world"));
    }

    #[test]
    fn test_csv() {
        let csv = build_csv(&["a", "b"], &[vec!["1".into(), "2".into()]]);
        assert!(csv.starts_with("a,b\n"));
        assert!(csv.contains("1,2"));
    }
}
