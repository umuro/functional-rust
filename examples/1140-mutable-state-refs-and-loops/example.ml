(* Mutable State — Refs and Loops *)
(* Use references for mutable state *)

let counter = ref 0

let next () =
  let v = !counter in
  counter := v + 1;
  v

let () =
  for _ = 1 to 5 do
    Printf.printf "%d " (next ())
  done;
  print_newline ()

(* Imperative factorial *)
let factorial n =
  let result = ref 1 in
  for i = 2 to n do
    result := !result * i
  done;
  !result

let () = Printf.printf "10! = %d\n" (factorial 10)
