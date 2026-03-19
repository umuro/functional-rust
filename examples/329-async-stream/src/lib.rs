#![allow(clippy::all)]
//! # Async Streams
//!
//! An iterator that yields values one by one — with the ability to pause
//! and resume between each item. Foundation for streaming APIs and pipelines.

/// A lazy range stream that generates values on demand.
pub struct RangeStream {
    current: i64,
    end: i64,
}

impl RangeStream {
    pub fn new(start: i64, end: i64) -> Self {
        Self {
            current: start,
            end,
        }
    }
}

impl Iterator for RangeStream {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            None
        } else {
            let value = self.current;
            self.current += 1;
            Some(value)
        }
    }
}

/// A stateful stream that yields data in fixed-size chunks.
pub enum ChunkedStream<T> {
    Active {
        data: Vec<T>,
        position: usize,
        chunk_size: usize,
    },
    Done,
}

impl<T: Clone> ChunkedStream<T> {
    pub fn new(data: Vec<T>, chunk_size: usize) -> Self {
        Self::Active {
            data,
            position: 0,
            chunk_size,
        }
    }

    pub fn next_chunk(&mut self) -> Option<Vec<T>> {
        match self {
            Self::Done => None,
            Self::Active {
                data,
                position,
                chunk_size,
            } => {
                if *position >= data.len() {
                    *self = Self::Done;
                    return None;
                }
                let end = (*position + *chunk_size).min(data.len());
                let chunk = data[*position..end].to_vec();
                *position = end;
                Some(chunk)
            }
        }
    }
}

/// A stream that applies a transformation to each element.
pub struct MapStream<I, F> {
    inner: I,
    f: F,
}

impl<I, F, T, U> MapStream<I, F>
where
    I: Iterator<Item = T>,
    F: FnMut(T) -> U,
{
    pub fn new(inner: I, f: F) -> Self {
        Self { inner, f }
    }
}

impl<I, F, T, U> Iterator for MapStream<I, F>
where
    I: Iterator<Item = T>,
    F: FnMut(T) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(&mut self.f)
    }
}

/// A stream that filters elements based on a predicate.
pub struct FilterStream<I, P> {
    inner: I,
    predicate: P,
}

impl<I, P, T> FilterStream<I, P>
where
    I: Iterator<Item = T>,
    P: FnMut(&T) -> bool,
{
    pub fn new(inner: I, predicate: P) -> Self {
        Self { inner, predicate }
    }
}

impl<I, P, T> Iterator for FilterStream<I, P>
where
    I: Iterator<Item = T>,
    P: FnMut(&T) -> bool,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                None => return None,
                Some(item) => {
                    if (self.predicate)(&item) {
                        return Some(item);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_stream_basic() {
        let stream = RangeStream::new(0, 5);
        let values: Vec<_> = stream.collect();
        assert_eq!(values, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_range_stream_empty() {
        let stream = RangeStream::new(5, 5);
        let values: Vec<_> = stream.collect();
        assert!(values.is_empty());
    }

    #[test]
    fn test_range_stream_with_filter_map() {
        let result: Vec<i64> = RangeStream::new(0, 10)
            .filter(|x| x % 2 == 0)
            .map(|x| x * x)
            .collect();
        assert_eq!(result, vec![0, 4, 16, 36, 64]);
    }

    #[test]
    fn test_chunked_stream_exact() {
        let mut stream = ChunkedStream::new(vec![1, 2, 3, 4, 5, 6], 2);
        assert_eq!(stream.next_chunk(), Some(vec![1, 2]));
        assert_eq!(stream.next_chunk(), Some(vec![3, 4]));
        assert_eq!(stream.next_chunk(), Some(vec![5, 6]));
        assert_eq!(stream.next_chunk(), None);
    }

    #[test]
    fn test_chunked_stream_partial() {
        let mut stream = ChunkedStream::new(vec![1, 2, 3, 4, 5], 2);
        let mut all = Vec::new();
        while let Some(chunk) = stream.next_chunk() {
            all.extend(chunk);
        }
        assert_eq!(all, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_map_stream() {
        let stream = MapStream::new(vec![1, 2, 3].into_iter(), |x| x * 2);
        let values: Vec<_> = stream.collect();
        assert_eq!(values, vec![2, 4, 6]);
    }

    #[test]
    fn test_filter_stream() {
        let stream = FilterStream::new(vec![1, 2, 3, 4, 5].into_iter(), |x| x % 2 == 1);
        let values: Vec<_> = stream.collect();
        assert_eq!(values, vec![1, 3, 5]);
    }
}
