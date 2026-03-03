(* Type-safe printf: the format string determines the function's type.
   OCaml's Printf.printf is already type-safe — this shows how it works. *)

(* Using OCaml's built-in type-safe format strings *)
let () =
  (* The format string determines the argument types at compile time *)
  Printf.printf "Hello, %s! You are %d years old.\n" "Alice" 30;
  Printf.printf "Pi is approximately %.4f\n" 3.14159;

  (* Format can be stored and reused *)
  let fmt = format_of_string "%s: %d\n" in
  Printf.printf fmt "score" 42;
  Printf.printf fmt "count" 100;

  (* sprintf for string building *)
  let s = Printf.sprintf "(%d, %d)" 3 4 in
  Printf.printf "Point: %s\n" s;

  (* ksprintf for custom continuations *)
  let log msg = Printf.eprintf "[LOG] %s\n" msg in
  Printf.ksprintf log "value = %d, name = %s" 99 "test"
