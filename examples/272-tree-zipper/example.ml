(* Tree zipper — O(1) local navigation and functional editing *)

type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

(* A crumb records the direction taken and what was left behind *)
type 'a crumb = Left of 'a * 'a tree | Right of 'a tree * 'a
type 'a zipper = { focus: 'a tree; trail: 'a crumb list }

(* --- Navigation --- *)

let of_tree t = { focus = t; trail = [] }

let go_left z = match z.focus with
  | Leaf -> None
  | Node (l, v, r) -> Some { focus = l; trail = Left (v, r) :: z.trail }

let go_right z = match z.focus with
  | Leaf -> None
  | Node (l, v, r) -> Some { focus = r; trail = Right (l, v) :: z.trail }

let go_up z = match z.trail with
  | [] -> None
  | Left (v, r) :: rest -> Some { focus = Node (z.focus, v, r); trail = rest }
  | Right (l, v) :: rest -> Some { focus = Node (l, v, z.focus); trail = rest }

(* --- Editing --- *)

let set_value x z = match z.focus with
  | Leaf -> z
  | Node (l, _, r) -> { z with focus = Node (l, x, r) }

(* Idiomatic: climb to root using tail recursion *)
let rec to_tree z = match go_up z with
  | None    -> z.focus
  | Some z' -> to_tree z'

(* --- Tests --- *)

let () =
  let tree = Node (Node (Leaf, 1, Leaf), 2, Node (Leaf, 3, Leaf)) in
  let z = of_tree tree in

  (* go_left moves focus to left child *)
  let z_left = Option.get (go_left z) in
  assert (z_left.focus = Node (Leaf, 1, Leaf));

  (* go_right moves focus to right child *)
  let z_right = Option.get (go_right z) in
  assert (z_right.focus = Node (Leaf, 3, Leaf));

  (* go_up after go_left reconstructs root *)
  let z_up = Option.get (go_up z_left) in
  assert (z_up.focus = tree);

  (* set_value then to_tree rebuilds whole tree *)
  let z_edited = set_value 10 z_left in
  let result = to_tree z_edited in
  assert (result = Node (Node (Leaf, 10, Leaf), 2, Node (Leaf, 3, Leaf)));

  (* go_left on Leaf returns None *)
  assert (go_left { focus = Leaf; trail = [] } = None);

  (* go_up at root returns None *)
  assert (go_up { focus = tree; trail = [] } = None);

  ignore z_right;
  print_endline "ok"
