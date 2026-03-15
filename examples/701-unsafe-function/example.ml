(* OCaml: no unsafe functions — safety is guaranteed by the type system.
   We model "low-level" access with validation wrappers. *)

(** Array.unsafe_get skips bounds check — the OCaml unsafe equivalent. *)
let unchecked_get (arr : 'a array) (i : int) : 'a = Array.unsafe_get arr i

(** Safe wrapper validates before delegating to unchecked_get. *)
let safe_get (arr : 'a array) (i : int) : 'a option =
  if i >= 0 && i < Array.length arr then Some (unchecked_get arr i)
  else None

let () =
  let data = [| 100; 200; 300 |] in
  (match safe_get data 1 with
   | Some v -> Printf.printf "safe_get(1) = %d\n" v
   | None   -> print_endline "out of bounds");
  (match safe_get data 5 with
   | Some _ -> print_endline "Should not happen"
   | None   -> print_endline "safe_get(5) = out of bounds (caught safely)")
