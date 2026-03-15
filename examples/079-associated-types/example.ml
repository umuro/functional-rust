(* 079: Associated Types — OCaml module abstract types *)

(* Approach 1: Module type with associated type *)
module type Container = sig
  type t
  type item
  val empty : t
  val push : item -> t -> t
  val pop : t -> (item * t) option
  val is_empty : t -> bool
end

module IntStack : Container with type item = int = struct
  type t = int list
  type item = int
  let empty = []
  let push x s = x :: s
  let pop = function [] -> None | x :: xs -> Some (x, xs)
  let is_empty = function [] -> true | _ -> false
end

module StringQueue : Container with type item = string = struct
  type t = string list
  type item = string
  let empty = []
  let push x s = s @ [x]
  let pop = function [] -> None | x :: xs -> Some (x, xs)
  let is_empty = function [] -> true | _ -> false
end

(* Approach 2: Iterator-like with associated type *)
module type Iterator = sig
  type t
  type item
  val next : t -> (item * t) option
end

module RangeIter : Iterator with type item = int = struct
  type t = { current: int; stop: int }
  type item = int
  let next r =
    if r.current >= r.stop then None
    else Some (r.current, { r with current = r.current + 1 })
end

let collect_iter (type a) (module I : Iterator with type item = a) init =
  let rec aux acc state =
    match I.next state with
    | None -> List.rev acc
    | Some (item, next) -> aux (item :: acc) next
  in
  aux [] init

(* Tests *)
let () =
  let s = IntStack.push 3 (IntStack.push 2 (IntStack.push 1 IntStack.empty)) in
  assert (not (IntStack.is_empty s));
  (match IntStack.pop s with Some (v, _) -> assert (v = 3) | None -> assert false);
  let q = StringQueue.push "b" (StringQueue.push "a" StringQueue.empty) in
  (match StringQueue.pop q with Some (v, _) -> assert (v = "a") | None -> assert false);
  let range = RangeIter.{ current = 0; stop = 5 } in
  let items = collect_iter (module RangeIter) range in
  assert (items = [0; 1; 2; 3; 4]);
  Printf.printf "✓ All tests passed\n"
