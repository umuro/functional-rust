(* vec!-like macros in OCaml — collection literals *)

(* OCaml has literal list syntax; simulate for other structures *)

module type Collection = sig
  type 'a t
  val empty : 'a t
  val add : 'a -> 'a t -> 'a t
  val to_list : 'a t -> 'a list
end

module SimpleSet = struct
  type 'a t = 'a list
  let empty = []
  let add x s = if List.mem x s then s else x :: s
  let to_list x = List.sort compare x
end

(* Simulate a "macro" using a variadic function *)
let set_of items =
  List.fold_left (fun s x -> SimpleSet.add x s) SimpleSet.empty items

let queue_of items =
  let q = Queue.create () in
  List.iter (Queue.add q) items;
  q

let () =
  let s = set_of [3;1;4;1;5;9;2;6;5;3;5] in
  Printf.printf "Set: [%s]\n"
    (String.concat "; " (List.map string_of_int (SimpleSet.to_list s)));
  let q = queue_of [1;2;3;4;5] in
  Printf.printf "Queue size: %d\n" (Queue.length q)
