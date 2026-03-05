(* Transmute concept in OCaml using Obj.magic -- very unsafe! *)
(* Showing the concept, not recommended practice *)

let () =
  (* Type coercion via Obj.magic (like transmute) *)
  let x : int = 42 in
  let y : float = Obj.magic x in  (* UNSAFE: reinterpret bits *)
  Printf.printf "transmuted: %.2f\n" y;  (* garbage value *)

  (* Safe alternative: proper conversion *)
  let safe = float_of_int x in
  Printf.printf "safe conversion: %.2f\n" safe;

  (* Extending lifetime is impossible in OCaml -- GC prevents dangling *)
  Printf.printf "OCaml: GC makes lifetime extension unnecessary\n"
