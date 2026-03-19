(* OCaml implementation of Hello World *)

let greet (name : string) : string = 
  "Hello, " ^ name ^ "!"

(* Test cases *)
let () = 
  print_endline "--- Hello World Tests ---";
  let msg1 = greet "world" in
  Printf.printf "greet \"world\": %s (Expected: Hello, world!)\n" msg1;
  assert (msg1 = "Hello, world!");

  let msg2 = greet "OCaml" in
  Printf.printf "greet \"OCaml\": %s (Expected: Hello, OCaml!)\n" msg2;
  assert (msg2 = "Hello, OCaml!");

  let msg3 = greet "" in
  Printf.printf "greet \"\": %s (Expected: Hello, !)\n" msg3;
  assert (msg3 = "Hello, !");

  print_endline "All Hello World tests passed!";
