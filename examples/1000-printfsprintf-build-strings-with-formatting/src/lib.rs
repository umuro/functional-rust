//! Printf.sprintf — Build Strings with Formatting
//!
//! OCaml's `Printf.sprintf` creates formatted strings without printing.
//! Rust's `format!` macro is the direct equivalent: it returns a `String`.

/// Format a record row with fixed-width columns.
///
/// Mirrors OCaml: `Printf.sprintf "%-15s | %3d | %6.2f" name age score`
pub fn format_record(name: &str, age: u32, score: f64) -> String {
    format!("{:<15} | {:>3} | {:>6.2}", name, age, score)
}

/// Build the table header line.
pub fn format_header() -> String {
    format!("{:<15} | {:>3} | {:>6}", "Name", "Age", "Score")
}

/// Build a separator line matching the width of `header`.
pub fn format_separator(header: &str) -> String {
    "-".repeat(header.len())
}

/// Render a full table for a list of `(name, age, score)` records.
///
/// Returns a `Vec<String>` (one line per row) so callers choose how to
/// emit the output — no `println!` in library code.
pub fn render_table(records: &[(&str, u32, f64)]) -> Vec<String> {
    let header = format_header();
    let sep = format_separator(&header);

    let mut lines = vec![header, sep];
    lines.extend(records.iter().map(|(n, a, s)| format_record(n, *a, *s)));
    lines
}

/// Zero-pad an integer to a given width — common sprintf use-case.
pub fn zero_pad(n: u32, width: usize) -> String {
    format!("{:0>width$}", n, width = width)
}

/// Format a percentage with a configurable number of decimal places.
pub fn format_percent(value: f64, decimals: usize) -> String {
    format!("{:.prec$}%", value * 100.0, prec = decimals)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- format_record ---

    #[test]
    fn test_format_record_basic() {
        let result = format_record("Alice", 30, 95.5);
        assert_eq!(result, "Alice           |  30 |  95.50");
    }

    #[test]
    fn test_format_record_short_name() {
        let result = format_record("Bob", 25, 87.3);
        assert_eq!(result, "Bob             |  25 |  87.30");
    }

    #[test]
    fn test_format_record_long_name_overflow() {
        // Names longer than 15 chars are not truncated — they overflow the column
        let result = format_record("Alexander Hamilton", 42, 100.0);
        assert!(result.starts_with("Alexander Hamilton"));
    }

    #[test]
    fn test_format_record_zero_score() {
        let result = format_record("Dana", 20, 0.0);
        assert_eq!(result, "Dana            |  20 |   0.00");
    }

    // --- format_header ---

    #[test]
    fn test_format_header_contains_columns() {
        let h = format_header();
        assert!(h.contains("Name"));
        assert!(h.contains("Age"));
        assert!(h.contains("Score"));
    }

    #[test]
    fn test_format_header_width() {
        let h = format_header();
        // "Name" padded to 15, " | ", "Age" padded to 3, " | ", "Score" padded to 6
        // = 15 + 3 + 3 + 3 + 6 = 30
        assert_eq!(h.len(), 30);
    }

    // --- format_separator ---

    #[test]
    fn test_separator_matches_header_length() {
        let h = format_header();
        let sep = format_separator(&h);
        assert_eq!(sep.len(), h.len());
        assert!(sep.chars().all(|c| c == '-'));
    }

    #[test]
    fn test_separator_empty_header() {
        assert_eq!(format_separator(""), "");
    }

    // --- render_table ---

    #[test]
    fn test_render_table_empty() {
        let lines = render_table(&[]);
        assert_eq!(lines.len(), 2); // header + separator only
    }

    #[test]
    fn test_render_table_single_record() {
        let lines = render_table(&[("Alice", 30, 95.5)]);
        assert_eq!(lines.len(), 3);
        assert!(lines[2].contains("Alice"));
    }

    #[test]
    fn test_render_table_multiple_records() {
        let records = [("Alice", 30, 95.5), ("Bob", 25, 87.3), ("Carol", 28, 92.1)];
        let lines = render_table(&records);
        assert_eq!(lines.len(), 5);
        assert!(lines[2].contains("Alice"));
        assert!(lines[3].contains("Bob"));
        assert!(lines[4].contains("Carol"));
    }

    #[test]
    fn test_render_table_row_widths_consistent() {
        let records = [("Alice", 30, 95.5), ("Bob", 25, 87.3)];
        let lines = render_table(&records);
        // All data rows should have the same length as the header
        let header_len = lines[0].len();
        for line in lines.iter().skip(2) {
            assert_eq!(line.len(), header_len);
        }
    }

    // --- zero_pad ---

    #[test]
    fn test_zero_pad_basic() {
        assert_eq!(zero_pad(7, 3), "007");
    }

    #[test]
    fn test_zero_pad_no_padding_needed() {
        assert_eq!(zero_pad(42, 2), "42");
    }

    #[test]
    fn test_zero_pad_zero() {
        assert_eq!(zero_pad(0, 4), "0000");
    }

    #[test]
    fn test_zero_pad_exact_width() {
        assert_eq!(zero_pad(123, 3), "123");
    }

    // --- format_percent ---

    #[test]
    fn test_format_percent_two_decimals() {
        assert_eq!(format_percent(0.755, 2), "75.50%");
    }

    #[test]
    fn test_format_percent_zero_decimals() {
        assert_eq!(format_percent(0.5, 0), "50%");
    }

    #[test]
    fn test_format_percent_full() {
        assert_eq!(format_percent(1.0, 1), "100.0%");
    }

    #[test]
    fn test_format_percent_zero() {
        assert_eq!(format_percent(0.0, 2), "0.00%");
    }
}
