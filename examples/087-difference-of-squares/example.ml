(* Difference of Squares *)

(* Version 1: Using fold *)
let square_of_sum n =
  let s = List.init n (fun i -> i + 1) |> List.fold_left (+) 0 in
  s * s

let sum_of_squares n =
  List.init n (fun i -> i + 1)
  |> List.fold_left (fun acc x -> acc + x * x) 0

let difference n = square_of_sum n - sum_of_squares n

(* Version 2: Closed-form *)
let square_of_sum_formula n =
  let s = n * (n + 1) / 2 in s * s

let sum_of_squares_formula n =
  n * (n + 1) * (2 * n + 1) / 6

let () =
  assert (difference 10 = 2640);
  assert (square_of_sum_formula 10 = square_of_sum 10);
  assert (sum_of_squares_formula 10 = sum_of_squares 10)
