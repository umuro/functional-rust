(* Example 103: Shared References — OCaml Read Access → Rust & Borrows *)

(* OCaml: all access is essentially "shared read" by default since
   immutable values can't be modified through any reference. *)

(* Approach 1: Passing values without consuming them *)
let string_info s =
  let len = String.length s in
  let upper = String.uppercase_ascii s in
  Printf.printf "String '%s' has length %d, upper: %s\n" s len upper;
  len

let approach1 () =
  let msg = "hello world" in
  let len1 = string_info msg in
  let len2 = string_info msg in  (* still available *)
  assert (len1 = len2);
  Printf.printf "Used msg twice, length = %d\n" len1

(* Approach 2: Multiple readers of same data *)
let sum_list lst = List.fold_left ( + ) 0 lst
let max_list lst = List.fold_left max min_int lst
let min_list lst = List.fold_left min max_int lst

let approach2 () =
  let data = [10; 20; 30; 40; 50] in
  let s = sum_list data in
  let mx = max_list data in
  let mn = min_list data in
  Printf.printf "sum=%d, max=%d, min=%d\n" s mx mn;
  assert (s = 150);
  assert (mx = 50);
  assert (mn = 10)

(* Approach 3: Sharing between data structures *)
let approach3 () =
  let shared = "shared string" in
  let pair = (shared, shared) in  (* both elements point to same string *)
  let lst = [shared; shared; shared] in
  assert (fst pair = shared);
  assert (List.hd lst = shared);
  Printf.printf "Shared '%s' across pair and list\n" shared

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
