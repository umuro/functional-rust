(* Labeled and Optional Arguments *)
(* Named parameters and default values *)

let create_greeting ?(title="Mr.") ?(greeting="Hello") ~name () =
  Printf.sprintf "%s, %s %s!" greeting title name

let () =
  print_endline (create_greeting ~name:"Smith" ());
  print_endline (create_greeting ~title:"Dr." ~name:"Jones" ());
  print_endline (create_greeting ~greeting:"Dear" ~title:"Prof." ~name:"Lee" ())

(* Optional with default *)
let pad ?(char=' ') ?(width=20) s =
  let len = String.length s in
  if len >= width then s
  else s ^ String.make (width - len) char

let () = Printf.printf "[%s]\n" (pad "hello")
let () = Printf.printf "[%s]\n" (pad ~char:'.' ~width:15 "hello")
