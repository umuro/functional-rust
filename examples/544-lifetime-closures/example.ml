(* Closures capturing references in OCaml — GC handles lifetimes *)
let make_adder_of data =
  (* data captured by GC reference *)
  let sum = List.fold_left (+) 0 data in
  fun x -> x + sum

let make_prefix_fn prefix =
  fun s -> prefix ^ ": " ^ s

let () =
  let data = [1;2;3;4;5] in
  let add_15 = make_adder_of data in
  Printf.printf "add_15(10) = %d\n" (add_15 10);

  let log = make_prefix_fn "INFO" in
  Printf.printf "%s\n" (log "something happened")
