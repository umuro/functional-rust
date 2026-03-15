(* OCaml: exporting to C via Callback.register or output-obj compilation. *)

(* In real OCaml-C interop, you'd compile with:
   ocamlfind ocamlopt -package ... -output-obj -o rust_funcs.o funcs.ml
   Then link into a C program.
   
   The Callback.register mechanism lets C call back into OCaml: *)

let () =
  Callback.register "rust_add" (fun a b -> (a : int) + b);
  Callback.register "rust_fib" (fun n ->
    let rec fib k = if k <= 1 then k else fib (k-1) + fib (k-2) in
    fib (n : int)
  );
  (* C would call: caml_callback2(*caml_named_value("rust_add"), Val_int(3), Val_int(4)) *)
  print_endline "Callbacks registered for C interop"
