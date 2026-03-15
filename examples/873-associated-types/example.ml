(* Example 079: Associated Types *)
(* OCaml module types → Rust associated types *)

(* Approach 1: Module type with associated type *)
module type Container = sig
  type t
  type item
  val empty : t
  val push : item -> t -> t
  val pop : t -> (item * t) option
  val is_empty : t -> bool
  val size : t -> int
end

(* Stack implementation *)
module Stack : Container with type item = int = struct
  type item = int
  type t = int list
  let empty = []
  let push x xs = x :: xs
  let pop = function [] -> None | x :: xs -> Some (x, xs)
  let is_empty = function [] -> true | _ -> false
  let size = List.length
end

(* Approach 2: Module type with type output *)
module type Addable = sig
  type t
  type output
  val add : t -> t -> output
end

module IntAdd : Addable with type t = int and type output = int = struct
  type t = int
  type output = int
  let add a b = a + b
end

module FloatAdd : Addable with type t = float and type output = float = struct
  type t = float
  type output = float
  let add a b = a +. b
end

(* Approach 3: Functor with associated output *)
module type Transformer = sig
  type input
  type output
  val transform : input -> output
end

module StringLen : Transformer with type input = string and type output = int = struct
  type input = string
  type output = int
  let transform = String.length
end

(* Tests *)
let () =
  let s = Stack.empty in
  let s = Stack.push 1 s in
  let s = Stack.push 2 s in
  let s = Stack.push 3 s in
  assert (Stack.size s = 3);
  assert (not (Stack.is_empty s));
  (match Stack.pop s with
   | Some (v, rest) ->
     assert (v = 3);
     assert (Stack.size rest = 2)
   | None -> assert false);

  assert (IntAdd.add 3 4 = 7);
  assert (FloatAdd.add 1.5 2.5 = 4.0);
  assert (StringLen.transform "hello" = 5);

  Printf.printf "✓ All tests passed\n"
