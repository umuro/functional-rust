(* OCaml: all memory access is safe by construction.
   Module boundaries and the type system enforce invariants automatically. *)

(** A simple mutable counter without any unsafe code. *)
let counter = ref 0

let increment () = incr counter
let get_count () = !counter
let reset ()     = counter := 0

let () =
  for _ = 1 to 5 do increment () done;
  Printf.printf "Count after 5 increments: %d\n" (get_count ());
  reset ();
  Printf.printf "Count after reset: %d\n" (get_count ())
