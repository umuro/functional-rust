(* Lifetime cheatsheet in OCaml -- none needed! *)
(* This file shows the patterns OCaml doesn't need annotations for *)

(* 1. Function returning borrowed value *)
let identity x = x

(* 2. Struct with "borrowed" fields -- OCaml owns them *)
type config = { host: string; port: int }

(* 3. Multiple input references *)
let prefer_first a _b = a

(* 4. Static values *)
let static_name = "constant"

(* 5. Generic function *)
let apply f x = f x

(* 6. Nested references *)
let deref_and_use r = !r

let () =
  let cfg = { host = "localhost"; port = 8080 } in
  Printf.printf "host: %s\n" cfg.host;
  Printf.printf "prefer_first: %s\n" (prefer_first "a" "b");
  Printf.printf "apply double 5: %d\n" (apply (fun x -> x * 2) 5);
  let r = ref 42 in
  Printf.printf "deref: %d\n" (deref_and_use r)
