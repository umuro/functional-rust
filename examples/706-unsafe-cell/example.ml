(* OCaml: mutable cells are first-class via `ref`.
   Interior mutability is transparent — the type system does not
   distinguish shared vs exclusive references. *)

let cell : int ref = ref 0

let set   (c : int ref) (v : int) : unit = c := v
let get   (c : int ref) : int           = !c
let upd   (c : int ref) (f : int -> int) : unit = c := f !c

let () =
  upd cell (fun v -> v + 5);
  upd cell (fun v -> v + 3);
  Printf.printf "Cell value: %d\n" (get cell);
  set cell 100;
  Printf.printf "After set:  %d\n" (get cell)
