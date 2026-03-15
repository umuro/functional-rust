(* OCaml: no raw pointers — all memory is managed by the GC.
   We model the concept of "address-indexed" access with arrays,
   which is the closest safe analogue. *)

(** Read value at given index, returning None for out-of-bounds. *)
let read_at (arr : 'a array) (index : int) : 'a option =
  if index >= 0 && index < Array.length arr then Some arr.(index)
  else None

(** Write to a mutable slot, returning false if out-of-bounds. *)
let write_at (arr : 'a array) (index : int) (value : 'a) : bool =
  if index >= 0 && index < Array.length arr then begin
    arr.(index) <- value;
    true
  end else false

let () =
  let data = [| 10; 20; 30; 40; 50 |] in
  (match read_at data 2 with
   | Some v -> Printf.printf "Read at index 2: %d\n" v
   | None   -> print_endline "Out of bounds");
  let ok = write_at data 2 99 in
  Printf.printf "Write succeeded: %b\n" ok;
  Printf.printf "After write: %d\n" data.(2)
