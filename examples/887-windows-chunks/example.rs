// example.rs content
use example_887_windows_chunks::{
    chunk_exact_sums, chunk_maxes, chunk_sums, chunks_recursive, contains_pattern, local_maxima,
    moving_average, pairwise_diff, windows_recursive,
};

fn main() {
    let data = [1, 2, 3, 4, 5];

    // Windows (overlapping)
    let wins = windows_recursive(&data, 3);
    println!("windows(3): {:?}", wins);

    let avgs = moving_average(&data.map(|x| x as f64), 3);
    println!("moving_average(3): {:?}", avgs);

    let diffs = pairwise_diff(&[1, 3, 6, 10]);
    println!("pairwise_diff: {:?}", diffs);

    // Local maxima
    let maxima = local_maxima(&[1, 3, 2, 5, 4, 6, 1]);
    println!("local_maxima: {:?}", maxima);

    // Pattern search
    println!("contains [3,4,5]: {}", contains_pattern(&[3, 4, 5], &[1, 2, 3, 4, 5, 6]));
    println!("contains [3,5]:   {}", contains_pattern(&[3, 5], &[1, 2, 3, 4, 5, 6]));

    // Chunks (non-overlapping)
    let sums = chunk_sums(&data, 2);
    println!("chunk_sums(2): {:?}", sums);

    let maxes = chunk_maxes(&data, 2);
    println!("chunk_maxes(2): {:?}", maxes);

    let (exact, remainder) = chunk_exact_sums(&data, 2);
    println!("chunk_exact_sums(2): sums={:?} remainder={:?}", exact, remainder);

    // Recursive chunks
    let chunks = chunks_recursive(&data, 2);
    println!("chunks_recursive(2): {:?}", chunks);
}