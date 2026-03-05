(* Unit Type and Side Effects *)
(* Understanding unit type and sequencing side effects *)

(* Unit is the type of side effects *)
let greet name =
  Printf.printf "Hello, %s!\n" name;
  Printf.printf "Welcome to OCaml.\n"
  (* returns unit *)

let () = greet "World"

(* Semicolons sequence unit expressions *)
let count_down n =
  for i = n downto 1 do
    Printf.printf "%d... " i
  done;
  print_endline "Go!"

let () = count_down 5

(* ignore discards non-unit values *)
let () = ignore (1 + 2)
