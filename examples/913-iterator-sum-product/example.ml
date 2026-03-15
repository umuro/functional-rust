(* 274. Numeric reductions: sum() and product() - OCaml *)

let sum lst = List.fold_left (+) 0 lst
let product lst = List.fold_left ( * ) 1 lst

let () =
  let nums = [1; 2; 3; 4; 5] in
  Printf.printf "Sum: %d\n" (sum nums);
  Printf.printf "Product: %d\n" (product nums);

  let factorial n = product (List.init n (fun i -> i + 1)) in
  Printf.printf "5! = %d\n" (factorial 5);
  Printf.printf "10! = %d\n" (factorial 10);

  let sum_squares = sum (List.map (fun x -> x * x) nums) in
  Printf.printf "Sum of squares: %d\n" sum_squares;

  let prices = [9.99; 14.50; 3.75; 22.00] in
  let total = List.fold_left (+.) 0.0 prices in
  Printf.printf "Total: %.2f\n" total;
  Printf.printf "Average: %.2f\n" (total /. float_of_int (List.length prices))
