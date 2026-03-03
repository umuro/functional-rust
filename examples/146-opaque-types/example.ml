(* Opaque types: the type's representation is hidden by the module signature.
   Callers can only use the type through the provided interface. *)

module type STACK = sig
  type 'a t
  val empty  : 'a t
  val push   : 'a -> 'a t -> 'a t
  val pop    : 'a t -> ('a * 'a t) option
  val peek   : 'a t -> 'a option
  val size   : 'a t -> int
  val is_empty : 'a t -> bool
end

(* List-based implementation — type is opaque outside this module *)
module Stack : STACK = struct
  type 'a t = 'a list
  let empty       = []
  let push x s    = x :: s
  let pop = function
    | []    -> None
    | x :: xs -> Some (x, xs)
  let peek = function
    | []    -> None
    | x :: _ -> Some x
  let size        = List.length
  let is_empty    = (= [])
end

let () =
  let s = Stack.(push 3 (push 2 (push 1 empty))) in
  Printf.printf "size: %d\n"    (Stack.size s);
  Printf.printf "peek: %d\n"    (Option.get (Stack.peek s));
  (match Stack.pop s with
   | Some (top, rest) ->
     Printf.printf "popped: %d, new size: %d\n" top (Stack.size rest)
   | None -> assert false);
  Printf.printf "empty: %b\n"   (Stack.is_empty Stack.empty)
