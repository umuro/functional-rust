(* Scott encoding: represent each constructor as a function that
   takes one continuation per constructor (i.e., a fold/eliminator).
   Unlike Church encoding, enables O(1) pattern matching. *)

(* Scott-encoded Option: none k_none k_some = k_none
                         some x k_none k_some = k_some x *)
let none   k_none _k_some     = k_none ()
let some x _k_none k_some     = k_some x

let is_none o = o (fun () -> true)  (fun _ -> false)
let is_some o = o (fun () -> false) (fun _ -> true)
let get_or def o = o (fun () -> def) (fun x -> x)
let map_opt f o = o (fun () -> none) (fun x -> some (f x))

(* Scott-encoded List *)
let snil  k_nil _k_cons       = k_nil ()
let scons x xs _k_nil k_cons  = k_cons x xs

let head_or def lst = lst (fun () -> def) (fun x _ -> x)
let tail_opt lst    = lst (fun () -> none) (fun _ xs -> some xs)

let rec to_list l =
  l (fun () -> []) (fun x xs -> x :: to_list xs)

let () =
  (* Option *)
  let n = none and s = some 42 in
  Printf.printf "is_none none  = %b\n" (is_none n);
  Printf.printf "is_some some  = %b\n" (is_some s);
  Printf.printf "get_or 0 some = %d\n" (get_or 0 s);

  let doubled = map_opt (fun x -> x * 2) s in
  Printf.printf "map *2 some42 = %d\n" (get_or 0 doubled);

  (* List *)
  let lst = scons 1 (scons 2 (scons 3 snil)) in
  Printf.printf "head = %d\n"  (head_or 0 lst);
  Printf.printf "list = [%s]\n" (to_list lst |> List.map string_of_int |> String.concat ";")
