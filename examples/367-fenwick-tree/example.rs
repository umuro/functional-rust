struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self { tree: vec![0; n+1], n }
    }

    fn from_slice(arr: &[i64]) -> Self {
        let mut ft = Self::new(arr.len());
        for (i, &v) in arr.iter().enumerate() { ft.update(i+1, v); }
        ft
    }

    fn update(&mut self, mut i: usize, delta: i64) {
        while i <= self.n {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    fn prefix_sum(&self, mut i: usize) -> i64 {
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }

    fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.prefix_sum(r) - if l > 1 { self.prefix_sum(l-1) } else { 0 }
    }

    fn point_query(&self, i: usize) -> i64 {
        self.range_sum(i, i)
    }
}

fn main() {
    let arr: Vec<i64> = vec![1,2,3,4,5,6,7,8];
    let mut ft = FenwickTree::from_slice(&arr);
    println!("Prefix(4) = {}", ft.prefix_sum(4));    // 1+2+3+4=10
    println!("Range(2,5) = {}", ft.range_sum(2,5));  // 2+3+4+5=14
    ft.update(3, 10); // add 10 to position 3
    println!("After update(3,+10), Range(2,5) = {}", ft.range_sum(2,5)); // 2+13+4+5=24
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn prefix_sums() {
        let ft = FenwickTree::from_slice(&[1,2,3,4,5]);
        assert_eq!(ft.prefix_sum(3), 6);
        assert_eq!(ft.prefix_sum(5), 15);
    }
    #[test] fn range_query() {
        let ft = FenwickTree::from_slice(&[1,2,3,4,5]);
        assert_eq!(ft.range_sum(2,4), 9);
    }
    #[test] fn point_update() {
        let mut ft = FenwickTree::from_slice(&[1,2,3,4,5]);
        ft.update(3, 7);
        assert_eq!(ft.point_query(3), 10);
        assert_eq!(ft.prefix_sum(5), 22);
    }
}
