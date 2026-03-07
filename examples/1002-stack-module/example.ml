module type STACK = sig
  type 'a t
  exception Empty

  val empty    : 'a t
  val is_empty : 'a t -> bool
  val push     : 'a -> 'a t -> 'a t
  val peek     : 'a t -> 'a      (* raises Empty *)
  val pop      : 'a t -> 'a t    (* raises Empty *)
  val size     : 'a t -> int
end

module ListStack : STACK = struct
  type 'a t = 'a list
  exception Empty

  let empty        = []
  let is_empty s   = s = []
  let push x s     = x :: s
  let size s       = List.length s

  let peek = function
    | []     -> raise Empty
    | x :: _ -> x

  let pop = function
    | []      -> raise Empty
    | _ :: s  -> s
end

let () =
  let open ListStack in
  let s = empty |> push 1 |> push 2 |> push 3 in
  Printf.printf "size = %d\n" (size s);
  Printf.printf "peek = %d\n" (peek s);
  let s' = pop s in
  Printf.printf "after pop, peek = %d\n" (peek s')
