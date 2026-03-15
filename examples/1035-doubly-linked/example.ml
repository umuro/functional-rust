(* 1035: Doubly-Linked List *)
(* OCaml: immutable doubly-linked lists are impractical *)
(* We use a zipper (functional approach) or mutable records *)

(* Approach 1: Zipper as functional doubly-linked list *)
type 'a zipper = {
  left: 'a list;   (* reversed left part *)
  focus: 'a;
  right: 'a list;
}

let of_list = function
  | [] -> None
  | x :: xs -> Some { left = []; focus = x; right = xs }

let move_right z =
  match z.right with
  | [] -> None
  | x :: xs -> Some { left = z.focus :: z.left; focus = x; right = xs }

let move_left z =
  match z.left with
  | [] -> None
  | x :: xs -> Some { left = xs; focus = x; right = z.focus :: z.right }

let to_list z =
  List.rev z.left @ [z.focus] @ z.right

let zipper_test () =
  let z = Option.get (of_list [1; 2; 3; 4; 5]) in
  assert (z.focus = 1);
  let z = Option.get (move_right z) in
  assert (z.focus = 2);
  let z = Option.get (move_right z) in
  assert (z.focus = 3);
  let z = Option.get (move_left z) in
  assert (z.focus = 2);
  assert (to_list z = [1; 2; 3; 4; 5])

(* Approach 2: Mutable doubly-linked using records *)
type 'a dnode = {
  mutable value: 'a;
  mutable prev: 'a dnode option;
  mutable next: 'a dnode option;
}

type 'a dlist = {
  mutable head: 'a dnode option;
  mutable tail: 'a dnode option;
  mutable length: int;
}

let create () = { head = None; tail = None; length = 0 }

let push_back dl v =
  let node = { value = v; prev = dl.tail; next = None } in
  (match dl.tail with
   | Some t -> t.next <- Some node
   | None -> dl.head <- Some node);
  dl.tail <- Some node;
  dl.length <- dl.length + 1

let push_front dl v =
  let node = { value = v; prev = None; next = dl.head } in
  (match dl.head with
   | Some h -> h.prev <- Some node
   | None -> dl.tail <- Some node);
  dl.head <- Some node;
  dl.length <- dl.length + 1

let to_list_forward dl =
  let rec go acc = function
    | None -> List.rev acc
    | Some n -> go (n.value :: acc) n.next
  in
  go [] dl.head

let mutable_dlist_test () =
  let dl = create () in
  push_back dl 1;
  push_back dl 2;
  push_back dl 3;
  push_front dl 0;
  assert (to_list_forward dl = [0; 1; 2; 3]);
  assert (dl.length = 4)

let () =
  zipper_test ();
  mutable_dlist_test ();
  Printf.printf "✓ All tests passed\n"
