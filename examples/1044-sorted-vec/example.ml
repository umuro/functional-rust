(* 1044: Sorted Vec — Binary Search Insert *)
(* Maintain a sorted list/array with efficient search *)

(* Approach 1: Sorted list with binary search on array *)
let binary_search arr target =
  let rec go lo hi =
    if lo >= hi then lo
    else
      let mid = lo + (hi - lo) / 2 in
      if arr.(mid) < target then go (mid + 1) hi
      else go lo mid
  in
  go 0 (Array.length arr)

let sorted_array_ops () =
  let arr = [|1; 3; 5; 7; 9; 11|] in
  assert (binary_search arr 5 = 2);
  assert (binary_search arr 6 = 3);  (* insertion point *)
  assert (binary_search arr 0 = 0);
  assert (binary_search arr 12 = 6)

(* Approach 2: Sorted list with insertion *)
let sorted_insert lst x =
  let rec go = function
    | [] -> [x]
    | (h :: _) as l when x <= h -> x :: l
    | h :: t -> h :: go t
  in
  go lst

let sorted_list_ops () =
  let lst = [] in
  let lst = sorted_insert lst 5 in
  let lst = sorted_insert lst 3 in
  let lst = sorted_insert lst 7 in
  let lst = sorted_insert lst 1 in
  let lst = sorted_insert lst 4 in
  assert (lst = [1; 3; 4; 5; 7])

(* Approach 3: Sorted collection operations *)
let sorted_contains lst x =
  (* Linear for list, but maintains concept *)
  List.exists (fun e -> e = x) lst

let sorted_merge a b =
  let rec go a b =
    match a, b with
    | [], r | r, [] -> r
    | x :: xs, y :: ys ->
      if x <= y then x :: go xs (y :: ys)
      else y :: go (x :: xs) ys
  in
  go a b

let sorted_dedup lst =
  let rec go = function
    | [] | [_] as l -> l
    | x :: (y :: _ as rest) ->
      if x = y then go rest
      else x :: go rest
  in
  go lst

let collection_ops () =
  let a = [1; 3; 5; 7] in
  let b = [2; 3; 6; 8] in
  let merged = sorted_merge a b in
  assert (merged = [1; 2; 3; 3; 5; 6; 7; 8]);
  let deduped = sorted_dedup merged in
  assert (deduped = [1; 2; 3; 5; 6; 7; 8])

let () =
  sorted_array_ops ();
  sorted_list_ops ();
  collection_ops ();
  Printf.printf "✓ All tests passed\n"
