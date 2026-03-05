(* Robot Name *)
(* Mutable state with unique name generation *)

type robot = { mutable name_str : string }

let used_names = Hashtbl.create 1000

let random_name () =
  let letter () = Char.chr (Char.code 'A' + Random.int 26) in
  let digit () = Char.chr (Char.code '0' + Random.int 10) in
  Printf.sprintf "%c%c%c%c%c" (letter ()) (letter ()) (digit ()) (digit ()) (digit ())

let fresh_name () =
  let rec try_name () =
    let n = random_name () in
    if Hashtbl.mem used_names n then try_name ()
    else (Hashtbl.add used_names n true; n)
  in try_name ()

let new_robot () = Random.self_init (); { name_str = fresh_name () }
let name r = r.name_str
let reset r = r.name_str <- fresh_name ()
