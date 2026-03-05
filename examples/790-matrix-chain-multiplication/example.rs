// Matrix Chain Multiplication — bottom-up DP O(n³)
// dims[i..i+1] gives dimensions of matrix i: rows=dims[i], cols=dims[i+1]

fn matrix_chain(dims: &[usize]) -> (usize, Vec<Vec<usize>>) {
    let n = dims.len() - 1;
    let mut dp    = vec![vec![0usize; n]; n];
    let mut split = vec![vec![0usize; n]; n];

    // l = chain length
    for l in 2..=n {
        for i in 0..=(n - l) {
            let j = i + l - 1;
            dp[i][j] = usize::MAX;
            for k in i..j {
                let cost = dp[i][k]
                    .saturating_add(dp[k + 1][j])
                    .saturating_add(dims[i] * dims[k + 1] * dims[j + 1]);
                if cost < dp[i][j] {
                    dp[i][j] = cost;
                    split[i][j] = k;
                }
            }
        }
    }
    (dp[0][n - 1], split)
}

fn parenthesize(split: &Vec<Vec<usize>>, i: usize, j: usize) -> String {
    if i == j {
        format!("M{}", i + 1)
    } else {
        let k = split[i][j];
        format!("({} × {})", parenthesize(split, i, k), parenthesize(split, k + 1, j))
    }
}

fn main() {
    // 6 matrices: dims describe 7 boundary values
    let dims = vec![30, 35, 15, 5, 10, 20, 25];
    let n = dims.len() - 1;
    println!("Number of matrices: {n}");
    let (cost, split) = matrix_chain(&dims);
    println!("Minimum scalar multiplications: {cost}");
    println!("Optimal parenthesization: {}", parenthesize(&split, 0, n - 1));

    // Classic 3-matrix example
    let dims2 = vec![10, 30, 5, 60];
    let (c2, s2) = matrix_chain(&dims2);
    println!("\n3-matrix (10×30, 30×5, 5×60):");
    println!("Min cost: {c2}, Order: {}", parenthesize(&s2, 0, 2));
}
