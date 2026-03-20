#![allow(clippy::all)]
//! # Async Sink
//!
//! A destination that accepts values and flushes them in batches —
//! the write side of a stream for efficient I/O.

use std::collections::VecDeque;

/// A sink that buffers items and flushes them in batches.
pub struct BatchSink<T> {
    buffer: VecDeque<T>,
    capacity: usize,
    flushed_batches: Vec<Vec<T>>,
}

impl<T: Clone> BatchSink<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::new(),
            capacity,
            flushed_batches: Vec::new(),
        }
    }

    /// Send an item to the sink. Auto-flushes when buffer reaches capacity.
    pub fn send(&mut self, item: T) -> Result<(), String> {
        self.buffer.push_back(item);
        if self.buffer.len() >= self.capacity {
            self.flush()?;
        }
        Ok(())
    }

    /// Manually flush the buffer.
    pub fn flush(&mut self) -> Result<(), String> {
        if !self.buffer.is_empty() {
            let batch: Vec<T> = self.buffer.drain(..).collect();
            self.flushed_batches.push(batch);
        }
        Ok(())
    }

    /// Get the number of items currently buffered.
    pub fn buffered_count(&self) -> usize {
        self.buffer.len()
    }

    /// Get the number of batches that have been flushed.
    pub fn flushed_count(&self) -> usize {
        self.flushed_batches.len()
    }

    /// Consume the sink and return all flushed batches.
    pub fn into_batches(self) -> Vec<Vec<T>> {
        self.flushed_batches
    }
}

/// A sink with a custom flush function.
pub struct CallbackSink<T, F>
where
    F: FnMut(Vec<T>) -> Result<(), String>,
{
    buffer: VecDeque<T>,
    capacity: usize,
    on_flush: F,
    flush_count: usize,
}

impl<T, F> CallbackSink<T, F>
where
    F: FnMut(Vec<T>) -> Result<(), String>,
{
    pub fn new(capacity: usize, on_flush: F) -> Self {
        Self {
            buffer: VecDeque::new(),
            capacity,
            on_flush,
            flush_count: 0,
        }
    }

    pub fn send(&mut self, item: T) -> Result<(), String> {
        self.buffer.push_back(item);
        if self.buffer.len() >= self.capacity {
            self.flush()?;
        }
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), String> {
        if !self.buffer.is_empty() {
            let batch: Vec<T> = self.buffer.drain(..).collect();
            (self.on_flush)(batch)?;
            self.flush_count += 1;
        }
        Ok(())
    }

    pub fn flush_count(&self) -> usize {
        self.flush_count
    }
}

/// A counting sink that tracks statistics.
pub struct StatsSink {
    total_items: usize,
    total_batches: usize,
    max_batch_size: usize,
}

impl StatsSink {
    pub fn new() -> Self {
        Self {
            total_items: 0,
            total_batches: 0,
            max_batch_size: 0,
        }
    }

    pub fn record_batch(&mut self, size: usize) {
        self.total_items += size;
        self.total_batches += 1;
        self.max_batch_size = self.max_batch_size.max(size);
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        (self.total_items, self.total_batches, self.max_batch_size)
    }
}

impl Default for StatsSink {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_sink_auto_flush() {
        let mut sink: BatchSink<i32> = BatchSink::new(3);
        for i in 1..=9 {
            sink.send(i).unwrap();
        }
        let batches = sink.into_batches();
        assert_eq!(batches.len(), 3);
        assert_eq!(batches[0], vec![1, 2, 3]);
        assert_eq!(batches[1], vec![4, 5, 6]);
        assert_eq!(batches[2], vec![7, 8, 9]);
    }

    #[test]
    fn test_batch_sink_manual_flush() {
        let mut sink: BatchSink<i32> = BatchSink::new(5);
        for i in 1..=3 {
            sink.send(i).unwrap();
        }
        assert_eq!(sink.buffered_count(), 3);
        sink.flush().unwrap();
        assert_eq!(sink.buffered_count(), 0);
        let batches = sink.into_batches();
        assert_eq!(batches[0], vec![1, 2, 3]);
    }

    #[test]
    fn test_batch_sink_partial_batch() {
        let mut sink: BatchSink<i32> = BatchSink::new(3);
        for i in 1..=5 {
            sink.send(i).unwrap();
        }
        sink.flush().unwrap();
        let batches = sink.into_batches();
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0], vec![1, 2, 3]);
        assert_eq!(batches[1], vec![4, 5]);
    }

    #[test]
    fn test_callback_sink() {
        let mut collected = Vec::new();
        let mut sink = CallbackSink::new(2, |batch: Vec<i32>| {
            collected.extend(batch);
            Ok(())
        });

        for i in 1..=4 {
            sink.send(i).unwrap();
        }

        assert_eq!(sink.flush_count(), 2);
    }

    #[test]
    fn test_stats_sink() {
        let mut stats = StatsSink::new();
        stats.record_batch(3);
        stats.record_batch(5);
        stats.record_batch(2);

        let (items, batches, max) = stats.stats();
        assert_eq!(items, 10);
        assert_eq!(batches, 3);
        assert_eq!(max, 5);
    }
}
