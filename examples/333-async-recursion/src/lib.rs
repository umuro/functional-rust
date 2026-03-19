#![allow(clippy::all)]
//! # Async Recursion
//!
//! Recursive async functions need `Box::pin` — the future's size must be known at compile time.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// Type alias for a heap-pinned future with lifetime 'a.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

/// A binary tree for demonstrating recursive async operations.
#[derive(Debug, Clone)]
pub enum Tree<T> {
    Leaf,
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

impl<T> Tree<T> {
    pub fn leaf() -> Box<Self> {
        Box::new(Self::Leaf)
    }

    pub fn node(value: T, left: Box<Self>, right: Box<Self>) -> Box<Self> {
        Box::new(Self::Node { value, left, right })
    }
}

impl Tree<i32> {
    /// Create a sample tree for testing.
    pub fn sample() -> Box<Self> {
        //       1
        //      / \
        //     2   3
        //    / \   \
        //   4   5   6
        Self::node(
            1,
            Self::node(
                2,
                Self::node(4, Self::leaf(), Self::leaf()),
                Self::node(5, Self::leaf(), Self::leaf()),
            ),
            Self::node(3, Self::node(6, Self::leaf(), Self::leaf()), Self::leaf()),
        )
    }
}

/// Async sum of all values in the tree (recursive, uses Box::pin).
pub fn async_sum(tree: &Tree<i32>) -> BoxFuture<'_, i64> {
    Box::pin(async move {
        match tree {
            Tree::Leaf => 0,
            Tree::Node { value, left, right } => {
                *value as i64 + async_sum(left).await + async_sum(right).await
            }
        }
    })
}

/// Async depth calculation (recursive).
pub fn async_depth<T>(tree: &Tree<T>) -> BoxFuture<'_, usize> {
    Box::pin(async move {
        match tree {
            Tree::Leaf => 0,
            Tree::Node { left, right, .. } => {
                1 + async_depth(left).await.max(async_depth(right).await)
            }
        }
    })
}

/// Async count of nodes (recursive).
pub fn async_count<T>(tree: &Tree<T>) -> BoxFuture<'_, usize> {
    Box::pin(async move {
        match tree {
            Tree::Leaf => 0,
            Tree::Node { left, right, .. } => {
                1 + async_count(left).await + async_count(right).await
            }
        }
    })
}

/// A minimal single-threaded executor for running futures.
pub fn block_on<F: Future>(fut: F) -> F::Output {
    unsafe fn clone_waker(ptr: *const ()) -> RawWaker {
        RawWaker::new(ptr, &VTABLE)
    }
    unsafe fn noop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_waker, noop, noop, noop);

    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);

    loop {
        if let Poll::Ready(value) = fut.as_mut().poll(&mut cx) {
            return value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_sum_is_zero() {
        assert_eq!(block_on(async_sum(&Tree::Leaf)), 0);
    }

    #[test]
    fn test_sample_tree_sum() {
        let tree = Tree::sample();
        assert_eq!(block_on(async_sum(&tree)), 21); // 1+2+3+4+5+6 = 21
    }

    #[test]
    fn test_sample_tree_depth() {
        let tree = Tree::sample();
        assert_eq!(block_on(async_depth(&tree)), 3);
    }

    #[test]
    fn test_sample_tree_count() {
        let tree = Tree::sample();
        assert_eq!(block_on(async_count(&tree)), 6);
    }

    #[test]
    fn test_single_node() {
        let tree = Tree::node(42, Tree::leaf(), Tree::leaf());
        assert_eq!(block_on(async_sum(&tree)), 42);
        assert_eq!(block_on(async_depth(&tree)), 1);
        assert_eq!(block_on(async_count(&tree)), 1);
    }

    #[test]
    fn test_left_skewed_tree() {
        let tree = Tree::node(
            1,
            Tree::node(2, Tree::node(3, Tree::leaf(), Tree::leaf()), Tree::leaf()),
            Tree::leaf(),
        );
        assert_eq!(block_on(async_depth(&tree)), 3);
        assert_eq!(block_on(async_sum(&tree)), 6);
    }
}
