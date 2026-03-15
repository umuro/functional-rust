(* 103: Shared Borrowing *)
(* OCaml: immutable by default, no borrow checker needed *)

(* Approach 1: Multiple "readers" — always safe in OCaml *)
let sum_list lst = List.fold_left ( + ) 0 lst
let count_list lst = List.length lst

let multiple_readers () =
  let data = [1; 2; 3; 4; 5] in
  let s = sum_list data in      (* "borrow" 1 *)
  let c = count_list data in    (* "borrow" 2 — no problem *)
  let avg = float_of_int s /. float_of_int c in
  avg

(* Approach 2: Passing by reference (mutable structures) *)
let read_array_twice arr =
  let first = arr.(0) in
  let last = arr.(Array.length arr - 1) in
  (first, last)

(* Tests *)
let () =
  assert (abs_float (multiple_readers () -. 3.0) < 0.001);
  assert (read_array_twice [|10; 20; 30|] = (10, 30));
  Printf.printf "✓ All tests passed\n"
