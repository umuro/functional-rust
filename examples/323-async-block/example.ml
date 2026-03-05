(* OCaml: lazy evaluation with thunks *)

let lazy_comp label f =
  Printf.printf "Creating: %s\n" label;
  fun () -> Printf.printf "Executing: %s\n" label; f ()

let run_if cond thunk = if cond then Some (thunk ()) else None

let () =
  let t1 = lazy_comp "double(5)" (fun () -> 5*2) in
  let t2 = lazy_comp "square(4)" (fun () -> 4*4) in
  Printf.printf "Result1: %d\n" (t1 ());
  Printf.printf "Result2: %d\n" (t2 ());
  let r = run_if false (lazy_comp "expensive" (fun () -> 9999)) in
  Printf.printf "Cond: %s\n" (match r with None -> "skipped" | Some v -> string_of_int v)
