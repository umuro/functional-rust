(* OCaml — GC refs are explicit cells, not borrowing annotations *)
let () =
  let x = ref 42 in
  x := 100;
  Printf.printf "x=%d\n" !x;

  let opts = [Some 1; None; Some 42] in
  List.iter (function
    | Some v -> Printf.printf "Some %d\n" v
    | None   -> Printf.printf "None\n"
  ) opts
