#![allow(clippy::all)]
/// Generate the lines of a Sierpinski triangle of given order.
///
/// # Recursive approach (mirrors OCaml)
///
/// At order 0, we have a single `"*"`. Each higher order:
/// 1. Takes the previous triangle lines
/// 2. Centers them (pads with spaces) for the top half
/// 3. Duplicates each line side-by-side for the bottom half
pub fn sierpinski(n: u32) -> Vec<String> {
    if n == 0 {
        return vec!["*".to_string()];
    }

    let prev = sierpinski(n - 1);
    // Width used for centering the top half.
    // This matches OCaml's `1 lsl n - 1` = (1 << n) - 1
    let width = (1 << n) - 1;

    // Top: center each line from the previous order
    let top: Vec<String> = prev
        .iter()
        .map(|s| {
            let pad = (width - s.len()) / 2;
            format!("{}{}", " ".repeat(pad), s)
        })
        .collect();

    // Bottom: duplicate each line with a space between
    let bottom: Vec<String> = prev.iter().map(|s| format!("{} {}", s, s)).collect();

    [top, bottom].concat()
}

/// Iterative version — builds the triangle bottom-up using fold.
pub fn sierpinski_iter(n: u32) -> Vec<String> {
    (1..=n).fold(vec!["*".to_string()], |prev, i| {
        let width = (1 << i) - 1;

        let top: Vec<String> = prev
            .iter()
            .map(|s| {
                let pad = (width - s.len()) / 2;
                format!("{}{}", " ".repeat(pad), s)
            })
            .collect();

        let bottom: Vec<String> = prev.iter().map(|s| format!("{} {}", s, s)).collect();

        [top, bottom].concat()
    })
}

/// Print the Sierpinski triangle to stdout.
pub fn print_sierpinski(n: u32) {
    for line in sierpinski(n) {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_zero() {
        let result = sierpinski(0);
        assert_eq!(result, vec!["*"]);
    }

    #[test]
    fn test_order_one() {
        let result = sierpinski(1);
        assert_eq!(result.len(), 2);
        // Order 1: top is "*" (no pad since width=1), bottom is "* *"
        assert_eq!(result[0], "*");
        assert_eq!(result[1], "* *");
    }

    #[test]
    fn test_order_two() {
        let result = sierpinski(2);
        assert_eq!(result.len(), 4);
        // Order 2: width=3, centering the order-1 triangle
        assert_eq!(result[0], " *");
        assert_eq!(result[1], "* *");
        assert_eq!(result[2], "* *");
        assert_eq!(result[3], "* * * *");
    }

    #[test]
    fn test_order_four_line_count() {
        // Order n should have 2^n lines
        let result = sierpinski(4);
        assert_eq!(result.len(), 16); // 2^4 = 16
    }

    #[test]
    fn test_iterative_matches_recursive() {
        for n in 0..6 {
            assert_eq!(sierpinski(n), sierpinski_iter(n), "Mismatch at order {}", n);
        }
    }

    #[test]
    fn test_bottom_row_all_stars() {
        // The bottom row of order n contains 2^n asterisks separated by spaces
        let result = sierpinski(3);
        let bottom = result.last().unwrap();
        let star_count = bottom.chars().filter(|&c| c == '*').count();
        assert_eq!(star_count, 8); // 2^3 = 8 asterisks
    }

    #[test]
    fn test_first_line_single_star() {
        // The first line of any order > 0 should contain exactly one '*'
        for n in 1..5 {
            let result = sierpinski(n);
            let star_count = result[0].chars().filter(|&c| c == '*').count();
            assert_eq!(star_count, 1, "Order {} first line should have 1 star", n);
        }
    }
}
