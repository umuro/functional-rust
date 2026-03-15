(* 1034: Safe Linked List *)
(* OCaml lists ARE linked lists — this is the native data structure *)

(* Approach 1: OCaml's built-in list (singly-linked, immutable) *)
let builtin_list () =
  let lst = [1; 2; 3] in
  (* Cons is O(1) *)
  let lst = 0 :: lst in
  assert (lst = [0; 1; 2; 3]);
  (* Pattern match for head/tail *)
  let (hd, tl) = match lst with
    | x :: xs -> (x, xs)
    | [] -> failwith "empty"
  in
  assert (hd = 0);
  assert (tl = [1; 2; 3])

(* Approach 2: Custom linked list type (explicit) *)
type 'a node = Nil | Cons of 'a * 'a node

let rec to_list = function
  | Nil -> []
  | Cons (x, next) -> x :: to_list next

let rec from_list = function
  | [] -> Nil
  | x :: xs -> Cons (x, from_list xs)

let rec length = function
  | Nil -> 0
  | Cons (_, next) -> 1 + length next

let push x lst = Cons (x, lst)

let pop = function
  | Nil -> None
  | Cons (x, next) -> Some (x, next)

let custom_list () =
  let lst = from_list [1; 2; 3] in
  assert (length lst = 3);
  let lst = push 0 lst in
  assert (length lst = 4);
  let (v, lst) = Option.get (pop lst) in
  assert (v = 0);
  assert (to_list lst = [1; 2; 3])

(* Approach 3: Functional operations on custom list *)
let rec map f = function
  | Nil -> Nil
  | Cons (x, next) -> Cons (f x, map f next)

let rec filter p = function
  | Nil -> Nil
  | Cons (x, next) ->
    if p x then Cons (x, filter p next)
    else filter p next

let rec fold f acc = function
  | Nil -> acc
  | Cons (x, next) -> fold f (f acc x) next

let functional_ops () =
  let lst = from_list [1; 2; 3; 4; 5] in
  let doubled = map (fun x -> x * 2) lst in
  assert (to_list doubled = [2; 4; 6; 8; 10]);
  let evens = filter (fun x -> x mod 2 = 0) lst in
  assert (to_list evens = [2; 4]);
  let sum = fold (fun acc x -> acc + x) 0 lst in
  assert (sum = 15)

let () =
  builtin_list ();
  custom_list ();
  functional_ops ();
  Printf.printf "✓ All tests passed\n"
