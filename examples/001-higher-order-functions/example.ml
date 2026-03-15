(* 001: Higher-Order Functions *)
(* map, filter, fold — the three pillars of functional programming *)

(* Approach 1: Using standard library functions *)
let double_all lst = List.map (fun x -> x * 2) lst
let evens lst = List.filter (fun x -> x mod 2 = 0) lst
let sum lst = List.fold_left (fun acc x -> acc + x) 0 lst

(* Approach 2: Manual recursive implementations *)
let rec my_map f = function
  | [] -> []
  | x :: xs -> f x :: my_map f xs

let rec my_filter pred = function
  | [] -> []
  | x :: xs ->
    if pred x then x :: my_filter pred xs
    else my_filter pred xs

let rec my_fold_left f acc = function
  | [] -> acc
  | x :: xs -> my_fold_left f (f acc x) xs

(* Approach 3: Composition — combine HOFs *)
let sum_of_doubled_evens lst =
  lst
  |> List.filter (fun x -> x mod 2 = 0)
  |> List.map (fun x -> x * 2)
  |> List.fold_left ( + ) 0

(* Tests *)
let () =
  let nums = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10] in
  assert (double_all [1; 2; 3] = [2; 4; 6]);
  assert (evens nums = [2; 4; 6; 8; 10]);
  assert (sum nums = 55);
  assert (my_map (fun x -> x + 1) [1; 2; 3] = [2; 3; 4]);
  assert (my_filter (fun x -> x > 3) [1; 2; 3; 4; 5] = [4; 5]);
  assert (my_fold_left ( + ) 0 [1; 2; 3] = 6);
  assert (sum_of_doubled_evens nums = 60);
  Printf.printf "✓ All tests passed\n"
