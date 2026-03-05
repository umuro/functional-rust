(* Sorting Algorithms Overview in OCaml *)

(* Insertion sort - functional style *)
let rec insert x = function
  | [] -> [x]
  | h :: t -> if x <= h then x :: h :: t else h :: insert x t

let insertion_sort lst = List.fold_right insert lst []

(* Merge sort *)
let rec merge = function
  | [], ys -> ys
  | xs, [] -> xs
  | (x :: xs as xxs), (y :: ys as yys) ->
      if x <= y then x :: merge (xs, yys)
      else y :: merge (xxs, ys)

let rec merge_sort = function
  | [] | [_] as lst -> lst
  | lst ->
      let mid = List.length lst / 2 in
      let left, right = List.filteri (fun i _ -> i < mid) lst,
                        List.filteri (fun i _ -> i >= mid) lst in
      merge (merge_sort left, merge_sort right)

(* Quick sort *)
let rec quick_sort = function
  | [] -> []
  | pivot :: rest ->
      let left = List.filter (fun x -> x < pivot) rest in
      let right = List.filter (fun x -> x >= pivot) rest in
      quick_sort left @ [pivot] @ quick_sort right

let () =
  let arr = [64; 34; 25; 12; 22; 11; 90] in
  let sorted = quick_sort arr in
  Printf.printf "Sorted: [%s]\n" 
    (String.concat "; " (List.map string_of_int sorted))
