#[derive(Debug, Clone)]
struct Queue<T> {
    front: Vec<T>,
    back: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { front: Vec::new(), back: Vec::new() }
    }

    fn enqueue(mut self, x: T) -> Self {
        self.back.push(x);
        self
    }

    fn dequeue(mut self) -> Option<(T, Self)> {
        if self.front.is_empty() {
            if self.back.is_empty() {
                return None;
            }
            self.back.reverse();
            std::mem::swap(&mut self.front, &mut self.back);
        }
        let head = self.front.remove(0);
        Some((head, self))
    }
}

fn drain<T: std::fmt::Debug>(mut queue: Queue<T>) -> Vec<T> {
    let mut result = Vec::new();
    while let Some((x, rest)) = queue.dequeue() {
        result.push(x);
        queue = rest;
    }
    result
}

fn main() {
    let q = Queue::new().enqueue(1).enqueue(2).enqueue(3);
    let drained = drain(q);
    println!("Drained: {:?}", drained);

    // Interleaved operations
    let q = Queue::new().enqueue(1).enqueue(2);
    let (val, q) = q.dequeue().unwrap();
    println!("Dequeued: {}", val);
    let q = q.enqueue(3);
    println!("Remaining: {:?}", drain(q));
}

/* Output:
   Drained: [1, 2, 3]
   Dequeued: 1
   Remaining: [2, 3]
*/
