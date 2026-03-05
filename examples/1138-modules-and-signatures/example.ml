(* Modules and Signatures *)
(* Define modules with signatures for encapsulation *)

module type STACK = sig
  type 'a t
  val empty : 'a t
  val push : 'a -> 'a t -> 'a t
  val pop : 'a t -> ('a * 'a t) option
  val is_empty : 'a t -> bool
end

module ListStack : STACK = struct
  type 'a t = 'a list
  let empty = []
  let push x s = x :: s
  let pop = function
    | [] -> None
    | x :: xs -> Some (x, xs)
  let is_empty = function [] -> true | _ -> false
end

let s = ListStack.(empty |> push 1 |> push 2 |> push 3)
let () = match ListStack.pop s with
  | Some (x, _) -> Printf.printf "Top: %d\n" x
  | None -> print_endline "Empty"
