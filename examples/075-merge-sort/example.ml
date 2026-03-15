(* 075: Merge Sort
   Classic divide-and-conquer; OCaml lists split naturally *)

(* --- Approach 1: Classic recursive merge sort --- *)

let rec merge l1 l2 =
  match l1, l2 with
  | [], ys -> ys
  | xs, [] -> xs
  | x :: xs, y :: ys ->
    if x <= y then x :: merge xs (y :: ys)
    else            y :: merge (x :: xs) ys

(* Split a list into two halves *)
let split xs =
  let rec aux left right toggle = function
    | [] -> (List.rev left, List.rev right)
    | h :: t ->
      if toggle then aux (h :: left) right false t
      else           aux left (h :: right) true  t
  in
  aux [] [] true xs

let rec merge_sort = function
  | ([] | [_]) as xs -> xs
  | xs ->
    let (l, r) = split xs in
    merge (merge_sort l) (merge_sort r)

(* --- Approach 2: Generic with comparator function --- *)

let rec merge_by cmp l1 l2 =
  match l1, l2 with
  | [], ys -> ys
  | xs, [] -> xs
  | x :: xs, y :: ys ->
    if cmp x y <= 0 then x :: merge_by cmp xs (y :: ys)
    else                  y :: merge_by cmp (x :: xs) ys

let rec merge_sort_by cmp = function
  | ([] | [_]) as xs -> xs
  | xs ->
    let (l, r) = split xs in
    merge_by cmp (merge_sort_by cmp l) (merge_sort_by cmp r)

let () =
  let show xs = "[" ^ String.concat "; " (List.map string_of_int xs) ^ "]" in
  Printf.printf "merge_sort [5;3;8;1;9;2;7] = %s\n"
    (show (merge_sort [5;3;8;1;9;2;7]));
  Printf.printf "merge_sort [] = %s\n" (show (merge_sort []));
  Printf.printf "merge_sort [2;1] = %s\n" (show (merge_sort [2;1]));
  Printf.printf "merge_sort_by descending [5;3;8;1] = %s\n"
    (show (merge_sort_by (fun a b -> compare b a) [5;3;8;1]))
