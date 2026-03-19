//! # Streaming Parser Pattern
//!
//! Process data incrementally without loading everything into memory.

/// Parser state
#[derive(Debug)]
pub enum ParseState {
    Ready,
    InHeader,
    InBody { remaining: usize },
    Complete,
    Error(String),
}

/// Streaming message parser
pub struct StreamingParser {
    state: ParseState,
    header: Vec<u8>,
    body: Vec<u8>,
    body_length: usize,
}

impl StreamingParser {
    pub fn new() -> Self {
        StreamingParser {
            state: ParseState::Ready,
            header: Vec::new(),
            body: Vec::new(),
            body_length: 0,
        }
    }

    /// Feed data to the parser, returns bytes consumed
    pub fn feed(&mut self, data: &[u8]) -> usize {
        let mut consumed = 0;

        for &byte in data {
            consumed += 1;
            match &self.state {
                ParseState::Ready => {
                    if byte == b':' {
                        // Header format: "length:body"
                        let len_str = String::from_utf8_lossy(&self.header);
                        match len_str.parse::<usize>() {
                            Ok(len) => {
                                self.body_length = len;
                                self.state = ParseState::InBody { remaining: len };
                            }
                            Err(_) => {
                                self.state = ParseState::Error("Invalid length".to_string());
                                break;
                            }
                        }
                        self.header.clear();
                    } else {
                        self.header.push(byte);
                    }
                }
                ParseState::InBody { remaining } => {
                    self.body.push(byte);
                    let new_remaining = remaining - 1;
                    if new_remaining == 0 {
                        self.state = ParseState::Complete;
                        break;
                    } else {
                        self.state = ParseState::InBody {
                            remaining: new_remaining,
                        };
                    }
                }
                ParseState::Complete | ParseState::Error(_) => break,
                ParseState::InHeader => {}
            }
        }

        consumed
    }

    pub fn is_complete(&self) -> bool {
        matches!(self.state, ParseState::Complete)
    }

    pub fn is_error(&self) -> bool {
        matches!(self.state, ParseState::Error(_))
    }

    pub fn take_body(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.body)
    }

    pub fn reset(&mut self) {
        self.state = ParseState::Ready;
        self.header.clear();
        self.body.clear();
        self.body_length = 0;
    }
}

impl Default for StreamingParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Line-based streaming reader
pub struct LineReader {
    buffer: Vec<u8>,
    lines: Vec<String>,
}

impl LineReader {
    pub fn new() -> Self {
        LineReader {
            buffer: Vec::new(),
            lines: Vec::new(),
        }
    }

    /// Feed data, extracting complete lines
    pub fn feed(&mut self, data: &[u8]) {
        for &byte in data {
            if byte == b'\n' {
                if let Ok(line) = String::from_utf8(std::mem::take(&mut self.buffer)) {
                    self.lines.push(line);
                }
            } else if byte != b'\r' {
                self.buffer.push(byte);
            }
        }
    }

    /// Take all complete lines
    pub fn take_lines(&mut self) -> Vec<String> {
        std::mem::take(&mut self.lines)
    }

    /// Check for remaining buffered data
    pub fn has_partial(&self) -> bool {
        !self.buffer.is_empty()
    }
}

impl Default for LineReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_parser_complete() {
        let mut parser = StreamingParser::new();

        parser.feed(b"5:hel");
        assert!(!parser.is_complete());

        parser.feed(b"lo");
        assert!(parser.is_complete());
        assert_eq!(parser.take_body(), b"hello");
    }

    #[test]
    fn test_streaming_parser_single_feed() {
        let mut parser = StreamingParser::new();
        parser.feed(b"3:abc");
        assert!(parser.is_complete());
        assert_eq!(parser.take_body(), b"abc");
    }

    #[test]
    fn test_streaming_parser_reset() {
        let mut parser = StreamingParser::new();
        parser.feed(b"2:ok");
        assert!(parser.is_complete());

        parser.reset();
        parser.feed(b"3:yes");
        assert!(parser.is_complete());
        assert_eq!(parser.take_body(), b"yes");
    }

    #[test]
    fn test_line_reader() {
        let mut reader = LineReader::new();

        reader.feed(b"hello\nwor");
        let lines = reader.take_lines();
        assert_eq!(lines, vec!["hello"]);
        assert!(reader.has_partial());

        reader.feed(b"ld\n");
        let lines = reader.take_lines();
        assert_eq!(lines, vec!["world"]);
    }

    #[test]
    fn test_line_reader_multiple() {
        let mut reader = LineReader::new();
        reader.feed(b"a\nb\nc\n");
        let lines = reader.take_lines();
        assert_eq!(lines, vec!["a", "b", "c"]);
    }
}
