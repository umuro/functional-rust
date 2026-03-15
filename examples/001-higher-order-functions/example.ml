(* 001: Higher-Order Functions
   map, filter, fold_left — the three pillars of functional programming *)

(* --- Approach 1: Using stdlib higher-order functions --- *)

let double_all nums = List.map (fun x -> x * 2) nums

let evens nums = List.filter (fun x -> x mod 2 = 0) nums

let sum nums = List.fold_left ( + ) 0 nums

(* --- Approach 2: Manual recursive implementations --- *)

let rec my_map f = function
  | [] -> []
  | x :: xs -> f x :: my_map f xs

let rec my_filter pred = function
  | [] -> []
  | x :: xs ->
    if pred x then x :: my_filter pred xs
    else my_filter pred xs

(* fold_left with accumulator — naturally tail-recursive *)
let rec my_fold f acc = function
  | [] -> acc
  | x :: xs -> my_fold f (f acc x) xs

(* --- Approach 3: Chained pipeline --- *)

let sum_of_doubled_evens nums =
  nums
  |> List.filter (fun x -> x mod 2 = 0)
  |> List.map (fun x -> x * 2)
  |> List.fold_left ( + ) 0

let () =
  let nums = List.init 10 (fun i -> i + 1) in  (* [1..10] *)

  Printf.printf "double_all [1;2;3] = [%s]\n"
    (String.concat "; " (List.map string_of_int (double_all [1;2;3])));

  Printf.printf "evens [1..10] = [%s]\n"
    (String.concat "; " (List.map string_of_int (evens nums)));

  Printf.printf "sum [1..10] = %d\n" (sum nums);

  Printf.printf "my_map (+1) [1;2;3] = [%s]\n"
    (String.concat "; " (List.map string_of_int (my_map (fun x -> x + 1) [1;2;3])));

  Printf.printf "my_filter (>3) [1..5] = [%s]\n"
    (String.concat "; " (List.map string_of_int (my_filter (fun x -> x > 3) [1;2;3;4;5])));

  Printf.printf "my_fold (+) 0 [1;2;3] = %d\n" (my_fold ( + ) 0 [1;2;3]);

  Printf.printf "sum_of_doubled_evens [1..10] = %d\n" (sum_of_doubled_evens nums)
