#![allow(clippy::all)]
// Example 170: Error Recovery
// Parser error messages with position info and expected tokens

type ParseResult<'a, T> = Result<(T, &'a str, Position), ParseError>;

#[derive(Debug, Clone, PartialEq)]
struct Position {
    offset: usize,
    line: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct ParseError {
    pos: Position,
    expected: Vec<String>,
    got: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Error at line {}, col {}: expected {}, got {}",
            self.pos.line,
            self.pos.col,
            self.expected.join(" or "),
            self.got
        )
    }
}

fn init_pos() -> Position {
    Position {
        offset: 0,
        line: 1,
        col: 1,
    }
}

fn advance_pos(pos: &Position, c: char) -> Position {
    if c == '\n' {
        Position {
            offset: pos.offset + 1,
            line: pos.line + 1,
            col: 1,
        }
    } else {
        Position {
            offset: pos.offset + 1,
            line: pos.line,
            col: pos.col + 1,
        }
    }
}

// ============================================================
// Approach 1: Satisfy with position tracking
// ============================================================

fn satisfy_pos<'a>(
    pred: impl Fn(char) -> bool,
    desc: &str,
    input: &'a str,
    pos: &Position,
) -> ParseResult<'a, char> {
    match input.chars().next() {
        Some(c) if pred(c) => Ok((c, &input[c.len_utf8()..], advance_pos(pos, c))),
        Some(c) => Err(ParseError {
            pos: pos.clone(),
            expected: vec![desc.to_string()],
            got: format!("'{}'", c),
        }),
        None => Err(ParseError {
            pos: pos.clone(),
            expected: vec![desc.to_string()],
            got: "EOF".to_string(),
        }),
    }
}

fn tag_pos<'a>(expected: &str, input: &'a str, pos: &Position) -> ParseResult<'a, &'a str> {
    if input.starts_with(expected) {
        let mut new_pos = pos.clone();
        for c in expected.chars() {
            new_pos = advance_pos(&new_pos, c);
        }
        Ok((&input[..expected.len()], &input[expected.len()..], new_pos))
    } else {
        let got_len = expected.len().min(input.len());
        Err(ParseError {
            pos: pos.clone(),
            expected: vec![format!("\"{}\"", expected)],
            got: format!("\"{}\"", &input[..got_len]),
        })
    }
}

// ============================================================
// Approach 2: Error merging for alternatives
// ============================================================

fn alt_pos<'a, T>(
    r1: ParseResult<'a, T>,
    r2: impl FnOnce() -> ParseResult<'a, T>,
) -> ParseResult<'a, T> {
    match r1 {
        Ok(r) => Ok(r),
        Err(e1) => match r2() {
            Ok(r) => Ok(r),
            Err(e2) => {
                if e1.pos.offset == e2.pos.offset {
                    let mut expected = e1.expected;
                    expected.extend(e2.expected);
                    Err(ParseError {
                        pos: e1.pos,
                        expected,
                        got: e1.got,
                    })
                } else if e1.pos.offset > e2.pos.offset {
                    Err(e1)
                } else {
                    Err(e2)
                }
            }
        },
    }
}

// ============================================================
// Approach 3: Error recovery — skip to sync point
// ============================================================

fn recover_until<'a, T>(
    sync: impl Fn(char) -> bool,
    result: ParseResult<'a, T>,
    input: &'a str,
    pos: &Position,
) -> Result<(Option<T>, &'a str, Position), ParseError> {
    match result {
        Ok((v, rest, new_pos)) => Ok((Some(v), rest, new_pos)),
        Err(_) => {
            let mut remaining = input;
            let mut current_pos = pos.clone();
            while let Some(c) = remaining.chars().next() {
                if sync(c) {
                    return Ok((None, remaining, current_pos));
                }
                remaining = &remaining[c.len_utf8()..];
                current_pos = advance_pos(&current_pos, c);
            }
            Ok((None, "", current_pos))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_position() {
        let pos = init_pos();
        let err = satisfy_pos(|c| c.is_ascii_digit(), "digit", "abc", &pos).unwrap_err();
        assert_eq!(err.pos.line, 1);
        assert_eq!(err.pos.col, 1);
        assert_eq!(err.expected, vec!["digit"]);
        assert_eq!(err.got, "'a'");
    }

    #[test]
    fn test_error_merge() {
        let pos = init_pos();
        let r1 = satisfy_pos(|c| c.is_ascii_digit(), "digit", "!", &pos);
        let err = alt_pos(r1, || {
            satisfy_pos(|c| c.is_ascii_alphabetic(), "letter", "!", &pos)
        })
        .unwrap_err();
        assert_eq!(err.expected.len(), 2);
        assert!(err.expected.contains(&"digit".to_string()));
        assert!(err.expected.contains(&"letter".to_string()));
    }

    #[test]
    fn test_position_tracking() {
        let pos = init_pos();
        let (_, _, new_pos) = tag_pos("ab", "ab\ncd", &pos).unwrap();
        assert_eq!(new_pos.line, 1);
        assert_eq!(new_pos.col, 3);
    }

    #[test]
    fn test_newline_tracking() {
        let pos = init_pos();
        let (_, rest, new_pos) = tag_pos("ab\n", "ab\ncd", &pos).unwrap();
        assert_eq!(new_pos.line, 2);
        assert_eq!(new_pos.col, 1);
        assert_eq!(rest, "cd");
    }

    #[test]
    fn test_recovery() {
        let pos = init_pos();
        let bad = satisfy_pos(|c| c.is_ascii_digit(), "digit", "abc;rest", &pos);
        let (val, rest, _) = recover_until(|c| c == ';', bad, "abc;rest", &pos).unwrap();
        assert!(val.is_none());
        assert_eq!(rest, ";rest");
    }

    #[test]
    fn test_display_error() {
        let err = ParseError {
            pos: Position {
                offset: 5,
                line: 2,
                col: 3,
            },
            expected: vec!["digit".into(), "letter".into()],
            got: "'!'".into(),
        };
        assert_eq!(
            format!("{}", err),
            "Error at line 2, col 3: expected digit or letter, got '!'"
        );
    }
}
