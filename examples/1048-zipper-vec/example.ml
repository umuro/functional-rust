(* 1048: Zipper — Cursor over a Sequence
   The zipper pattern is a classic functional data structure.
   In OCaml it's especially natural: left (reversed) * focus * right. *)

type 'a zipper = {
  left  : 'a list;   (* reversed: closest to focus is head *)
  focus : 'a;
  right : 'a list;
}

let from_list = function
  | [] -> None
  | x :: rest -> Some { left = []; focus = x; right = rest }

(* Reconstruct the full sequence *)
let to_list z =
  List.rev_append z.left (z.focus :: z.right)

let move_right z =
  match z.right with
  | [] -> None
  | x :: rest -> Some { left = z.focus :: z.left; focus = x; right = rest }

let move_left z =
  match z.left with
  | [] -> None
  | x :: rest -> Some { left = rest; focus = x; right = z.focus :: z.right }

let move_to_start z =
  let rec go z = match move_left z with None -> z | Some z' -> go z' in
  go z

let move_to_end z =
  let rec go z = match move_right z with None -> z | Some z' -> go z' in
  go z

let set value z = { z with focus = value }

let modify f z = { z with focus = f z.focus }

let insert_right value z = { z with right = value :: z.right }

let insert_left value z = { z with left = value :: z.left }

let delete_right z =
  match z.right with
  | [] -> (None, z)
  | x :: rest -> (Some x, { z with right = rest })

let () =
  (* Navigation *)
  let z0 = Option.get (from_list [1; 2; 3; 4; 5]) in
  assert (z0.focus = 1);
  let z1 = Option.get (move_right z0) in
  assert (z1.focus = 2);
  let z2 = Option.get (move_right z1) in
  assert (z2.focus = 3);
  let z3 = Option.get (move_left z2) in
  assert (z3.focus = 2);
  assert (to_list z3 = [1; 2; 3; 4; 5]);

  (* Modification *)
  let z = Option.get (from_list [1; 2; 3; 4; 5]) in
  let z = Option.get (move_right z) in
  let z = Option.get (move_right z) in
  let z = set 99 z in
  assert (to_list z = [1; 2; 99; 4; 5]);
  let z = modify (fun x -> x * 2) z in
  assert (z.focus = 198);

  (* Editor: insert 'l' after 'l' in "helo" -> "hello" *)
  let z = Option.get (from_list ['h'; 'e'; 'l'; 'o']) in
  let z = Option.get (move_right z) in  (* e *)
  let z = Option.get (move_right z) in  (* l *)
  let z = insert_right 'l' z in
  assert (to_list z = ['h'; 'e'; 'l'; 'l'; 'o']);

  (* Boundaries *)
  let z = Option.get (from_list [1]) in
  assert (move_left z = None);
  assert (move_right z = None);
  assert (z.focus = 1);

  (* Move to extremes *)
  let z = Option.get (from_list [1; 2; 3; 4; 5]) in
  let z = move_to_end z in
  assert (z.focus = 5);
  let z = move_to_start z in
  assert (z.focus = 1);

  Printf.printf "All zipper tests passed.\n"
