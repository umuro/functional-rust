(* 748: Property-Based Testing — OCaml minimal QuickCheck *)

(* Simple LCG random number generator *)
let seed = ref 12345

let next_int () =
  seed := (!seed * 1664525 + 1013904223) land 0x7FFFFFFF;
  !seed

let rand_int lo hi =
  lo + (next_int () mod (hi - lo + 1))

let rand_list gen lo hi max_len =
  let len = rand_int 0 max_len in
  List.init len (fun _ -> gen lo hi)

(* Property: for all lists, sort is idempotent *)
let prop_sort_idempotent lst =
  let sorted1 = List.sort compare lst in
  let sorted2 = List.sort compare sorted1 in
  sorted1 = sorted2

(* Property: sort preserves length *)
let prop_sort_length lst =
  List.length (List.sort compare lst) = List.length lst

(* Property: sort result is ordered *)
let prop_sort_ordered lst =
  let sorted = List.sort compare lst in
  let rec check = function
    | [] | [_] -> true
    | x :: y :: rest -> x <= y && check (y :: rest)
  in
  check sorted

(* forall: run property on N random inputs *)
let forall n gen prop name =
  let failed = ref 0 in
  for _ = 1 to n do
    let input = gen (-100) 100 20 in
    if not (prop input) then incr failed
  done;
  if !failed = 0
  then Printf.printf "✓ %s: %d tests passed\n" name n
  else Printf.printf "✗ %s: %d/%d FAILED\n" name !failed n

let () =
  forall 1000 (rand_list rand_int) prop_sort_idempotent "sort is idempotent";
  forall 1000 (rand_list rand_int) prop_sort_length    "sort preserves length";
  forall 1000 (rand_list rand_int) prop_sort_ordered   "sort result is ordered"
