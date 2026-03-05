(* stringify! and concat! concepts in OCaml *)

(* OCaml has __MODULE__, __LOC__, __FILE__, __LINE__ built-ins *)

let () =
  Printf.printf "Module: %s\n" __MODULE__;
  Printf.printf "Location: %s\n" __LOC__;
  Printf.printf "File: %s\n" __FILE__;

  (* Compile-time string concatenation via string literals *)
  let greeting = "Hello" ^ ", " ^ "World" ^ "!" in
  Printf.printf "%s\n" greeting;

  (* String of identifier — limited in OCaml *)
  let field_name = "user_id" in  (* manual *)
  Printf.printf "Field: %s\n" field_name
