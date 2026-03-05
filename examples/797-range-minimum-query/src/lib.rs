//! # Range Minimum Query (Sparse Table)

pub struct SparseTable { table: Vec<Vec<usize>>, log: Vec<usize> }

impl SparseTable {
    pub fn new(arr: &[i32]) -> Self {
        let n = arr.len();
        let k = (n as f64).log2().floor() as usize + 1;
        let mut table = vec![vec![0; n]; k];
        let mut log = vec![0; n + 1];
        for i in 2..=n { log[i] = log[i/2] + 1; }
        for i in 0..n { table[0][i] = i; }
        for j in 1..k {
            for i in 0..=(n - (1 << j)) {
                let left = table[j-1][i];
                let right = table[j-1][i + (1 << (j-1))];
                table[j][i] = if arr[left] <= arr[right] { left } else { right };
            }
        }
        Self { table, log }
    }
    
    pub fn query(&self, arr: &[i32], l: usize, r: usize) -> usize {
        let j = self.log[r - l + 1];
        let left = self.table[j][l];
        let right = self.table[j][r - (1 << j) + 1];
        if arr[left] <= arr[right] { left } else { right }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_rmq() {
        let arr = [1, 3, 2, 7, 9, 11];
        let st = SparseTable::new(&arr);
        assert_eq!(st.query(&arr, 0, 2), 0);
    }
}
