/// Generate the lines of a Sierpinski triangle of given order.
fn sierpinski(n: u32) -> Vec<String> {
    if n == 0 {
        return vec!["*".to_string()];
    }
    let prev = sierpinski(n - 1);
    let width = (1 << n) - 1;

    let top: Vec<String> = prev
        .iter()
        .map(|s| {
            let pad = (width - s.len()) / 2;
            format!("{}{}", " ".repeat(pad), s)
        })
        .collect();

    let bottom: Vec<String> = prev.iter().map(|s| format!("{} {}", s, s)).collect();

    [top, bottom].concat()
}

fn main() {
    for line in sierpinski(4) {
        println!("{}", line);
    }
}

/* Output:
               *
              * *
             *   *
            * * * *
           *       *
          * *     * *
         *   *   *   *
        * * * * * * * *
       *               *
      * *             * *
     *   *           *   *
    * * * *         * * * *
   *       *       *       *
  * *     * *     * *     * *
 *   *   *   *   *   *   *   *
* * * * * * * * * * * * * * * *
*/
