struct SegTree {
    data: Vec<i64>,
    n: usize,
}

impl SegTree {
    fn new(arr: &[i64]) -> Self {
        let n = arr.len();
        let mut st = Self { data: vec![0; 4*n], n };
        st.build(arr, 1, 0, n-1);
        st
    }

    fn build(&mut self, arr: &[i64], v: usize, l: usize, r: usize) {
        if l == r { self.data[v] = arr[l]; return; }
        let m = (l+r)/2;
        self.build(arr, 2*v, l, m);
        self.build(arr, 2*v+1, m+1, r);
        self.data[v] = self.data[2*v] + self.data[2*v+1];
    }

    fn query(&self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if qr < l || r < ql { return 0; }
        if ql <= l && r <= qr { return self.data[v]; }
        let m = (l+r)/2;
        self.query(2*v, l, m, ql, qr) + self.query(2*v+1, m+1, r, ql, qr)
    }

    fn update(&mut self, v: usize, l: usize, r: usize, pos: usize, val: i64) {
        if l == r { self.data[v] = val; return; }
        let m = (l+r)/2;
        if pos <= m { self.update(2*v, l, m, pos, val); }
        else { self.update(2*v+1, m+1, r, pos, val); }
        self.data[v] = self.data[2*v] + self.data[2*v+1];
    }

    fn sum(&self, l: usize, r: usize) -> i64 { self.query(1, 0, self.n-1, l, r) }
    fn set(&mut self, pos: usize, val: i64) { self.update(1, 0, self.n-1, pos, val); }
}

fn main() {
    let arr: Vec<i64> = vec![1,3,5,7,9,11];
    let mut st = SegTree::new(&arr);
    println!("Sum[1..3] = {}", st.sum(1,3));  // 3+5+7=15
    println!("Sum[0..5] = {}", st.sum(0,5));  // 36
    st.set(2, 10); // change 5 -> 10
    println!("After update[2]=10, Sum[1..3] = {}", st.sum(1,3)); // 3+10+7=20
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn range_sum() {
        let st = SegTree::new(&[1,2,3,4,5]);
        assert_eq!(st.sum(0,4), 15);
        assert_eq!(st.sum(1,3), 9);
    }
    #[test] fn point_update() {
        let mut st = SegTree::new(&[1,2,3,4,5]);
        st.set(2, 10);
        assert_eq!(st.sum(0,4), 22); // 1+2+10+4+5
    }
}
