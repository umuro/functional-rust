(* 063: Stack Module
   Mutable stack via module + persistent (functional) stack via variants *)

(* --- Approach 1: Mutable stack as a module --- *)

module MutableStack = struct
  type 'a t = { mutable elements: 'a list }

  let create () = { elements = [] }
  let is_empty s = s.elements = []
  let push s x = s.elements <- x :: s.elements
  let pop s =
    match s.elements with
    | []      -> None
    | x :: rest -> s.elements <- rest; Some x
  let peek s = match s.elements with [] -> None | x :: _ -> Some x
  let size s = List.length s.elements
end

(* --- Approach 2: Persistent (immutable) stack — just a list with a nice API --- *)

module PersistentStack = struct
  type 'a t = 'a list

  let empty : 'a t = []
  let is_empty = function [] -> true | _ -> false
  let push x s = x :: s           (* returns a new stack *)
  let pop = function [] -> None | _ :: rest -> Some rest
  let peek = function [] -> None | x :: _ -> Some x
end

(* --- Approach 3: Fold-based stack operations --- *)

let of_list xs = List.fold_left (fun acc x -> x :: acc) [] xs

let () =
  (* mutable *)
  let s = MutableStack.create () in
  MutableStack.push s 1;
  MutableStack.push s 2;
  MutableStack.push s 3;
  Printf.printf "mutable peek = %s\n"
    (match MutableStack.peek s with Some v -> string_of_int v | None -> "None");
  Printf.printf "mutable pop  = %s\n"
    (match MutableStack.pop s with Some v -> string_of_int v | None -> "None");
  Printf.printf "mutable size = %d\n" (MutableStack.size s);

  (* persistent *)
  let open PersistentStack in
  let s0 = push 3 (push 2 (push 1 empty)) in
  let s1 = Option.value ~default:empty (pop s0) in
  Printf.printf "persistent peek before pop = %s\n"
    (match peek s0 with Some v -> string_of_int v | None -> "None");
  Printf.printf "persistent peek after pop  = %s\n"
    (match peek s1 with Some v -> string_of_int v | None -> "None");
  (* original untouched *)
  Printf.printf "original peek still        = %s\n"
    (match peek s0 with Some v -> string_of_int v | None -> "None");

  Printf.printf "of_list [1;2;3] top = %s\n"
    (match of_list [1;2;3] with x :: _ -> string_of_int x | [] -> "empty")
