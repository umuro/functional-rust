/// Compute the Hamming distance between two strings.
fn hamming(s1: &str, s2: &str) -> Result<usize, &'static str> {
    if s1.len() != s2.len() {
        return Err("strands must be of equal length");
    }
    Ok(s1
        .chars()
        .zip(s2.chars())
        .filter(|(a, b)| a != b)
        .count())
}

/// Fold-based version — mirrors OCaml's Seq.fold_left.
fn hamming_fold(s1: &str, s2: &str) -> Result<usize, &'static str> {
    if s1.len() != s2.len() {
        return Err("strands must be of equal length");
    }
    Ok(s1
        .chars()
        .zip(s2.chars())
        .fold(0, |acc, (a, b)| if a != b { acc + 1 } else { acc }))
}

fn main() {
    match hamming("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT") {
        Ok(d) => println!("Hamming distance: {}", d),
        Err(e) => println!("Error: {}", e),
    }

    // Demonstrate error case
    match hamming("AB", "ABC") {
        Ok(d) => println!("Distance: {}", d),
        Err(e) => println!("Error: {}", e),
    }

    // Fold version
    println!(
        "Fold version: {:?}",
        hamming_fold("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT")
    );
}

/* Output:
   Hamming distance: 7
   Error: strands must be of equal length
   Fold version: Ok(7)
*/
