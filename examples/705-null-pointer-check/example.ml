(* OCaml: no nulls — every value is non-null by definition.
   We model nullable vs non-nullable using option types. *)

(** NonNull is the default in OCaml — every int is non-null. *)
let make_nonnull (x : 'a) : 'a = x   (* identity: all values are non-null *)

(** Nullable via option. *)
let to_option (x : 'a option) : 'a option = x

let () =
  let nn = make_nonnull 42 in
  Printf.printf "NonNull value: %d\n" nn;
  let some_v = to_option (Some 99) in
  let none_v = to_option None in
  Printf.printf "Some: %s\n" (match some_v with Some v -> string_of_int v | None -> "null");
  Printf.printf "None: %s\n" (match none_v with Some v -> string_of_int v | None -> "null");
  (* Option size hint: OCaml options are always pointer-size *)
  Printf.printf "Simulating null-ptr optimisation: present in OCaml by default\n"
