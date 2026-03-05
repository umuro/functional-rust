/// Generate the next row of Pascal's triangle using zip-with-add.
fn next_row(row: &[u64]) -> Vec<u64> {
    std::iter::once(&0)
        .chain(row.iter())
        .zip(row.iter().chain(std::iter::once(&0)))
        .map(|(a, b)| a + b)
        .collect()
}

/// Generate n rows of Pascal's triangle.
fn pascal(n: usize) -> Vec<Vec<u64>> {
    std::iter::successors(Some(vec![1u64]), |prev| Some(next_row(prev)))
        .take(n)
        .collect()
}

fn main() {
    for row in pascal(8) {
        let s: Vec<String> = row.iter().map(|x| x.to_string()).collect();
        println!("{}", s.join(" "));
    }
}

/* Output:
   1
   1 1
   1 2 1
   1 3 3 1
   1 4 6 4 1
   1 5 10 10 5 1
   1 6 15 20 15 6 1
   1 7 21 35 35 21 7 1
*/
