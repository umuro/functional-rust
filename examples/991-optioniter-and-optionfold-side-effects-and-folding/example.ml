(* Option.iter and Option.fold — Side Effects and Folding *)
(* Perform actions on optional values *)

let maybe_name = Some "Alice"
let no_name : string option = None

let () = Option.iter (fun name -> Printf.printf "Hello, %s!\n" name) maybe_name
let () = Option.iter (fun name -> Printf.printf "Hello, %s!\n" name) no_name

let greeting = Option.fold ~none:"Hello, stranger!" ~some:(fun n -> "Hello, " ^ n ^ "!") maybe_name
let () = print_endline greeting
