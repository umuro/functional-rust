(* Command pattern in OCaml *)
type cmd =
  | Add    of string * int
  | Remove of string
  | Set    of string * int
  | Clear

type store = (string * int) list

let execute store = function
  | Add(k,v)    -> (k,v) :: List.filter (fun (k',_)->k'<>k) store
  | Remove k    -> List.filter (fun (k',_)->k'<>k) store
  | Set(k,v)    -> (k,v) :: List.filter (fun (k',_)->k'<>k) store
  | Clear       -> []

let () =
  let cmds = [Add("x",1);Add("y",2);Set("x",10);Remove("y");Add("z",3)] in
  let store = List.fold_left execute [] cmds in
  List.iter (fun (k,v)->Printf.printf "%s=%d\n" k v) store
