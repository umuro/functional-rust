(* 1044: Sorted Vec — Maintained-order List with Binary Search
   OCaml's Set module gives O(log n) insert/lookup with balanced BST.
   We also show a sorted array approach for direct binary search. *)

(* Approach 1: OCaml Set (always sorted, no duplicates) *)
module IntSet = Set.Make(Int)

let set_demo () =
  let s = IntSet.of_list [5; 3; 7; 1; 4] in
  let sorted = IntSet.elements s in
  assert (sorted = [1; 3; 4; 5; 7]);
  assert (IntSet.mem 4 s);
  assert (not (IntSet.mem 6 s));
  let s2 = IntSet.remove 3 s in
  assert (not (IntSet.mem 3 s2))

(* Approach 2: Sorted array with binary search and range query *)
(* Binary search: returns index of value or insertion point *)
let binary_search arr value =
  let lo = ref 0 and hi = ref (Array.length arr) in
  while !lo < !hi do
    let mid = (!lo + !hi) / 2 in
    if arr.(mid) < value then lo := mid + 1
    else hi := mid
  done;
  (* !lo is insertion point; check if exact match *)
  if !lo < Array.length arr && arr.(!lo) = value then Ok !lo
  else Error !lo

(* Sorted vec backed by array — insert maintains order *)
let sorted_insert arr value =
  let insert_pos = match binary_search arr value with
    | Ok i -> i
    | Error i -> i
  in
  let n = Array.length arr in
  let result = Array.make (n + 1) value in
  Array.blit arr 0 result 0 insert_pos;
  result.(insert_pos) <- value;
  Array.blit arr insert_pos result (insert_pos + 1) (n - insert_pos);
  result

let sorted_contains arr value =
  match binary_search arr value with Ok _ -> true | Error _ -> false

let sorted_find arr value =
  match binary_search arr value with Ok i -> Some i | Error _ -> None

(* Range query: elements in [lo, hi] *)
let range_query arr lo hi =
  let start = match binary_search arr lo with
    | Ok i -> i
    | Error i -> i
  in
  (* Find end: first element > hi *)
  let hi_end = ref (Array.length arr) in
  let lo_h = ref start and hi_h = ref (Array.length arr) in
  while !lo_h < !hi_h do
    let mid = (!lo_h + !hi_h) / 2 in
    if arr.(mid) <= hi then lo_h := mid + 1
    else hi_h := mid
  done;
  hi_end := !lo_h;
  Array.sub arr start (!hi_end - start)

let () =
  set_demo ();

  (* Build sorted array incrementally *)
  let arr = ref [||] in
  List.iter (fun x -> arr := sorted_insert !arr x) [5; 3; 7; 1; 4];
  assert (!arr = [|1; 3; 4; 5; 7|]);
  assert (sorted_contains !arr 4);
  assert (not (sorted_contains !arr 6));
  assert (sorted_find !arr 5 = Some 3);

  (* Range query *)
  let arr2 = Array.of_list [1; 3; 5; 7; 9; 11] in
  assert (range_query arr2 3 9 = [|3; 5; 7; 9|]);
  assert (range_query arr2 4 8 = [|5; 7|]);
  assert (range_query arr2 20 30 = [||]);

  (* Duplicates in sorted list using List.sort *)
  let sorted_dupes = List.sort compare [1; 1; 2] in
  assert (sorted_dupes = [1; 1; 2]);

  Printf.printf "All sorted-vec tests passed.\n"
