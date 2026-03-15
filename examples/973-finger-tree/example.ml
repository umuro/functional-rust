(* 973: Finger Tree (Simplified) *)
(* Deque with O(1) amortized push/pop at both ends *)
(* Simplified: 2-3 finger tree with digit spines *)

(* Simplified finger tree as a 3-layer structure *)
(* Digit: 1-4 elements at each end *)
(* For teaching: we model a simplified version *)

type 'a digit =
  | One of 'a
  | Two of 'a * 'a
  | Three of 'a * 'a * 'a
  | Four of 'a * 'a * 'a * 'a

type 'a node =
  | Node2 of 'a * 'a
  | Node3 of 'a * 'a * 'a

type 'a finger_tree =
  | Empty
  | Single of 'a
  | Deep of 'a digit * 'a node finger_tree * 'a digit

(* Push to front *)
let push_front x = function
  | Empty -> Single x
  | Single y -> Deep (One x, Empty, One y)
  | Deep (One a, spine, r) -> Deep (Two (x, a), spine, r)
  | Deep (Two (a, b), spine, r) -> Deep (Three (x, a, b), spine, r)
  | Deep (Three (a, b, c), spine, r) -> Deep (Four (x, a, b, c), spine, r)
  | Deep (Four (a, b, c, d), spine, r) ->
    Deep (Two (x, a), push_front (Node3 (b, c, d)) spine, r)

(* Push to back *)
let push_back x = function
  | Empty -> Single x
  | Single y -> Deep (One y, Empty, One x)
  | Deep (l, spine, One a) -> Deep (l, spine, Two (a, x))
  | Deep (l, spine, Two (a, b)) -> Deep (l, spine, Three (a, b, x))
  | Deep (l, spine, Three (a, b, c)) -> Deep (l, spine, Four (a, b, c, x))
  | Deep (l, spine, Four (a, b, c, d)) ->
    Deep (l, push_back (Node3 (b, c, d)) spine, Two (a, x))

(* Convert to list (for testing) *)
let digit_to_list = function
  | One a -> [a]
  | Two (a, b) -> [a; b]
  | Three (a, b, c) -> [a; b; c]
  | Four (a, b, c, d) -> [a; b; c; d]

let node_to_list = function
  | Node2 (a, b) -> [a; b]
  | Node3 (a, b, c) -> [a; b; c]

let rec to_list = function
  | Empty -> []
  | Single x -> [x]
  | Deep (l, spine, r) ->
    digit_to_list l @
    List.concat_map node_to_list (to_list spine) @
    digit_to_list r

let () =
  let t = Empty in
  let t = push_back 1 t in
  let t = push_back 2 t in
  let t = push_back 3 t in
  let t = push_front 0 t in
  let t = push_back 4 t in
  let t = push_front (-1) t in

  let lst = to_list t in
  assert (lst = [-1; 0; 1; 2; 3; 4]);

  (* Build a longer sequence *)
  let t2 = List.fold_left (fun acc x -> push_back x acc) Empty [1;2;3;4;5;6;7;8;9;10] in
  assert (to_list t2 = [1;2;3;4;5;6;7;8;9;10]);

  Printf.printf "✓ All tests passed\n"
