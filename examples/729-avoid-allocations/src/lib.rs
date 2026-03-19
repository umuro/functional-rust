/// 729: Avoid Allocations — Hot path techniques in std-only Rust

// ── Technique 1: Stack-allocated scratch buffer ──────────────────────────────

/// Write a formatted record into a pre-allocated `String` buffer.
/// Caller owns the buffer and can reuse it across many calls.
fn format_record_into(buf: &mut String, name: &str, score: u32) {
    buf.clear(); // reset without freeing heap memory
    buf.push_str(name);
    buf.push(':');
    // itoa-style: write integer without allocating a temporary String
    let mut tmp = [0u8; 20];
    let digits = u32_to_str(score, &mut tmp);
    buf.push_str(digits);
}

/// Format u32 into a stack buffer; return the filled slice.
fn u32_to_str(mut n: u32, buf: &mut [u8; 20]) -> &str {
    if n == 0 {
        buf[19] = b'0';
        return std::str::from_utf8(&buf[19..]).unwrap();
    }
    let mut pos = 20usize;
    while n > 0 {
        pos -= 1;
        buf[pos] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    std::str::from_utf8(&buf[pos..]).unwrap()
}

// ── Technique 2: Iterator chains — zero intermediate allocations ──────────────

fn sum_squares(n: u64) -> u64 {
    (0..n).map(|i| i * i).sum() // no Vec created; purely lazy
}

fn hot_filter_sum(data: &[i32]) -> i32 {
    data.iter().filter(|&&x| x > 0).map(|&x| x * 2).sum() // single pass, zero allocs
}

// ── Technique 3: Reuse a Vec by clearing, not dropping ───────────────────────

struct Pipeline {
    scratch: Vec<i32>,
}

impl Pipeline {
    fn new() -> Self {
        Pipeline {
            scratch: Vec::with_capacity(1024),
        }
    }

    fn process(&mut self, input: &[i32]) -> &[i32] {
        self.scratch.clear(); // keeps allocated capacity
        for &x in input {
            if x.rem_euclid(2) == 0 {
                self.scratch.push(x * 3);
            }
        }
        &self.scratch
    }
}

// ── Technique 4: Fixed-size stack array as scratch ────────────────────────────

fn count_words_no_alloc(s: &str) -> usize {
    // Count words without allocating — just scan bytes
    let bytes = s.as_bytes();
    let mut in_word = false;
    let mut count = 0usize;
    for &b in bytes {
        match (in_word, b == b' ' || b == b'\t' || b == b'\n') {
            (false, false) => {
                in_word = true;
                count += 1;
            }
            (true, true) => {
                in_word = false;
            }
            _ => {}
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_record_reuse() {
        let mut buf = String::with_capacity(32);
        format_record_into(&mut buf, "Alice", 99);
        assert_eq!(buf, "Alice:99");
        format_record_into(&mut buf, "Bob", 0);
        assert_eq!(buf, "Bob:0");
    }

    #[test]
    fn test_sum_squares() {
        assert_eq!(sum_squares(0), 0);
        assert_eq!(sum_squares(4), 0 + 1 + 4 + 9); // 14
    }

    #[test]
    fn test_hot_filter_sum() {
        assert_eq!(hot_filter_sum(&[1, -2, 3, -4]), (1 + 3) * 2);
        assert_eq!(hot_filter_sum(&[]), 0);
    }

    #[test]
    fn test_pipeline_reuse() {
        let mut p = Pipeline::new();
        let r1 = p.process(&[2, 3, 4]);
        assert_eq!(r1, &[6, 12]); // evens * 3
        let r2 = p.process(&[10]);
        assert_eq!(r2, &[30]);
    }

    #[test]
    fn test_word_count() {
        assert_eq!(count_words_no_alloc(""), 0);
        assert_eq!(count_words_no_alloc("hello"), 1);
        assert_eq!(count_words_no_alloc("hello world"), 2);
        assert_eq!(count_words_no_alloc("  a  b  c  "), 3);
    }
}
