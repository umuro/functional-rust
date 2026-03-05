(* OCaml: no Pin concept needed — GC handles moves *)

(* In OCaml, all values are effectively heap-allocated and the GC
   handles any internal references. There's no concept of "pinning"
   because values don't have a fixed memory address that user code
   depends on. *)

type self_ref = {
  data: string;
  mutable ptr: int option;  (* Would be an index, not a raw pointer *)
}

let make_self_ref s =
  let r = { data = s; ptr = None } in
  r.ptr <- Some 0;  (* Just tracking, not a real pointer *)
  r

let () =
  let sr = make_self_ref "hello" in
  Printf.printf "Data: %s\n" sr.data;
  Printf.printf "Ptr: %s\n" (match sr.ptr with Some i -> string_of_int i | None -> "none")
