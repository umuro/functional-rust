(* 1039: Stack Using List *)
(* OCaml's list IS a stack — cons and pattern match are push and pop *)

(* Approach 1: List as a stack (idiomatic OCaml) *)
let list_stack () =
  let stack = [] in
  let stack = 1 :: stack in
  let stack = 2 :: stack in
  let stack = 3 :: stack in
  assert (List.hd stack = 3);  (* top *)
  let (top, stack) = (List.hd stack, List.tl stack) in
  assert (top = 3);
  assert (List.hd stack = 2)

(* Approach 2: Module-based stack *)
module Stack : sig
  type 'a t
  val empty : 'a t
  val push : 'a -> 'a t -> 'a t
  val pop : 'a t -> ('a * 'a t) option
  val peek : 'a t -> 'a option
  val is_empty : 'a t -> bool
  val size : 'a t -> int
  val to_list : 'a t -> 'a list
end = struct
  type 'a t = { items: 'a list; size: int }

  let empty = { items = []; size = 0 }
  let push x s = { items = x :: s.items; size = s.size + 1 }
  let pop s = match s.items with
    | [] -> None
    | x :: xs -> Some (x, { items = xs; size = s.size - 1 })
  let peek s = match s.items with
    | [] -> None
    | x :: _ -> Some x
  let is_empty s = s.items = []
  let size s = s.size
  let to_list s = s.items
end

let module_stack () =
  let s = Stack.empty in
  let s = Stack.push 10 s in
  let s = Stack.push 20 s in
  let s = Stack.push 30 s in
  assert (Stack.size s = 3);
  assert (Stack.peek s = Some 30);
  let (v, s) = Option.get (Stack.pop s) in
  assert (v = 30);
  assert (Stack.peek s = Some 20)

(* Approach 3: Stack-based expression evaluator *)
let eval_rpn tokens =
  let stack = List.fold_left (fun stack token ->
    match token with
    | "+" | "-" | "*" ->
      let b = List.hd stack in
      let a = List.hd (List.tl stack) in
      let rest = List.tl (List.tl stack) in
      let result = match token with
        | "+" -> a + b
        | "-" -> a - b
        | "*" -> a * b
        | _ -> failwith "impossible"
      in
      result :: rest
    | n -> int_of_string n :: stack
  ) [] tokens in
  List.hd stack

let eval_test () =
  (* 3 4 + 2 * = (3 + 4) * 2 = 14 *)
  let result = eval_rpn ["3"; "4"; "+"; "2"; "*"] in
  assert (result = 14)

let () =
  list_stack ();
  module_stack ();
  eval_test ();
  Printf.printf "✓ All tests passed\n"
