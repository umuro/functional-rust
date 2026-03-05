(* Higher-Order Functions — Map and Filter from Scratch *)
(* Implement map and filter as higher-order functions *)

let rec my_map f = function
  | [] -> []
  | x :: xs -> f x :: my_map f xs

let rec my_filter pred = function
  | [] -> []
  | x :: xs ->
    if pred x then x :: my_filter pred xs
    else my_filter pred xs

let squares = my_map (fun x -> x * x) [1;2;3;4;5]
let big = my_filter (fun x -> x > 10) squares
let () = List.iter (fun x -> Printf.printf "%d " x) big
