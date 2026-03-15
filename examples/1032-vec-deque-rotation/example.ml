(* 1032: VecDeque Rotation — Efficient Front/Back Operations *)
(* OCaml uses lists (efficient at front) or Queue module *)

(* Approach 1: List as a deque — efficient front, O(n) back *)
let list_deque () =
  let q = [1; 2; 3; 4; 5] in
  (* Push front: O(1) *)
  let q = 0 :: q in
  assert (q = [0; 1; 2; 3; 4; 5]);
  (* Pop front: O(1) *)
  let (front, q) = (List.hd q, List.tl q) in
  assert (front = 0);
  assert (q = [1; 2; 3; 4; 5]);
  (* Push back: O(n) *)
  let q = q @ [6] in
  assert (q = [1; 2; 3; 4; 5; 6])

(* Approach 2: Two-list queue for amortized O(1) both ends *)
type 'a deque = { front: 'a list; back: 'a list }

let empty = { front = []; back = [] }
let push_front x d = { d with front = x :: d.front }
let push_back x d = { d with back = x :: d.back }

let pop_front d =
  match d.front with
  | x :: rest -> Some (x, { d with front = rest })
  | [] ->
    match List.rev d.back with
    | x :: rest -> Some (x, { front = rest; back = [] })
    | [] -> None

let pop_back d =
  match d.back with
  | x :: rest -> Some (x, { d with back = rest })
  | [] ->
    match List.rev d.front with
    | x :: rest -> Some (x, { front = []; back = rest })
    | [] -> None

let two_list_deque () =
  let d = empty in
  let d = push_back 1 d in
  let d = push_back 2 d in
  let d = push_back 3 d in
  let d = push_front 0 d in
  let (v, d) = Option.get (pop_front d) in
  assert (v = 0);
  let (v, d) = Option.get (pop_front d) in
  assert (v = 1);
  let (v, _d) = Option.get (pop_back d) in
  assert (v = 3)

(* Approach 3: Rotation *)
let rotate_left lst n =
  let len = List.length lst in
  let n = n mod len in
  let rec take_drop i = function
    | [] -> ([], [])
    | x :: xs when i > 0 ->
      let (taken, rest) = take_drop (i - 1) xs in
      (x :: taken, rest)
    | xs -> ([], xs)
  in
  let (front, back) = take_drop n lst in
  back @ front

let rotation_test () =
  let lst = [1; 2; 3; 4; 5] in
  assert (rotate_left lst 2 = [3; 4; 5; 1; 2]);
  assert (rotate_left lst 0 = [1; 2; 3; 4; 5])

let () =
  list_deque ();
  two_list_deque ();
  rotation_test ();
  Printf.printf "✓ All tests passed\n"
