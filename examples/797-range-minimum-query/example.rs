// Range Minimum Query — Sparse Table O(n log n) build, O(1) query

struct SparseTable {
    table: Vec<Vec<i64>>,
    log2: Vec<usize>,
}

impl SparseTable {
    fn build(arr: &[i64]) -> Self {
        let n = arr.len();
        let levels = if n > 1 { usize::BITS as usize - n.leading_zeros() as usize } else { 1 };
        let mut table = vec![arr.to_vec()];
        for k in 1..levels {
            let prev = &table[k - 1];
            let half = 1 << (k - 1);
            let row: Vec<i64> = (0..n.saturating_sub((1 << k) - 1))
                .map(|i| prev[i].min(prev[i + half]))
                .collect();
            table.push(row);
        }
        // Precompute floor(log2(i)) for i = 0..n
        let mut log2 = vec![0usize; n + 1];
        for i in 2..=n { log2[i] = log2[i / 2] + 1; }
        SparseTable { table, log2 }
    }

    fn query(&self, l: usize, r: usize) -> i64 {
        let k = self.log2[r - l + 1];
        self.table[k][l].min(self.table[k][r + 1 - (1 << k)])
    }
}

fn main() {
    let arr = vec![2i64, 4, 3, 1, 6, 7, 8, 9, 1, 7];
    let st  = SparseTable::build(&arr);
    println!("Array: {:?}", arr);
    println!("RMQ(0,9) = {}  (expect 1)", st.query(0, 9));
    println!("RMQ(1,5) = {}  (expect 1)", st.query(1, 5));
    println!("RMQ(2,4) = {}  (expect 1)", st.query(2, 4));
    println!("RMQ(6,9) = {}  (expect 1)", st.query(6, 9));
}
