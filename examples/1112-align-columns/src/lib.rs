//! Align Columns — format tabular text with left, right, or center justification.
//!
//! Fields within each line are delimited by a single character (e.g. `$`).
//! Each column is padded to the width of the widest field in that column,
//! with at least one space separating columns.

/// Alignment direction for column formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

/// Parse delimited text into rows of fields.
///
/// Splits on newlines first, then on `delimiter` within each line.
/// Returns `&str` slices into the original string — no allocation per field.
pub fn parse_fields(text: &str, delimiter: char) -> Vec<Vec<&str>> {
    text.lines()
        .map(|line| line.split(delimiter).collect())
        .collect()
}

/// Compute the maximum field width for each column across all rows.
///
/// The resulting `Vec` has one entry per column; missing fields in shorter
/// rows simply contribute nothing to the maximum.
pub fn column_widths(rows: &[Vec<&str>]) -> Vec<usize> {
    let num_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    (0..num_cols)
        .map(|col| {
            rows.iter()
                .filter_map(|row| row.get(col))
                .map(|word| word.len())
                .max()
                .unwrap_or(0)
        })
        .collect()
}

/// Format a single row of fields using the given column widths and alignment.
///
/// Each field is padded to its column width, then followed by a single space
/// separator (matching the OCaml output).
pub fn format_row(row: &[&str], widths: &[usize], alignment: Alignment) -> String {
    row.iter()
        .enumerate()
        .map(|(i, word)| {
            let w = widths.get(i).copied().unwrap_or(word.len());
            match alignment {
                Alignment::Left => format!("{word:<w$} "),
                Alignment::Right => format!("{word:>w$} "),
                Alignment::Center => format!("{word:^w$} "),
            }
        })
        .collect()
}

/// Align all columns in `text` using the specified `alignment`.
///
/// This is the main entry point: parses the text, computes column widths,
/// and returns the formatted string.
pub fn align_columns(text: &str, delimiter: char, alignment: Alignment) -> String {
    let rows = parse_fields(text, delimiter);
    let widths = column_widths(&rows);
    rows.iter()
        .map(|row| format_row(row, &widths, alignment))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Recursive implementation: computes column widths by walking rows one at a
/// time, threading the accumulated widths through tail recursion.
///
/// Mirrors the OCaml `List.iter` / `Array.iteri` pattern explicitly.
pub fn column_widths_recursive(rows: &[Vec<&str>]) -> Vec<usize> {
    fn update_widths(mut widths: Vec<usize>, row: &[&str]) -> Vec<usize> {
        for (i, word) in row.iter().enumerate() {
            if i >= widths.len() {
                widths.push(word.len());
            } else {
                widths[i] = widths[i].max(word.len());
            }
        }
        widths
    }

    rows.iter()
        .fold(Vec::new(), |acc, row| update_widths(acc, row))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
Given$a$text$file$of$many$lines,$where$fields$within$a$line$\
are$delineated$by$a$single$'dollar'$character,$write$a$program\
that$aligns$each$column$of$fields$by$ensuring$that$words$in$each$\
column$are$separated$by$at$least$one$space.\
Further,$allow$for$each$word$in$a$column$to$be$either$left$\
justified,$right$justified,$or$center$justified$within$its$column.";

    // ── parse_fields ─────────────────────────────────────────────────────────

    #[test]
    fn test_parse_single_line_single_field() {
        let rows = parse_fields("hello", '$');
        assert_eq!(rows, vec![vec!["hello"]]);
    }

    #[test]
    fn test_parse_single_line_multiple_fields() {
        let rows = parse_fields("a$b$c", '$');
        assert_eq!(rows, vec![vec!["a", "b", "c"]]);
    }

    #[test]
    fn test_parse_multiple_lines() {
        let rows = parse_fields("a$b\nc$d", '$');
        assert_eq!(rows, vec![vec!["a", "b"], vec!["c", "d"]]);
    }

    #[test]
    fn test_parse_empty_string() {
        let rows = parse_fields("", '$');
        // Rust's `str::lines()` yields 0 lines for an empty string
        assert_eq!(rows.len(), 0);
    }

    // ── column_widths ─────────────────────────────────────────────────────────

    #[test]
    fn test_column_widths_uniform() {
        let rows = vec![vec!["ab", "cd"], vec!["ef", "gh"]];
        assert_eq!(column_widths(&rows), vec![2, 2]);
    }

    #[test]
    fn test_column_widths_ragged() {
        // second row is shorter — column 1 width still from first row
        let rows = vec![vec!["hello", "world"], vec!["hi"]];
        assert_eq!(column_widths(&rows), vec![5, 5]);
    }

    #[test]
    fn test_column_widths_single_cell() {
        let rows = vec![vec!["rust"]];
        assert_eq!(column_widths(&rows), vec![4]);
    }

    #[test]
    fn test_column_widths_empty() {
        let rows: Vec<Vec<&str>> = vec![];
        assert_eq!(column_widths(&rows), Vec::<usize>::new());
    }

    // ── format_row ────────────────────────────────────────────────────────────

    #[test]
    fn test_format_row_left() {
        let widths = vec![5, 5];
        let row = vec!["hi", "bye"];
        // "hi" → "hi   " (width 5) + " " = 6 chars; "bye" → "bye  " (width 5) + " " = 6 chars
        assert_eq!(format_row(&row, &widths, Alignment::Left), "hi    bye   ");
    }

    #[test]
    fn test_format_row_right() {
        let widths = vec![5, 5];
        let row = vec!["hi", "bye"];
        // "   hi" (width 5) + " " = 6 chars; "  bye" (width 5) + " " = 6 chars
        assert_eq!(format_row(&row, &widths, Alignment::Right), "   hi   bye ");
    }

    #[test]
    fn test_format_row_center_even_pad() {
        // "hi" in width 4 → 1 pad left + 1 pad right
        let widths = vec![4];
        let row = vec!["hi"];
        assert_eq!(format_row(&row, &widths, Alignment::Center), " hi  ");
    }

    #[test]
    fn test_format_row_center_odd_pad() {
        // "hi" (2 chars) in width 5 → 1 left, 2 right (Rust puts extra on right)
        let widths = vec![5];
        let row = vec!["hi"];
        let result = format_row(&row, &widths, Alignment::Center);
        // total = 5 (column) + 1 (trailing space) = 6 chars
        assert_eq!(result.len(), 6);
        assert!(result.contains("hi"));
    }

    // ── align_columns (integration) ──────────────────────────────────────────

    #[test]
    fn test_align_left_simple() {
        let text = "aa$b\ncc$ddd";
        let result = align_columns(text, '$', Alignment::Left);
        let lines: Vec<&str> = result.lines().collect();
        // column 0 width = 2, column 1 width = 3
        assert_eq!(lines[0], "aa b   ");
        assert_eq!(lines[1], "cc ddd ");
    }

    #[test]
    fn test_align_right_simple() {
        let text = "aa$b\ncc$ddd";
        let result = align_columns(text, '$', Alignment::Right);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines[0], "aa   b ");
        assert_eq!(lines[1], "cc ddd ");
    }

    #[test]
    fn test_align_center_simple() {
        let text = "a$bbb\ncc$d";
        let result = align_columns(text, '$', Alignment::Center);
        // column 0 width = 2, column 1 width = 3
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 2);
        // each line: field centred in its column width, plus trailing space
        for line in &lines {
            assert!(!line.is_empty());
        }
    }

    #[test]
    fn test_align_single_column() {
        let text = "short\nverylongword\nmed";
        let result = align_columns(text, '$', Alignment::Left);
        let lines: Vec<&str> = result.lines().collect();
        // only one column — each line should just be the word + trailing space
        assert_eq!(lines[0], "short        ");
        assert_eq!(lines[1], "verylongword ");
        assert_eq!(lines[2], "med          ");
    }

    // ── column_widths_recursive matches column_widths ─────────────────────────

    #[test]
    fn test_recursive_widths_match_iterative() {
        let rows = parse_fields("Given$a$text\nfile$of$many$lines", '$');
        assert_eq!(column_widths(&rows), column_widths_recursive(&rows));
    }

    #[test]
    fn test_recursive_widths_ragged_rows() {
        let rows = vec![vec!["longer", "x"], vec!["a", "medium", "tiny"], vec!["b"]];
        assert_eq!(column_widths(&rows), column_widths_recursive(&rows));
    }

    // ── all alignments preserve line count ───────────────────────────────────

    #[test]
    fn test_all_alignments_same_line_count() {
        let left = align_columns(SAMPLE, '$', Alignment::Left);
        let right = align_columns(SAMPLE, '$', Alignment::Right);
        let center = align_columns(SAMPLE, '$', Alignment::Center);
        assert_eq!(left.lines().count(), right.lines().count());
        assert_eq!(left.lines().count(), center.lines().count());
    }
}
