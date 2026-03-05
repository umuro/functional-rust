(* Divide and Conquer Framework in OCaml *)

(* Merge sort — the canonical D&C algorithm *)
let rec merge_sort (xs : 'a list) : 'a list =
  match xs with
  | [] | [_] -> xs
  | _ ->
    let n = List.length xs in
    let mid = n / 2 in
    let left = List.filteri (fun i _ -> i < mid) xs in
    let right = List.filteri (fun i _ -> i >= mid) xs in
    let sl = merge_sort left in
    let sr = merge_sort right in
    (* Merge two sorted lists *)
    let rec merge a b = match (a, b) with
      | ([], b) -> b
      | (a, []) -> a
      | (x::xs, y::ys) ->
        if x <= y then x :: merge xs b
        else y :: merge a ys
    in
    merge sl sr

(* Binary search: returns Some index or None *)
let binary_search (arr : 'a array) (target : 'a) : int option =
  let rec go lo hi =
    if lo > hi then None
    else
      let mid = (lo + hi) / 2 in
      if arr.(mid) = target then Some mid
      else if arr.(mid) < target then go (mid + 1) hi
      else go lo (mid - 1)
  in
  go 0 (Array.length arr - 1)

(* Maximum subarray sum via D&C (Kadane's is O(n) but D&C shows the pattern) *)
let max_crossing_sum (arr : int array) (lo mid hi : int) : int =
  let left_sum = ref min_int and s = ref 0 in
  for i = mid downto lo do
    s := !s + arr.(i);
    if !s > !left_sum then left_sum := !s
  done;
  let right_sum = ref min_int in
  s := 0;
  for i = mid + 1 to hi do
    s := !s + arr.(i);
    if !s > !right_sum then right_sum := !s
  done;
  !left_sum + !right_sum

let rec max_subarray (arr : int array) (lo hi : int) : int =
  if lo = hi then arr.(lo)
  else
    let mid = (lo + hi) / 2 in
    let left_max = max_subarray arr lo mid in
    let right_max = max_subarray arr (mid + 1) hi in
    let cross_max = max_crossing_sum arr lo mid hi in
    max left_max (max right_max cross_max)

let () =
  let xs = [5; 3; 8; 1; 9; 2; 7; 4; 6] in
  let sorted = merge_sort xs in
  Printf.printf "merge_sort %s = %s\n"
    (String.concat "," (List.map string_of_int xs))
    (String.concat "," (List.map string_of_int sorted));

  let arr = [| 1; 3; 5; 7; 9; 11; 13 |] in
  Printf.printf "binary_search(7) = %s\n"
    (match binary_search arr 7 with Some i -> string_of_int i | None -> "None");
  Printf.printf "binary_search(6) = %s\n"
    (match binary_search arr 6 with Some i -> string_of_int i | None -> "None");

  let nums = [| -2; 1; -3; 4; -1; 2; 1; -5; 4 |] in
  Printf.printf "max_subarray([-2,1,-3,4,-1,2,1,-5,4]) = %d (expected 6)\n"
    (max_subarray nums 0 (Array.length nums - 1))
