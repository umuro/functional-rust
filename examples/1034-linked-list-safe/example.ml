(* 1034: Safe Singly-Linked List
   In OCaml, algebraic data types make linked lists the natural default.
   This shows the idiomatic OCaml list type and a mutable stack variant. *)

(* Approach 1: OCaml's built-in list IS a singly-linked list *)
let list_demo () =
  (* Prepend = O(1), access head = O(1) *)
  let lst = 3 :: 2 :: 1 :: [] in
  assert (List.length lst = 3);
  assert (List.hd lst = 3);
  let tl = List.tl lst in
  assert (tl = [2; 1]);
  let sum = List.fold_left ( + ) 0 lst in
  assert (sum = 6);
  let evens = List.filter (fun x -> x mod 2 = 0) [1;2;3;4;5] in
  assert (evens = [2; 4])

(* Approach 2: Explicit linked list type (mirrors Rust's Node<T>) *)
type 'a node =
  | Nil
  | Cons of 'a * 'a node

let push value lst = Cons (value, lst)

let pop = function
  | Nil -> (None, Nil)
  | Cons (v, rest) -> (Some v, rest)

let peek = function
  | Nil -> None
  | Cons (v, _) -> Some v

let rec length = function
  | Nil -> 0
  | Cons (_, rest) -> 1 + length rest

let rec to_list = function
  | Nil -> []
  | Cons (v, rest) -> v :: to_list rest

let node_demo () =
  let lst = push 3 (push 2 (push 1 Nil)) in
  assert (length lst = 3);
  assert (peek lst = Some 3);
  let (v, rest) = pop lst in
  assert (v = Some 3);
  let (v2, _) = pop rest in
  assert (v2 = Some 2);
  assert (to_list lst = [3; 2; 1])

(* Approach 3: Mutable stack using ref (OCaml's safe mutation via ref) *)
type 'a stack = { mutable head : 'a node }

let make_stack () = { head = Nil }

let stack_push s value =
  s.head <- Cons (value, s.head)

let stack_pop s =
  match s.head with
  | Nil -> None
  | Cons (v, rest) ->
    s.head <- rest;
    Some v

let stack_demo () =
  let s = make_stack () in
  stack_push s 1;
  stack_push s 2;
  stack_push s 3;
  assert (peek s.head = Some 3);
  assert (stack_pop s = Some 3);
  assert (stack_pop s = Some 2);
  assert (stack_pop s = Some 1);
  assert (stack_pop s = None)

let () =
  list_demo ();
  node_demo ();
  stack_demo ();
  Printf.printf "All singly-linked list tests passed.\n"
