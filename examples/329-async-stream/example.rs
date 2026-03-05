struct RangeStream { current: i64, end: i64 }

impl RangeStream {
    fn new(start: i64, end: i64) -> Self { Self { current: start, end } }
}

impl Iterator for RangeStream {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.current >= self.end { None }
        else { let v = self.current; self.current += 1; Some(v) }
    }
}

enum ChunkedStream { Active { data: Vec<i32>, pos: usize, sz: usize }, Done }

impl ChunkedStream {
    fn new(data: Vec<i32>, sz: usize) -> Self { Self::Active { data, pos: 0, sz } }
    fn next_chunk(&mut self) -> Option<Vec<i32>> {
        match self {
            Self::Done => None,
            Self::Active { data, pos, sz } => {
                if *pos >= data.len() { *self = Self::Done; return None; }
                let end = (*pos + *sz).min(data.len());
                let chunk = data[*pos..end].to_vec();
                *pos = end;
                Some(chunk)
            }
        }
    }
}

fn main() {
    let result: Vec<i64> = RangeStream::new(0,20).filter(|x| x%2==0).map(|x|x*2).take(5).collect();
    println!("Stream: {result:?}");
    let mut s = ChunkedStream::new((0..10).collect(), 3);
    while let Some(chunk) = s.next_chunk() { println!("Chunk: {chunk:?}"); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn range_correct() { let v: Vec<_> = RangeStream::new(1,6).collect(); assert_eq!(v, vec![1,2,3,4,5]); }
    #[test] fn filter_map() {
        let v: Vec<i64> = RangeStream::new(0,10).filter(|x|x%2==0).map(|x|x*x).collect();
        assert_eq!(v, vec![0,4,16,36,64]);
    }
    #[test] fn chunked_all() {
        let mut s = ChunkedStream::new(vec![1,2,3,4,5], 2);
        let mut all = Vec::new();
        while let Some(c) = s.next_chunk() { all.extend(c); }
        assert_eq!(all, vec![1,2,3,4,5]);
    }
}
