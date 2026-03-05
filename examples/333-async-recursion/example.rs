use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

#[derive(Debug)]
enum Tree { Leaf, Node { value: i32, left: Box<Tree>, right: Box<Tree> } }

impl Tree {
    fn leaf() -> Box<Self> { Box::new(Self::Leaf) }
    fn node(v: i32, l: Box<Self>, r: Box<Self>) -> Box<Self> { Box::new(Self::Node{value:v,left:l,right:r}) }
}

fn async_sum(t: &Tree) -> BoxFuture<'_, i64> {
    Box::pin(async move {
        match t {
            Tree::Leaf => 0,
            Tree::Node{value,left,right} => *value as i64 + async_sum(left).await + async_sum(right).await,
        }
    })
}

fn async_depth(t: &Tree) -> BoxFuture<'_, usize> {
    Box::pin(async move {
        match t {
            Tree::Leaf => 0,
            Tree::Node{left,right,..} => 1 + async_depth(left).await.max(async_depth(right).await),
        }
    })
}

fn block_on<F: Future>(fut: F) -> F::Output {
    use std::task::{RawWaker,RawWakerVTable,Waker};
    unsafe fn cl(p: *const())->RawWaker{RawWaker::new(p,&V)} unsafe fn n(_:*const()){}
    static V: RawWakerVTable = RawWakerVTable::new(cl,n,n,n);
    let w = unsafe{Waker::from_raw(RawWaker::new(std::ptr::null(),&V))};
    let mut cx = Context::from_waker(&w);
    let mut f = Box::pin(fut);
    loop { if let Poll::Ready(v) = f.as_mut().poll(&mut cx) { return v; } }
}

fn sample() -> Box<Tree> {
    Tree::node(1, Tree::node(2,Tree::node(4,Tree::leaf(),Tree::leaf()),Tree::node(5,Tree::leaf(),Tree::leaf())), Tree::node(3,Tree::node(6,Tree::leaf(),Tree::leaf()),Tree::leaf()))
}

fn main() {
    let t = sample();
    println!("Sum: {}", block_on(async_sum(&t)));
    println!("Depth: {}", block_on(async_depth(&t)));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn leaf_sum_zero() { assert_eq!(block_on(async_sum(&Tree::Leaf)), 0); }
    #[test] fn tree_sum() { assert_eq!(block_on(async_sum(&sample())), 21); }
    #[test] fn tree_depth() { assert_eq!(block_on(async_depth(&sample())), 3); }
}
