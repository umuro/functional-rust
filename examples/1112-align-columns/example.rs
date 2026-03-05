//! Align Columns — format tabular text with left, right, or center justification.

/// Alignment direction for column formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

/// Parse delimited text into rows of fields.
pub fn parse_fields(text: &str, delimiter: char) -> Vec<Vec<&str>> {
    text.lines()
        .map(|line| line.split(delimiter).collect())
        .collect()
}

/// Compute the maximum field width for each column across all rows.
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
pub fn align_columns(text: &str, delimiter: char, alignment: Alignment) -> String {
    let rows = parse_fields(text, delimiter);
    let widths = column_widths(&rows);
    rows.iter()
        .map(|row| format_row(row, &widths, alignment))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let input = "\
Given$a$text$file$of$many$lines,$where$fields$within$a$line$
are$delineated$by$a$single$'dollar'$character,$write$a$program
that$aligns$each$column$of$fields$by$ensuring$that$words$in$each$
column$are$separated$by$at$least$one$space.
Further,$allow$for$each$word$in$a$column$to$be$either$left$
justified,$right$justified,$or$center$justified$within$its$column.";

    println!("=== Left aligned ===");
    println!("{}", align_columns(input, '$', Alignment::Left));

    println!("\n=== Right aligned ===");
    println!("{}", align_columns(input, '$', Alignment::Right));

    println!("\n=== Center aligned ===");
    println!("{}", align_columns(input, '$', Alignment::Center));
}

/* Output (abridged):
=== Left aligned ===
Given      a          text       file       of         many       lines,     where      fields  within  a     line
are        delineated by         a          single     'dollar'   character, write      a       program
...

=== Right aligned ===
     Given          a       text       file         of       many     lines,      where  fields  within  a  line
       are delineated         by          a     single   'dollar' character,      write       a program
...

=== Center aligned ===
  Given      a      text     file      of      many    lines,   where   fields  within   a    line
   are   delineated   by      a     single  'dollar' character, write     a    program
...
*/
