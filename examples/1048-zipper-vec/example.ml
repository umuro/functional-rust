(* 1048: Vec Zipper — Cursor with Left, Focus, Right *)
(* A zipper provides O(1) local navigation and modification *)

type 'a zipper = {
  left: 'a list;    (* reversed: closest element first *)
  focus: 'a;
  right: 'a list;
}

(* Construction *)
let of_list = function
  | [] -> None
  | x :: xs -> Some { left = []; focus = x; right = xs }

let to_list z =
  List.rev z.left @ [z.focus] @ z.right

(* Navigation *)
let move_right z =
  match z.right with
  | [] -> None
  | x :: xs -> Some { left = z.focus :: z.left; focus = x; right = xs }

let move_left z =
  match z.left with
  | [] -> None
  | x :: xs -> Some { left = xs; focus = x; right = z.focus :: z.right }

let move_to_start z =
  let rec go z =
    match move_left z with
    | None -> z
    | Some z' -> go z'
  in
  go z

let move_to_end z =
  let rec go z =
    match move_right z with
    | None -> z
    | Some z' -> go z'
  in
  go z

(* Modification at focus *)
let set value z = { z with focus = value }
let modify f z = { z with focus = f z.focus }
let insert_right value z = { z with right = value :: z.right }
let insert_left value z = { z with left = value :: z.left }

let delete_right z =
  match z.right with
  | [] -> None
  | _ :: xs -> Some { z with right = xs }

(* Approach 1: Basic navigation *)
let navigation_test () =
  let z = Option.get (of_list [1; 2; 3; 4; 5]) in
  assert (z.focus = 1);
  let z = Option.get (move_right z) in
  assert (z.focus = 2);
  let z = Option.get (move_right z) in
  assert (z.focus = 3);
  let z = Option.get (move_left z) in
  assert (z.focus = 2);
  assert (to_list z = [1; 2; 3; 4; 5])

(* Approach 2: Modification *)
let modification_test () =
  let z = Option.get (of_list [1; 2; 3; 4; 5]) in
  let z = Option.get (move_right z) in
  let z = Option.get (move_right z) in
  let z = set 99 z in  (* Change 3 to 99 *)
  assert (to_list z = [1; 2; 99; 4; 5]);
  let z = modify (fun x -> x * 2) z in  (* 99 -> 198 *)
  assert (z.focus = 198)

(* Approach 3: Text editor cursor simulation *)
let editor_test () =
  let z = Option.get (of_list ['h'; 'e'; 'l'; 'o']) in
  let z = Option.get (move_right z) in
  let z = Option.get (move_right z) in
  (* Insert 'l' after current position *)
  let z = insert_right 'l' z in
  assert (to_list z = ['h'; 'e'; 'l'; 'l'; 'o'])

let () =
  navigation_test ();
  modification_test ();
  editor_test ();
  Printf.printf "✓ All tests passed\n"
