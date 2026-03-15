(* 063: Stack Module *)
(* Functional stack with module encapsulation *)

(* Approach 1: Simple module *)
module Stack = struct
  type 'a t = 'a list

  let empty = []
  let is_empty = function [] -> true | _ -> false
  let push x s = x :: s
  let pop = function
    | [] -> None
    | _ :: xs -> Some xs
  let peek = function
    | [] -> None
    | x :: _ -> Some x
  let size = List.length
  let to_list s = s
end

(* Approach 2: With signature hiding *)
module type STACK = sig
  type 'a t
  val empty : 'a t
  val is_empty : 'a t -> bool
  val push : 'a -> 'a t -> 'a t
  val pop : 'a t -> 'a t option
  val peek : 'a t -> 'a option
  val size : 'a t -> int
end

module SafeStack : STACK = struct
  type 'a t = 'a list
  let empty = []
  let is_empty = function [] -> true | _ -> false
  let push x s = x :: s
  let pop = function [] -> None | _ :: xs -> Some xs
  let peek = function [] -> None | x :: _ -> Some x
  let size = List.length
end

(* Approach 3: Stack with fold *)
let stack_of_list lst =
  List.fold_left (fun s x -> Stack.push x s) Stack.empty lst

let stack_sum s =
  List.fold_left ( + ) 0 (Stack.to_list s)

(* Tests *)
let () =
  let s = Stack.empty in
  assert (Stack.is_empty s);
  let s = Stack.push 1 s in
  let s = Stack.push 2 s in
  let s = Stack.push 3 s in
  assert (not (Stack.is_empty s));
  assert (Stack.peek s = Some 3);
  assert (Stack.size s = 3);
  let s = match Stack.pop s with Some s -> s | None -> s in
  assert (Stack.peek s = Some 2);
  let s2 = stack_of_list [1; 2; 3] in
  assert (Stack.peek s2 = Some 3);
  assert (stack_sum s2 = 6);
  Printf.printf "✓ All tests passed\n"
