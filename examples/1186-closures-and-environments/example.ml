(* Closures and Environments *)
(* Understanding closures and captured variables *)

(* A closure captures its environment *)
let make_greeting prefix suffix =
  fun name -> prefix ^ name ^ suffix

let hello = make_greeting "Hello, " "!"
let bye = make_greeting "Goodbye, " "."

let () =
  Printf.printf "%s\n" (hello "Alice");
  Printf.printf "%s\n" (bye "Bob")

(* Accumulator closure *)
let make_accumulator init =
  let total = ref init in
  fun amount ->
    total := !total + amount;
    !total

let acc = make_accumulator 100
let () = Printf.printf "Balance: %d %d %d\n" (acc 50) (acc (-30)) (acc 20)
